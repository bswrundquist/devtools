use anyhow::Result;
use std::fs;

use super::{Context, Section, SectionReport};
use crate::config::expand_tilde;
use crate::output;

pub struct Directories;

impl Section for Directories {
    fn name(&self) -> &'static str {
        "directories"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("directories");

        let dirs_config = match &ctx.config.directories {
            Some(c) => c,
            None => return Ok(report),
        };

        output::section_header("directories");

        for path_str in &dirs_config.paths {
            let path = expand_tilde(path_str)?;

            if path.is_dir() {
                output::skipped(&format!("{} (exists)", path_str));
                report.skipped.push(path_str.clone());
            } else if ctx.dry_run {
                output::success(&format!("{} (would create)", path_str));
                report.succeeded.push(path_str.clone());
            } else {
                match fs::create_dir_all(&path) {
                    Ok(_) => {
                        output::success(&format!("{} (created)", path_str));
                        report.succeeded.push(path_str.clone());
                    }
                    Err(e) => {
                        output::error(&format!("{}: {}", path_str, e));
                        report.failed.push((path_str.clone(), e.to_string()));
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
    use crate::exec::RealRunner;
    use tempfile::TempDir;

    fn make_config(paths: Vec<String>) -> Config {
        let toml_str = format!(
            r#"
[directories]
paths = [{}]
"#,
            paths
                .iter()
                .map(|p| format!("\"{}\"", p))
                .collect::<Vec<_>>()
                .join(", ")
        );
        toml::from_str(&toml_str).unwrap()
    }

    #[test]
    fn test_creates_missing_directories() {
        let tmp = TempDir::new().unwrap();
        let code_dir = tmp.path().join("code");
        let work_dir = tmp.path().join("code/work");

        let config = make_config(vec![
            code_dir.to_string_lossy().to_string(),
            work_dir.to_string_lossy().to_string(),
        ]);
        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Directories.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 2);
        assert!(code_dir.is_dir());
        assert!(work_dir.is_dir());
    }

    #[test]
    fn test_skips_existing_directories() {
        let tmp = TempDir::new().unwrap();
        let code_dir = tmp.path().join("code");
        fs::create_dir(&code_dir).unwrap();

        let config = make_config(vec![code_dir.to_string_lossy().to_string()]);
        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Directories.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
        assert!(report.succeeded.is_empty());
    }

    #[test]
    fn test_dry_run_does_not_create() {
        let tmp = TempDir::new().unwrap();
        let code_dir = tmp.path().join("code");

        let config = make_config(vec![code_dir.to_string_lossy().to_string()]);
        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: true,
            force: false,
            runner: &runner,
        };

        let report = Directories.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);
        assert!(!code_dir.exists());
    }

    #[test]
    fn test_no_config_returns_empty_report() {
        let config: Config = toml::from_str("").unwrap();
        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Directories.apply(&ctx).unwrap();
        assert!(report.succeeded.is_empty());
        assert!(report.skipped.is_empty());
    }
}
