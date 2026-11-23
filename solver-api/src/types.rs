use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QualityPreset {
    /// Fastest. Single bet size per street, 200 iter cap, 2% expl target.
    Fast,
    /// Default. Two bet sizes flop, single turn, 500 iter cap, 1% expl target.
    Balanced,
    /// Most precise. Three bet sizes flop, two on river, 1500 iter cap, 0.4% target.
    Deep,
}

impl Default for QualityPreset {
    fn default() -> Self {
        QualityPreset::Balanced
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolveRequest {
    pub board: String,
    pub oop_range: String,
    pub ip_range: String,
    pub starting_pot: i32,
    pub effective_stack: i32,
    /// Quality preset. When set, max_iterations / target_exploitability_pct /
    /// bet_sizes are derived from the preset (overriding any explicit values).
    /// Omit the field for the legacy explicit-knobs behaviour.
    #[serde(default)]
    pub quality: Option<QualityPreset>,
    #[serde(default)]
    pub bet_sizes: Option<BetSizes>,
    #[serde(default)]
    pub raise_sizes: Option<Vec<String>>,
    #[serde(default = "default_max_iter")]
    pub max_iterations: u32,
    #[serde(default = "default_target_exploitability_pct")]
    pub target_exploitability_pct: f32,
    /// When best bet >= this multiple of pot, add an extra all-in size.
    /// Defaults to 1.5. Set very high (e.g. 999.0) to disable.
    #[serde(default)]
    pub add_allin_threshold: Option<f64>,
    /// When SPR <= this, only all-in is considered. Defaults to 0.15. Set
    /// high (e.g. 10.0) to collapse any bet to all-in and shrink the tree.
    #[serde(default)]
    pub force_allin_threshold: Option<f64>,
    /// When set false, skip appending all-in as an extra bet size.
    #[serde(default = "default_true")]
    pub include_allin_bet: bool,
}

impl SolveRequest {
    /// Apply the quality preset (when set) by overriding iter cap, target, and
    /// bet-size defaults. Called once at the start of `run_solve`.
    pub fn applied_iterations(&self) -> u32 {
        match self.quality {
            Some(QualityPreset::Fast) => 200,
            Some(QualityPreset::Balanced) => 500,
            Some(QualityPreset::Deep) => 1500,
            None => self.max_iterations,
        }
    }
    pub fn applied_target_pct(&self) -> f32 {
        match self.quality {
            Some(QualityPreset::Fast) => 2.0,
            Some(QualityPreset::Balanced) => 1.0,
            Some(QualityPreset::Deep) => 0.4,
            None => self.target_exploitability_pct,
        }
    }
    /// Returns a preset-derived BetSizes when `quality` is set; otherwise None
    /// so the caller falls through to `bet_sizes` or hard defaults.
    pub fn preset_bet_sizes(&self) -> Option<BetSizes> {
        match self.quality {
            Some(QualityPreset::Fast) => Some(BetSizes {
                flop: Some(vec![0.50]),
                turn: Some(vec![0.75]),
                river: Some(vec![1.00]),
            }),
            Some(QualityPreset::Balanced) => Some(BetSizes {
                flop: Some(vec![0.33, 0.75]),
                turn: Some(vec![0.75]),
                river: Some(vec![0.75, 1.50]),
            }),
            Some(QualityPreset::Deep) => Some(BetSizes {
                flop: Some(vec![0.33, 0.75, 1.50]),
                turn: Some(vec![0.50, 1.00]),
                river: Some(vec![0.50, 1.00, 1.50]),
            }),
            None => None,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_max_iter() -> u32 {
    1000
}

fn default_target_exploitability_pct() -> f32 {
    0.5
}

#[derive(Debug, Clone, Deserialize)]
pub struct BetSizes {
    #[serde(default)]
    pub flop: Option<Vec<f32>>,
    #[serde(default)]
    pub turn: Option<Vec<f32>>,
    #[serde(default)]
    pub river: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobSubmission {
    pub job_id: Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobState {
    pub status: JobStatus,
    pub progress: f32,
    pub iterations: u32,
    pub exploitability: Option<f32>,
    pub result: Option<SolveResult>,
    pub error: Option<String>,
}

impl JobState {
    pub fn new() -> Self {
        Self {
            status: JobStatus::Queued,
            progress: 0.0,
            iterations: 0,
            exploitability: None,
            result: None,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SolveResult {
    pub board: String,
    pub starting_pot: i32,
    pub effective_stack: i32,
    pub oop_equity: f32,
    pub ip_equity: f32,
    pub actions: Vec<ActionStrategy>,
    pub total_iterations: u32,
    pub final_exploitability: f32,
    pub solve_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActionStrategy {
    pub action: String,
    pub amount: Option<i32>,
    pub frequency: f32,
    pub ev: f32,
}
