use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct StartTimerRequest {
    pub task: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Timer {
    pub status: Option<String>,
    pub duration: Option<i64>,
    pub started_at: Option<String>,
    pub task: Option<TimerTask>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TimerTask {
    pub id: String,
    pub name: String,
}
