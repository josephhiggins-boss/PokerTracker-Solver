use std::sync::Arc;
use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use dashmap::DashMap;
use postflop_solver::*;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

use crate::types::{
    ActionStrategy, BetSizes, JobState, JobStatus, SolveRequest, SolveResult,
};

pub type JobStore = Arc<DashMap<Uuid, Arc<RwLock<JobState>>>>;

pub fn spawn_solve(store: JobStore, job_id: Uuid, req: SolveRequest) {
    let state = Arc::new(RwLock::new(JobState::new()));
    store.insert(job_id, state.clone());

    // Solver work is CPU-bound and synchronous; hand it to a dedicated
    // blocking thread so it doesn't starve the tokio runtime.
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        let result = run_solve(&state, &rt, &req);
        match result {
            Ok(solve_result) => {
                rt.block_on(async {
                    let mut s = state.write().await;
                    s.status = JobStatus::Done;
                    s.progress = 1.0;
                    s.result = Some(solve_result);
                });
            }
            Err(e) => {
                error!(job = %job_id, err = %e, "solve failed");
                rt.block_on(async {
                    let mut s = state.write().await;
                    s.status = JobStatus::Error;
                    s.error = Some(format!("{e:#}"));
                });
            }
        }
    });
}

fn run_solve(
    state: &Arc<RwLock<JobState>>,
    rt: &tokio::runtime::Handle,
    req: &SolveRequest,
) -> Result<SolveResult> {
    rt.block_on(async {
        let mut s = state.write().await;
        s.status = JobStatus::Running;
    });

    let started = Instant::now();

    let oop: Range = req
        .oop_range
        .parse()
        .map_err(|e: String| anyhow!("failed to parse oop_range: {e}"))?;
    let ip: Range = req
        .ip_range
        .parse()
        .map_err(|e: String| anyhow!("failed to parse ip_range: {e}"))?;

    let (flop, turn, river, initial_state) = parse_board(&req.board)?;

    let card_config = CardConfig {
        range: [oop, ip],
        flop,
        turn,
        river,
    };

    let tree_config = build_tree_config(req, initial_state)?;
    let action_tree = ActionTree::new(tree_config)
        .map_err(|e| anyhow!("failed to build action tree: {e}"))?;
    let mut game = PostFlopGame::with_config(card_config, action_tree)
        .map_err(|e| anyhow!("failed to init PostFlopGame: {e}"))?;

    // `false` = uncompressed f32 memory (fastest solve, higher RAM).
    game.allocate_memory(false);

    let starting_pot = game.tree_config().starting_pot as f32;
    let target = starting_pot * (req.applied_target_pct() / 100.0);

    // Iterative solve loop so we can stream progress to the job store.
    let mut expl = compute_exploitability(&game);
    let max_iter = req.applied_iterations().max(1);
    let mut last_iter = 0u32;
    for t in 0..max_iter {
        if expl <= target {
            break;
        }
        solve_step(&game, t as u32);
        last_iter = t + 1;
        if last_iter % 10 == 0 || last_iter == max_iter {
            expl = compute_exploitability(&game);
            let progress = (last_iter as f32 / max_iter as f32).min(1.0);
            rt.block_on(async {
                let mut s = state.write().await;
                s.iterations = last_iter;
                s.progress = progress;
                s.exploitability = Some(expl);
            });
            if expl <= target {
                break;
            }
        }
    }

    finalize(&mut game);
    game.cache_normalized_weights();

    let actions = game.available_actions();
    let strat = game.strategy();
    // expected_values_detail returns per-action-per-hand EV when the target
    // player is to act at the current node (true at the root for the first
    // to act). Falls back to per-hand if not — we guard below.
    let acting_player = game.current_player();
    let ev_detail = game.expected_values_detail(acting_player);
    let weights_acting = game.normalized_weights(acting_player).to_vec();
    let weights_oop = game.normalized_weights(0).to_vec();
    let weights_ip = game.normalized_weights(1).to_vec();
    let equity_oop = game.equity(0);
    let equity_ip = game.equity(1);

    let n = game.private_cards(acting_player).len();
    if actions.is_empty() || n == 0 {
        return Err(anyhow!("solver returned empty action/combo set"));
    }
    if strat.len() != actions.len() * n {
        return Err(anyhow!(
            "strategy length mismatch: {} vs {}*{}",
            strat.len(),
            actions.len(),
            n
        ));
    }
    let has_per_action_ev = ev_detail.len() == actions.len() * n;

    let mut out_actions = Vec::with_capacity(actions.len());
    for (a_idx, action) in actions.iter().enumerate() {
        let strat_slice = &strat[a_idx * n..(a_idx + 1) * n];
        let freq = compute_average(strat_slice, &weights_acting);
        let ev = if has_per_action_ev {
            let ev_slice = &ev_detail[a_idx * n..(a_idx + 1) * n];
            compute_average(ev_slice, &weights_acting)
        } else {
            f32::NAN
        };
        let (name, amount) = action_label(action);
        out_actions.push(ActionStrategy {
            action: name,
            amount,
            frequency: freq,
            ev,
        });
    }

    let avg_equity_oop = compute_average(&equity_oop, &weights_oop);
    let avg_equity_ip = compute_average(&equity_ip, &weights_ip);

    Ok(SolveResult {
        board: req.board.clone(),
        starting_pot: req.starting_pot,
        effective_stack: req.effective_stack,
        oop_equity: avg_equity_oop,
        ip_equity: avg_equity_ip,
        actions: out_actions,
        total_iterations: last_iter,
        final_exploitability: expl,
        solve_ms: started.elapsed().as_millis() as u64,
    })
}

