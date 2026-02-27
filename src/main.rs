mod api;
mod config;
mod models;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};

use api::EverhourClient;
use config::{load_config, save_config};
use models::Config;

#[derive(Parser)]
#[command(name = "everhour", about = "Everhour time tracking CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Save your Everhour API key
    Login,
    /// Start a timer on a task matching the given ticket (e.g. TRG-80)
    Start {
        /// Linear ticket identifier (e.g. TRG-80)
        ticket: String,
    },
    /// Stop the currently running timer
    Stop,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Login => cmd_login()?,
        Command::Start { ticket } => cmd_start(&ticket).await?,
        Command::Stop => cmd_stop().await?,
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

async fn cmd_stop() -> Result<()> {
    let config = load_config()?;
    let client = EverhourClient::new(config.api_key);

    let timer = client.stop_timer().await?;
    if let Some(duration) = timer.duration {
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;
        println!("Timer stopped. Duration: {hours:02}:{minutes:02}:{seconds:02}");
    } else {
        println!("Timer stopped.");
    }
    Ok(())
}
