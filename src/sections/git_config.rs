use anyhow::Result;

use super::{Context, Section, SectionReport};
use crate::output;

pub struct GitSetup;

impl Section for GitSetup {
    fn name(&self) -> &'static str {
        "git"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("git");

        let git_config = match &ctx.config.git {
            Some(c) => c,
            None => return Ok(report),
        };

        let global = match &git_config.global {
            Some(g) => g,
            None => return Ok(report),
        };

        output::section_header("git");

        for (key, value) in global {
            // Check current value
            let current = ctx.runner.run_success("git", &["config", "--global", "--get", key]);

            match current {
                Ok(current_val) if current_val == *value => {
                    output::skipped(&format!("{} = {} (already set)", key, value));
                    report.skipped.push(format!("{} = {}", key, value));
                }
                _ => {
                    // Value is missing or different — set it
                    if ctx.dry_run {
                        output::success(&format!("{} = {} (would set)", key, value));
                        report.succeeded.push(format!("{} = {}", key, value));
                    } else {
                        match ctx
                            .runner
                            .run_success("git", &["config", "--global", key, value])
                        {
                            Ok(_) => {
                                output::success(&format!("{} = {} (set)", key, value));
                                report.succeeded.push(format!("{} = {}", key, value));
                            }
                            Err(e) => {
                                output::error(&format!("{}: {}", key, e));
                                report
                                    .failed
                                    .push((format!("{} = {}", key, value), e.to_string()));
                            }
                        }
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
    fn test_sets_missing_git_config() {
        let config: Config = toml::from_str(
            r#"
[git.global]
"user.name" = "Test User"
"user.email" = "test@test.com"
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        // git config --global --get user.email -> fail (not set)
        runner.push_failure(""); // get user.email
        runner.push_success(""); // set user.email
        // git config --global --get user.name -> fail (not set)
        runner.push_failure(""); // get user.name
        runner.push_success(""); // set user.name

        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = GitSetup.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 2);
        assert!(report.skipped.is_empty());
    }

    #[test]
    fn test_skips_already_set_values() {
        let config: Config = toml::from_str(
            r#"
[git.global]
"user.name" = "Test User"
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        // git config --global --get user.name -> "Test User" (already set correctly)
        runner.push_success("Test User");

        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = GitSetup.apply(&ctx).unwrap();
        assert!(report.succeeded.is_empty());
        assert_eq!(report.skipped.len(), 1);
    }
}
