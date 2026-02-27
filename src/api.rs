use anyhow::{Result, bail};
use reqwest::Client;

use crate::models::{StartTimerRequest, StartTimerResponse, StopTimerResponse, Task};

const BASE_URL: &str = "https://api.everhour.com";

pub struct EverhourClient {
    http: Client,
    api_key: String,
}

impl EverhourClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
        }
    }

    pub async fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        let url = format!("{BASE_URL}/tasks/search");
        let resp = self
            .http
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .query(&[("query", query), ("limit", "10")])
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("Search failed (HTTP {status}): {body}");
        }

        let tasks: Vec<Task> = resp.json().await?;
        Ok(tasks)
    }

    pub async fn start_timer(&self, task_id: &str) -> Result<StartTimerResponse> {
        let url = format!("{BASE_URL}/timers");
        let body = StartTimerRequest {
            task: task_id.to_string(),
        };
        let resp = self
            .http
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("Start timer failed (HTTP {status}): {body}");
        }

        let timer: StartTimerResponse = resp.json().await?;
        Ok(timer)
    }

    pub async fn stop_timer(&self) -> Result<StopTimerResponse> {
        let url = format!("{BASE_URL}/timers/current");
        let resp = self
            .http
            .delete(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("Stop timer failed (HTTP {status}): {body}");
        }

        let timer: StopTimerResponse = resp.json().await?;
        Ok(timer)
    }
}
