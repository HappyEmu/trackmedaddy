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

// POST /timers response
#[derive(Debug, Deserialize)]
pub struct StartTimerResponse {
    pub status: Option<String>,
}

// DELETE /timers/current response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct StopTimerResponse {
    pub status: Option<String>,
    pub task_time: Option<TaskTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskTime {
    pub time: Option<i64>,
    pub last_history: Option<HistoryEntry>,
    pub task: Option<TaskInfo>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryEntry {
    pub time: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TaskInfo {
    pub name: String,
}
