mod api;
mod config;
mod models;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

use api::EverhourClient;
use config::{load_config, save_config};
use models::Config;

#[derive(Parser)]
#[command(name = "trackmedaddy", about = "CLI for Everhour time tracking")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Save your Everhour API key
    Login,
    /// Remove your saved API key
    Logout,
    /// Start a timer on a task matching the given ticket (e.g. TRG-80)
    Start {
        /// Linear ticket identifier (e.g. TRG-80)
        ticket: String,
    },
    /// Show the currently running timer
    Status,
    /// Stop the currently running timer
    Stop,
    /// Install the trackmedaddy skill/command for your AI coding agent
    Skill {
        /// Agent to install for: claude or codex
        agent: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Login => cmd_login()?,
        Command::Logout => cmd_logout()?,
        Command::Start { ticket } => cmd_start(&ticket).await?,
        Command::Status => cmd_status().await?,
        Command::Stop => cmd_stop().await?,
        Command::Skill { agent } => cmd_skill(&agent)?,
    }
    Ok(())
}

fn cmd_login() -> Result<()> {
    eprint!("Enter your Everhour API key: ");
    let mut key = String::new();
    std::io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();
    if key.is_empty() {
        bail!("API key cannot be empty");
    }
    save_config(&Config { api_key: key })?;
    let path = config::config_path()?;
    println!("Config saved to {}", path.display());
    Ok(())
}

fn cmd_logout() -> Result<()> {
    let path = config::config_path()?;
    let legacy_path = config::legacy_config_path()?;

    let removed_current = remove_file_if_exists(&path)?;
    let removed_legacy = remove_file_if_exists(&legacy_path)?;

    if removed_current || removed_legacy {
        if removed_current {
            println!("Config removed: {}", path.display());
        }
        if removed_legacy {
            println!("Legacy config removed: {}", legacy_path.display());
        }
    } else {
        println!("Already logged out (no config file found).");
    }
    Ok(())
}

/// Remove a file if it exists. Returns true if a file was removed.
fn remove_file_if_exists(path: &std::path::Path) -> Result<bool> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

const SKILL_CONTENT: &str = include_str!("../skill.md");

fn cmd_skill(agent: &str) -> Result<()> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let (path, legacy_paths) = match agent.to_lowercase().as_str() {
        "claude" => {
            let claude = home.join(".claude");
            (
                claude.join("skills").join("trackmedaddy").join("SKILL.md"),
                vec![
                    claude.join("commands").join("everhour.md"),
                    claude.join("commands").join("trackmedaddy.md"),
                ],
            )
        }
        "codex" => {
            let agents = home.join(".agents");
            let codex = home.join(".codex");
            (
                agents.join("skills").join("trackmedaddy").join("SKILL.md"),
                vec![
                    codex.join("instructions").join("trackmedaddy.md"),
                    codex.join("instructions").join("everhour.md"),
                ],
            )
        }
        _ => bail!("Unknown agent \"{agent}\". Supported: claude, codex"),
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, SKILL_CONTENT)?;
    println!("Skill installed to {}", path.display());
    for legacy in &legacy_paths {
        if remove_file_if_exists(legacy)? {
            println!("Removed old skill file: {}", legacy.display());
        }
    }
    Ok(())
}

async fn cmd_start(ticket: &str) -> Result<()> {
    let config = load_config()?;
    let client = EverhourClient::new(config.api_key);

    let tasks = client.search_tasks(ticket).await?;
    if tasks.is_empty() {
        bail!("No tasks found matching \"{ticket}\"");
    }

    let task = &tasks[0];
    println!("Found task: {} ({})", task.name, task.id);

    let timer = client.start_timer(&task.id).await?;
    println!(
        "Timer started (status: {})",
        timer.status.as_deref().unwrap_or("unknown")
    );
    Ok(())
}

async fn cmd_status() -> Result<()> {
    let config = load_config()?;
    let client = EverhourClient::new(config.api_key);

    let resp = client.current_timer().await?;
    match resp.status.as_deref() {
        Some("active") => {
            let task_name = resp
                .task
                .as_ref()
                .map(|t| t.name.as_str())
                .unwrap_or("unknown");
            let elapsed = format_duration(resp.duration.unwrap_or(0));
            let today = format_duration(resp.today.unwrap_or(0));
            println!("Running: {} (elapsed: {}, today: {})", task_name, elapsed, today);
        }
        _ => {
            println!("No timer running.");
        }
    }
    Ok(())
}

fn format_duration(seconds: i64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

async fn cmd_stop() -> Result<()> {
    let config = load_config()?;
    let client = EverhourClient::new(config.api_key);

    let resp = client.stop_timer().await?;
    if let Some(task_time) = &resp.task_time {
        let task_name = task_time
            .task
            .as_ref()
            .map(|t| t.name.as_str())
            .unwrap_or("unknown");
        let session = task_time
            .last_history
            .as_ref()
            .and_then(|h| h.time)
            .unwrap_or(0);
        let total_today = task_time.time.unwrap_or(0);
        println!(
            "Timer stopped: {} (session: {}, today: {})",
            task_name,
            format_duration(session),
            format_duration(total_today),
        );
    } else {
        println!("Timer stopped.");
    }
    Ok(())
}
