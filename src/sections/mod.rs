pub mod claude;
pub mod directories;
pub mod dotfiles;
pub mod git_config;
pub mod packages;
pub mod repositories;
pub mod ssh;

use anyhow::Result;
use std::path::PathBuf;

use crate::config::Config;
use crate::exec::CommandRunner;

/// Shared context passed to every section.
pub struct Context<'a> {
    pub config: &'a Config,
    pub home_dir: PathBuf,
    pub dry_run: bool,
    pub force: bool,
    pub runner: &'a dyn CommandRunner,
}

/// Outcome of running a single section.
pub struct SectionReport {
    pub section: &'static str,
    pub succeeded: Vec<String>,
    pub skipped: Vec<String>,
    pub conflicts: Vec<String>,
    pub failed: Vec<(String, String)>,
}

impl SectionReport {
    pub fn new(section: &'static str) -> Self {
        Self {
            section,
            succeeded: Vec::new(),
            skipped: Vec::new(),
            conflicts: Vec::new(),
            failed: Vec::new(),
        }
    }
}

/// Every setup section implements this trait.
pub trait Section {
    fn name(&self) -> &'static str;
    fn apply(&self, ctx: &Context) -> Result<SectionReport>;
}

/// Build the ordered list of all sections.
fn all_sections() -> Vec<Box<dyn Section>> {
    // Execution order: directories → ssh → git → repositories → dotfiles → packages → claude
    vec![
        Box::new(directories::Directories),
        Box::new(ssh::Ssh),
        Box::new(git_config::GitSetup),
        Box::new(repositories::Repositories),
        Box::new(dotfiles::Dotfiles),
        Box::new(packages::Packages),
        Box::new(claude::Claude),
    ]
}

/// Run all sections (or a filtered one) in dependency order.
pub fn run_sections(ctx: &Context, filter: Option<&str>) -> Vec<SectionReport> {
    let mut reports = Vec::new();

    let sections = all_sections();

    for section in &sections {
        if let Some(name) = filter {
            if section.name() != name {
                continue;
            }
        }

        match section.apply(ctx) {
            Ok(report) => reports.push(report),
            Err(e) => {
                crate::output::error(&format!("{}: {}", section.name(), e));
            }
        }
    }

    reports
}
