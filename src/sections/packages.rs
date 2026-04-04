use anyhow::Result;

use super::{Context, Section, SectionReport};
use crate::output;

pub struct Packages;

impl Section for Packages {
    fn name(&self) -> &'static str {
        "packages"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("packages");

        let pkg_config = match &ctx.config.packages {
            Some(p) => p,
            None => return Ok(report),
        };

        output::section_header("packages");

        // Check if brew is available
        let brew_available = ctx.runner.run("brew", &["--version"]).is_ok()
            && ctx
                .runner
                .run("brew", &["--version"])
                .map(|o| o.status.success())
                .unwrap_or(false);

        if !brew_available && (!pkg_config.formulae.is_empty() || !pkg_config.casks.is_empty()) {
            output::error("Homebrew is not installed. Install it from https://brew.sh");
            report
                .failed
                .push(("homebrew".to_string(), "not installed".to_string()));
            // Skip brew packages but continue with runtimes
        } else {
            // Get installed formulae and casks in batch for efficiency
            let installed_formulae = if !pkg_config.formulae.is_empty() {
                ctx.runner
                    .run_success("brew", &["list", "--formula", "-1"])
                    .unwrap_or_default()
            } else {
                String::new()
            };

            let installed_casks = if !pkg_config.casks.is_empty() {
                ctx.runner
                    .run_success("brew", &["list", "--cask", "-1"])
                    .unwrap_or_default()
            } else {
                String::new()
            };

            // Install formulae
            for formula in &pkg_config.formulae {
                if installed_formulae.lines().any(|l| l.trim() == formula.as_str()) {
                    output::skipped(&format!("{} (installed)", formula));
                    report.skipped.push(formula.clone());
                } else if ctx.dry_run {
                    output::success(&format!("{} (would install)", formula));
                    report.succeeded.push(formula.clone());
                } else {
                    match ctx.runner.run_success("brew", &["install", formula]) {
                        Ok(_) => {
                            output::success(&format!("{} (installed)", formula));
                            report.succeeded.push(formula.clone());
                        }
                        Err(e) => {
                            output::error(&format!("{}: {}", formula, e));
                            report.failed.push((formula.clone(), e.to_string()));
                        }
                    }
                }
            }

            // Install casks
            for cask in &pkg_config.casks {
                if installed_casks.lines().any(|l| l.trim() == cask.as_str()) {
                    output::skipped(&format!("{} (installed)", cask));
                    report.skipped.push(cask.clone());
                } else if ctx.dry_run {
                    output::success(&format!("{} (would install cask)", cask));
                    report.succeeded.push(cask.clone());
                } else {
                    match ctx
                        .runner
                        .run_success("brew", &["install", "--cask", cask])
                    {
                        Ok(_) => {
                            output::success(&format!("{} (installed cask)", cask));
                            report.succeeded.push(cask.clone());
                        }
                        Err(e) => {
                            output::error(&format!("{}: {}", cask, e));
                            report.failed.push((cask.clone(), e.to_string()));
                        }
                    }
                }
            }
        }

        // Install runtimes
        for runtime in &pkg_config.runtimes {
            // Check if runtime is already installed using the check command
            let check_parts: Vec<&str> = runtime.check.split_whitespace().collect();
            let is_installed = if let Some((cmd, args)) = check_parts.split_first() {
                ctx.runner.run(cmd, args).map(|o| o.status.success()).unwrap_or(false)
            } else {
                false
            };

            if is_installed {
                output::skipped(&format!("{} (installed)", runtime.name));
                report.skipped.push(runtime.name.clone());
            } else if ctx.dry_run {
                output::success(&format!("{} (would install)", runtime.name));
                report.succeeded.push(runtime.name.clone());
            } else {
                // Run install command through shell for pipe support
                match ctx
                    .runner
                    .run_success("sh", &["-c", &runtime.install_cmd])
                {
                    Ok(_) => {
                        output::success(&format!("{} (installed)", runtime.name));
                        report.succeeded.push(runtime.name.clone());
                    }
                    Err(e) => {
                        output::error(&format!("{}: {}", runtime.name, e));
                        report.failed.push((runtime.name.clone(), e.to_string()));
                    }
                }
            }
        }

        output::summary(&report);
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::exec::tests::MockRunner;

    #[test]
    fn test_installs_missing_formulae() {
        let config: Config = toml::from_str(
            r#"
[packages]
formulae = ["jq", "ripgrep"]
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success("Homebrew 4.0"); // brew --version (availability check)
        runner.push_success("Homebrew 4.0"); // brew --version (second call from the is_ok + status check)
        runner.push_success("git\nfzf\n"); // brew list --formula (neither jq nor ripgrep installed)
        runner.push_success(""); // brew install jq
        runner.push_success(""); // brew install ripgrep

        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Packages.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 2);
    }

    #[test]
    fn test_skips_installed_formulae() {
        let config: Config = toml::from_str(
            r#"
[packages]
formulae = ["jq"]
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success("Homebrew 4.0"); // brew --version
        runner.push_success("Homebrew 4.0"); // brew --version
        runner.push_success("jq\ngit\n"); // brew list --formula (jq is installed)

        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Packages.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
        assert!(report.succeeded.is_empty());
    }

    #[test]
    fn test_checks_runtime_installation() {
        let config: Config = toml::from_str(
            r#"
[packages]

[[packages.runtimes]]
name = "rust"
check = "rustup --version"
install_cmd = "curl https://sh.rustup.rs | sh"
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success("Homebrew 4.0"); // brew --version
        runner.push_success("Homebrew 4.0"); // brew --version
        // rustup --version -> succeeds (already installed)
        runner.push_success("rustup 1.27.0");

        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Packages.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
    }
}
