use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};
use solver_api::{build_router, new_app_state};
use tokio::net::TcpListener;

// Tiny spot — keep ranges small and bet sizes minimal so the test runs in
// tens of seconds rather than minutes. This test proves the pipeline works
// end to end; a realistic solve is something you do via curl.
const BOARD: &str = "Ah7d2c";
const OOP_RANGE: &str = "TT+,AKs";
const IP_RANGE: &str = "QQ+,AKs";

#[tokio::test]
async fn solve_round_trip() {
    let state = new_app_state();
    let app = build_router(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::new();
    let base = format!("http://{addr}");

    // Sanity: /health responds.
    let health: Value = client
        .get(format!("{base}/health"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(health["status"], "ok");

    // Submit solve. Single flop size, no turn/river tree complexity, short
    // iteration cap — we're testing the pipeline, not depth of solve.
    // Minimal tree: SPR=1 (pot 100, stack 100) with force_allin_threshold=10
    // collapses any bet into all-in at every street. No raises. Keeps the
    // solve under a minute on a laptop.
    let submit: Value = client
        .post(format!("{base}/solve"))
        .json(&json!({
            "board": BOARD,
            "oop_range": OOP_RANGE,
            "ip_range": IP_RANGE,
            "starting_pot": 100,
            "effective_stack": 100,
            "bet_sizes": {
                "flop": [0.75],
                "turn": [],
                "river": []
            },
            "raise_sizes": [],
            "include_allin_bet": false,
            "force_allin_threshold": 10.0,
            "add_allin_threshold": 999.0,
            "max_iterations": 50,
            "target_exploitability_pct": 10.0
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let job_id = submit["job_id"].as_str().unwrap().to_string();
    assert!(!job_id.is_empty(), "job_id missing");

    // Poll up to 5 minutes for the solver to finish.
    let deadline = std::time::Instant::now() + Duration::from_secs(5 * 60);
    let mut last_state;
    loop {
        let v: Value = client
            .get(format!("{base}/solve/{job_id}"))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        last_state = v.clone();
        match v["status"].as_str().unwrap_or("") {
            "done" => break,
            "error" => panic!("solve errored: {v:#?}"),
            _ => {
                if std::time::Instant::now() >= deadline {
                    panic!("solve timed out: {v:#?}");
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    let result = &last_state["result"];
    assert!(!result.is_null(), "result missing on done job: {last_state:#?}");

    let actions = result["actions"].as_array().expect("actions array");
    assert!(!actions.is_empty(), "no actions in strategy");

    let sum: f32 = actions
        .iter()
        .map(|a| a["frequency"].as_f64().unwrap_or(0.0) as f32)
        .sum();
    assert!(
        (sum - 1.0).abs() < 0.05,
        "action frequencies should sum ~1.0, got {sum} (actions={actions:#?})"
    );

    // Board and pot should round-trip.
    assert_eq!(result["board"].as_str().unwrap(), BOARD);
    assert_eq!(result["starting_pot"].as_i64().unwrap(), 100);

    // Some iterations should have run.
    let iters = result["total_iterations"].as_u64().unwrap_or(0);
    assert!(iters > 0, "solver ran 0 iterations");

    eprintln!(
        "solved in {}ms over {} iterations, exploitability {:.3}",
        result["solve_ms"].as_u64().unwrap_or(0),
        iters,
        result["final_exploitability"].as_f64().unwrap_or(0.0)
    );
    eprintln!("actions: {actions:#?}");
}