fn parse_board(board: &str) -> Result<([u8; 3], u8, u8, BoardState)> {
    // Accept "AsKdQh" (flop), 8-char "AsKdQhJs" (turn), 10-char (river).
    let clean: String = board.chars().filter(|c| !c.is_whitespace()).collect();
    match clean.len() {
        6 => {
            let flop = flop_from_str(&clean)
                .map_err(|e| anyhow!("bad flop: {e}"))?;
            Ok((flop, NOT_DEALT, NOT_DEALT, BoardState::Flop))
        }
        8 => {
            let flop = flop_from_str(&clean[..6])
                .map_err(|e| anyhow!("bad flop: {e}"))?;
            let turn = card_from_str(&clean[6..8])
                .map_err(|e| anyhow!("bad turn: {e}"))?;
            Ok((flop, turn, NOT_DEALT, BoardState::Turn))
        }
        10 => {
            let flop = flop_from_str(&clean[..6])
                .map_err(|e| anyhow!("bad flop: {e}"))?;
            let turn = card_from_str(&clean[6..8])
                .map_err(|e| anyhow!("bad turn: {e}"))?;
            let river = card_from_str(&clean[8..10])
                .map_err(|e| anyhow!("bad river: {e}"))?;
            Ok((flop, turn, river, BoardState::River))
        }
        other => Err(anyhow!(
            "board must be 6, 8, or 10 chars (e.g. AsKdQh). got len {other}"
        )),
    }
}

fn build_tree_config(req: &SolveRequest, initial_state: BoardState) -> Result<TreeConfig> {
    // Quality preset takes precedence over explicit bet_sizes / hard defaults.
    let preset = req.preset_bet_sizes();
    let defaults = BetSizes {
        flop: Some(vec![0.33, 0.75, 1.50]),
        turn: Some(vec![0.75]),
        river: Some(vec![0.75, 1.50]),
    };
    let b = preset.as_ref().or(req.bet_sizes.as_ref()).unwrap_or(&defaults);

    let flop_str = bet_size_string(
        b.flop.as_deref().unwrap_or(&[0.33, 0.75, 1.50]),
        req.include_allin_bet,
    );
    let turn_str = bet_size_string(
        b.turn.as_deref().unwrap_or(&[0.75]),
        req.include_allin_bet,
    );
    let river_str = bet_size_string(
        b.river.as_deref().unwrap_or(&[0.75, 1.50]),
        req.include_allin_bet,
    );

    let default_raises = vec!["2.5x".to_string()];
    let raises = req
        .raise_sizes
        .as_deref()
        .unwrap_or(&default_raises);
    let raise_str = raises.join(",");

    let flop_bets = BetSizeOptions::try_from((flop_str.as_str(), raise_str.as_str()))
        .map_err(|e| anyhow!("bad flop bet sizes: {e}"))
        .context("flop_bet_sizes")?;
    let turn_bets = BetSizeOptions::try_from((turn_str.as_str(), raise_str.as_str()))
        .map_err(|e| anyhow!("bad turn bet sizes: {e}"))
        .context("turn_bet_sizes")?;
    let river_bets = BetSizeOptions::try_from((river_str.as_str(), raise_str.as_str()))
        .map_err(|e| anyhow!("bad river bet sizes: {e}"))
        .context("river_bet_sizes")?;

    Ok(TreeConfig {
        initial_state,
        starting_pot: req.starting_pot,
        effective_stack: req.effective_stack,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [flop_bets.clone(), flop_bets],
        turn_bet_sizes: [turn_bets.clone(), turn_bets],
        river_bet_sizes: [river_bets.clone(), river_bets],
        turn_donk_sizes: None,
        river_donk_sizes: None,
        add_allin_threshold: req.add_allin_threshold.unwrap_or(1.5),
        force_allin_threshold: req.force_allin_threshold.unwrap_or(0.15),
        merging_threshold: 0.1,
    })
}

fn bet_size_string(fractions: &[f32], include_allin: bool) -> String {
    let mut parts: Vec<String> = fractions
        .iter()
        .map(|f| format!("{}%", (f * 100.0).round() as i32))
        .collect();
    if include_allin {
        parts.push("a".to_string());
    }
    parts.join(",")
}

fn action_label(action: &Action) -> (String, Option<i32>) {
    match *action {
        Action::None => ("none".into(), None),
        Action::Fold => ("fold".into(), None),
        Action::Check => ("check".into(), None),
        Action::Call => ("call".into(), None),
        Action::Bet(amount) => ("bet".into(), Some(amount)),
        Action::Raise(amount) => ("raise".into(), Some(amount)),
        Action::AllIn(amount) => ("allin".into(), Some(amount)),
        Action::Chance(card) => ("chance".into(), Some(card as i32)),
    }
}

#[allow(dead_code)]
pub fn warn_large_allocation(bytes: u64) {
    if bytes > 4 * 1024 * 1024 * 1024 {
        info!("solver allocation: {:.1} GB", bytes as f64 / 1e9);
    }
}
