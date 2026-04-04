use anyhow::Result;

use super::{Context, Section, SectionReport};
use crate::config::expand_tilde;
use crate::output;

pub struct Repositories;

impl Section for Repositories {
    fn name(&self) -> &'static str {
        "repositories"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("repositories");

        let repos = match &ctx.config.repositories {
            Some(r) => r,
            None => return Ok(report),
        };

        output::section_header("repositories");

        for repo in repos {
            let path = expand_tilde(&repo.path)?;

            if path.exists() {
                // Verify it's a git repo with the correct remote
                let remote_result = ctx.runner.run_success(
                    "git",
                    &["-C", path.to_str().unwrap_or(""), "remote", "get-url", "origin"],
                );

                match remote_result {
                    Ok(current_remote) if current_remote.trim() == repo.remote => {
                        if repo.pull {
                            if ctx.dry_run {
                                output::success(&format!("{} (would pull)", repo.path));
                                report.succeeded.push(format!("{} (pull)", repo.path));
                            } else {
                                match ctx.runner.run_success(
                                    "git",
                                    &["-C", path.to_str().unwrap_or(""), "pull", "--ff-only"],
                                ) {
                                    Ok(_) => {
                                        output::success(&format!("{} (pulled)", repo.path));
                                        report.succeeded.push(format!("{} (pull)", repo.path));
                                    }
                                    Err(e) => {
                                        output::error(&format!("{} pull failed: {}", repo.path, e));
                                        report.failed.push((repo.path.clone(), e.to_string()));
                                    }
                                }
                            }
                        } else {
                            output::skipped(&format!("{} (exists)", repo.path));
                            report.skipped.push(repo.path.clone());
                        }
                    }
                    Ok(current_remote) => {
                        output::conflict(&format!(
                            "{} (remote mismatch: {} != {})",
                            repo.path,
                            current_remote.trim(),
                            repo.remote
                        ));
                        report.conflicts.push(repo.path.clone());
                    }
                    Err(_) => {
                        output::conflict(&format!(
                            "{} (directory exists but is not a git repo)",
                            repo.path
                        ));
                        report.conflicts.push(repo.path.clone());
                    }
                }
            } else {
                // Clone
                if ctx.dry_run {
                    output::success(&format!("{} (would clone from {})", repo.path, repo.remote));
                    report.succeeded.push(repo.path.clone());
                } else {
                    // Ensure parent directory exists
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }

                    let mut args = vec!["clone", &repo.remote, path.to_str().unwrap_or("")];
                    if let Some(branch) = &repo.branch {
                        args.push("--branch");
                        args.push(branch);
                    }

                    match ctx.runner.run_success("git", &args) {
                        Ok(_) => {
                            output::success(&format!(
                                "{} (cloned from {})",
                                repo.path, repo.remote
                            ));
                            report.succeeded.push(repo.path.clone());
                        }
                        Err(e) => {
                            output::error(&format!("{}: {}", repo.path, e));
                            report.failed.push((repo.path.clone(), e.to_string()));
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
    use tempfile::TempDir;

    #[test]
    fn test_clones_missing_repo() {
        let tmp = TempDir::new().unwrap();
        let repo_path = tmp.path().join("code").join("myrepo");

        let config: Config = toml::from_str(&format!(
            r#"
[[repositories]]
path = "{}"
remote = "git@github.com:user/repo.git"
"#,
            repo_path.display()
        ))
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success(""); // git clone succeeds

        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Repositories.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);

        let calls = runner.calls.borrow();
        assert_eq!(calls[0].0, "git");
        assert!(calls[0].1.contains(&"clone".to_string()));
    }

    #[test]
    fn test_skips_existing_repo_with_correct_remote() {
        let tmp = TempDir::new().unwrap();
        let repo_path = tmp.path().join("myrepo");
        std::fs::create_dir_all(&repo_path).unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[[repositories]]
path = "{}"
remote = "git@github.com:user/repo.git"
"#,
            repo_path.display()
        ))
        .unwrap();

        let runner = MockRunner::new();
        // git remote get-url origin returns the correct URL
        runner.push_success("git@github.com:user/repo.git");

        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Repositories.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
    }

    #[test]
    fn test_detects_remote_mismatch() {
        let tmp = TempDir::new().unwrap();
        let repo_path = tmp.path().join("myrepo");
        std::fs::create_dir_all(&repo_path).unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[[repositories]]
path = "{}"
remote = "git@github.com:user/repo.git"
"#,
            repo_path.display()
        ))
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success("git@github.com:other/repo.git");

        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Repositories.apply(&ctx).unwrap();
        assert_eq!(report.conflicts.len(), 1);
    }

    #[test]
    fn test_pulls_when_configured() {
        let tmp = TempDir::new().unwrap();
        let repo_path = tmp.path().join("myrepo");
        std::fs::create_dir_all(&repo_path).unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[[repositories]]
path = "{}"
remote = "git@github.com:user/repo.git"
pull = true
"#,
            repo_path.display()
        ))
        .unwrap();

        let runner = MockRunner::new();
        runner.push_success("git@github.com:user/repo.git"); // remote check
        runner.push_success("Already up to date."); // pull

        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Repositories.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);

        let calls = runner.calls.borrow();
        assert_eq!(calls.len(), 2);
        assert!(calls[1].1.contains(&"pull".to_string()));
    }
}
