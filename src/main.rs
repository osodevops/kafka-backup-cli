mod app;
mod config;
mod dashboard;
mod data;
mod effects;
mod recording;
mod theme;
mod widgets;

use clap::{Parser, Subcommand};
use color_eyre::Result;

#[derive(Parser)]
#[command(name = "kafka-backup-monitor")]
#[command(about = "Real-time TUI dashboard for kafka-backup monitoring")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a scripted demo scenario (for screen recording)
    Demo {
        /// Path to YAML scenario file
        #[arg(short, long)]
        scenario: Option<String>,

        /// Auto-start without waiting for keypress
        #[arg(long, default_value_t = false)]
        auto_start: bool,
    },

    /// Monitor a live kafka-backup process
    Live {
        /// Path to config YAML
        #[arg(short, long)]
        config: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Demo { scenario, auto_start } => run_demo(scenario, auto_start),
        Commands::Live { config: _ } => run_live(),
    }
}

fn run_demo(scenario_path: Option<String>, auto_start: bool) -> Result<()> {
    let mut app = match scenario_path {
        Some(path) => {
            let scenario = config::load_scenario(&path)?;
            app::App::from_scenario(scenario)
        }
        None => app::App::with_sample_data(true),
    };
    app.paused = !auto_start;

    let terminal = ratatui::init();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

fn run_live() -> Result<()> {
    let terminal = ratatui::init();
    let app = app::App::new(false);
    let result = app.run(terminal);
    ratatui::restore();
    result
}
