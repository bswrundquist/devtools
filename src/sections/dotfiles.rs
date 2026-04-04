use anyhow::{Context as AnyhowContext, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::PathBuf;

use super::{Context, Section, SectionReport};
use crate::config::expand_tilde;
use crate::output;

pub struct Dotfiles;

impl Section for Dotfiles {
    fn name(&self) -> &'static str {
        "dotfiles"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("dotfiles");

        let df_config = match &ctx.config.dotfiles {
            Some(c) => c,
            None => return Ok(report),
        };

        output::section_header("dotfiles");

        let source_dir = df_config
            .source_dir
            .as_deref()
            .map(expand_tilde)
            .transpose()?;

        for link in &df_config.links {
            let source_path = if link.absolute {
                PathBuf::from(&link.source)
            } else if let Some(ref base) = source_dir {
                base.join(&link.source)
            } else {
                output::error(&format!(
                    "{}: relative source path but no source_dir set",
                    link.source
                ));
                report.failed.push((
                    link.target.clone(),
                    "relative source path but no source_dir configured".to_string(),
                ));
                continue;
            };

            let target_path = expand_tilde(&link.target)?;

            // Check if source exists
            if !source_path.exists() {
                output::error(&format!(
                    "{} -> {}: source does not exist",
                    link.target,
                    source_path.display()
                ));
                report.failed.push((
                    link.target.clone(),
                    format!("source {} does not exist", source_path.display()),
                ));
                continue;
            }

            if target_path.is_symlink() {
                let current_target = fs::read_link(&target_path)
                    .with_context(|| format!("reading symlink {}", target_path.display()))?;
                if current_target == source_path {
                    output::skipped(&format!("{} (correct symlink)", link.target));
                    report.skipped.push(link.target.clone());
                    continue;
                }

                // Symlink exists but points elsewhere
                if ctx.force {
                    if ctx.dry_run {
                        output::success(&format!(
                            "{} (would relink -> {})",
                            link.target,
                            source_path.display()
                        ));
                    } else {
                        fs::remove_file(&target_path)?;
                        unix_fs::symlink(&source_path, &target_path)?;
                        output::success(&format!(
                            "{} (relinked -> {})",
                            link.target,
                            source_path.display()
                        ));
                    }
                    report.succeeded.push(link.target.clone());
                } else {
                    output::conflict(&format!(
                        "{} -> {} (points to {}, use --force)",
                        link.target,
                        source_path.display(),
                        current_target.display()
                    ));
                    report.conflicts.push(link.target.clone());
                }
            } else if target_path.exists() {
                // Regular file exists at target
                if ctx.force {
                    if ctx.dry_run {
                        output::success(&format!(
                            "{} (would replace file with symlink -> {})",
                            link.target,
                            source_path.display()
                        ));
                    } else {
                        fs::remove_file(&target_path)?;
                        unix_fs::symlink(&source_path, &target_path)?;
                        output::success(&format!(
                            "{} (replaced file with symlink -> {})",
                            link.target,
                            source_path.display()
                        ));
                    }
                    report.succeeded.push(link.target.clone());
                } else {
                    output::conflict(&format!(
                        "{} (file exists, use --force to replace with symlink)",
                        link.target
                    ));
                    report.conflicts.push(link.target.clone());
                }
            } else {
                // Target doesn't exist — create symlink
                if ctx.dry_run {
                    output::success(&format!(
                        "{} -> {} (would create)",
                        link.target,
                        source_path.display()
                    ));
                } else {
                    if let Some(parent) = target_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    unix_fs::symlink(&source_path, &target_path)?;
                    output::success(&format!(
                        "{} -> {} (created)",
                        link.target,
                        source_path.display()
                    ));
                }
                report.succeeded.push(link.target.clone());
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
    use crate::exec::RealRunner;
    use tempfile::TempDir;

    #[test]
    fn test_creates_symlinks() {
        let tmp = TempDir::new().unwrap();
        let source_dir = tmp.path().join("dotfiles");
        let target_dir = tmp.path().join("home");

        // Create source file
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join(".zshrc"), "# zshrc").unwrap();

        let target_path = target_dir.join(".zshrc");

        let config: Config = toml::from_str(&format!(
            r#"
[dotfiles]
source_dir = "{}"

[[dotfiles.links]]
source = ".zshrc"
target = "{}"
"#,
            source_dir.display(),
            target_path.display()
        ))
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Dotfiles.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);
        assert!(target_path.is_symlink());
        assert_eq!(fs::read_link(&target_path).unwrap(), source_dir.join(".zshrc"));
    }

    #[test]
    fn test_skips_correct_symlinks() {
        let tmp = TempDir::new().unwrap();
        let source_dir = tmp.path().join("dotfiles");
        let target_dir = tmp.path().join("home");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(source_dir.join(".zshrc"), "# zshrc").unwrap();

        let target_path = target_dir.join(".zshrc");
        unix_fs::symlink(source_dir.join(".zshrc"), &target_path).unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[dotfiles]
source_dir = "{}"

[[dotfiles.links]]
source = ".zshrc"
target = "{}"
"#,
            source_dir.display(),
            target_path.display()
        ))
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Dotfiles.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
        assert!(report.succeeded.is_empty());
    }

    #[test]
    fn test_conflict_when_file_exists() {
        let tmp = TempDir::new().unwrap();
        let source_dir = tmp.path().join("dotfiles");
        let target_dir = tmp.path().join("home");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(source_dir.join(".zshrc"), "# source").unwrap();

        let target_path = target_dir.join(".zshrc");
        fs::write(&target_path, "# existing file").unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[dotfiles]
source_dir = "{}"

[[dotfiles.links]]
source = ".zshrc"
target = "{}"
"#,
            source_dir.display(),
            target_path.display()
        ))
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Dotfiles.apply(&ctx).unwrap();
        assert_eq!(report.conflicts.len(), 1);
        // File should be preserved
        assert!(target_path.is_file());
        assert!(!target_path.is_symlink());
    }

    #[test]
    fn test_force_replaces_existing_file() {
        let tmp = TempDir::new().unwrap();
        let source_dir = tmp.path().join("dotfiles");
        let target_dir = tmp.path().join("home");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(source_dir.join(".zshrc"), "# source").unwrap();

        let target_path = target_dir.join(".zshrc");
        fs::write(&target_path, "# existing").unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[dotfiles]
source_dir = "{}"

[[dotfiles.links]]
source = ".zshrc"
target = "{}"
"#,
            source_dir.display(),
            target_path.display()
        ))
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: true,
            runner: &runner,
        };

        let report = Dotfiles.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);
        assert!(target_path.is_symlink());
    }
}
