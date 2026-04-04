use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use devtools::config;
use devtools::exec::RealRunner;
use devtools::output;
use devtools::sections;

#[derive(Parser)]
#[command(name = "devtools", about = "Bootstrap developer machines from a single config file")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Show what would change without making modifications
    #[arg(long, global = true)]
    dry_run: bool,

    /// Overwrite conflicts instead of warning
    #[arg(long, global = true)]
    force: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Full machine bootstrap (all sections or a specific one)
    Setup {
        /// Run only a specific section
        section: Option<SectionName>,
    },
    /// Verify current state against config, report drift
    Check {
        /// Check only a specific section
        section: Option<SectionName>,
    },
    /// Generate a starter config file
    Init,
}

#[derive(Clone, ValueEnum)]
enum SectionName {
    Dirs,
    Repos,
    Dotfiles,
    Git,
    Ssh,
    Packages,
    Claude,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init()?,
        Commands::Setup { section } => {
            let config_path = resolve_config_path(cli.config)?;
            let config = config::load_config(&config_path)?;
            cmd_setup(config, cli.dry_run, cli.force, section)?;
        }
        Commands::Check { section } => {
            let config_path = resolve_config_path(cli.config)?;
            let config = config::load_config(&config_path)?;
            // Check is just setup with dry_run forced on
            cmd_setup(config, true, cli.force, section)?;
        }
    }

    Ok(())
}

fn resolve_config_path(explicit: Option<PathBuf>) -> Result<PathBuf> {
    match explicit {
        Some(p) => Ok(p),
        None => config::default_config_path(),
    }
}

fn cmd_init() -> Result<()> {
    let config_path = config::default_config_path()?;

    if config_path.exists() {
        eprintln!(
            "Config already exists at {}",
            config_path.display()
        );
        eprintln!("Edit it directly or remove it to re-initialize.");
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    std::fs::write(&config_path, config::starter_config())
        .with_context(|| format!("writing {}", config_path.display()))?;

    output::success(&format!("Created {}", config_path.display()));
    eprintln!("Edit this file, then run: devtools setup");

    Ok(())
}

fn cmd_setup(
    config: config::Config,
    dry_run: bool,
    force: bool,
    section: Option<SectionName>,
) -> Result<()> {
    let home_dir = dirs::home_dir().context("could not determine home directory")?;
    let runner = RealRunner;

    let ctx = sections::Context {
        config: &config,
        home_dir,
        dry_run,
        force,
        runner: &runner,
    };

    if dry_run {
        output::info("dry run — no changes will be made");
    }

    let filter = section.map(|s| match s {
        SectionName::Dirs => "directories",
        SectionName::Repos => "repositories",
        SectionName::Dotfiles => "dotfiles",
        SectionName::Git => "git",
        SectionName::Ssh => "ssh",
        SectionName::Packages => "packages",
        SectionName::Claude => "claude",
    });

    let reports = sections::run_sections(&ctx, filter);

    // Final summary
    let total_ok: usize = reports.iter().map(|r| r.succeeded.len()).sum();
    let total_skip: usize = reports.iter().map(|r| r.skipped.len()).sum();
    let total_conflict: usize = reports.iter().map(|r| r.conflicts.len()).sum();
    let total_fail: usize = reports.iter().map(|r| r.failed.len()).sum();

    eprintln!();
    if total_fail == 0 && total_conflict == 0 {
        output::success(&format!(
            "Done. {} applied, {} unchanged.",
            total_ok, total_skip
        ));
    } else {
        output::info(&format!(
            "Done. {} applied, {} unchanged, {} conflicts, {} failed.",
            total_ok, total_skip, total_conflict, total_fail
        ));
    }

    Ok(())
}
