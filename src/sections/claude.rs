use anyhow::Result;
use include_dir::{include_dir, Dir};
use std::path::Path;

use super::{Context, Section, SectionReport};
use crate::config::expand_tilde;
use crate::output;
use crate::sync::SyncEngine;

static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/claude");

pub struct Claude;

impl Section for Claude {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("claude");

        let claude_config = match &ctx.config.claude {
            Some(c) => c,
            None => return Ok(report),
        };

        output::section_header("claude");

        let engine = SyncEngine::new(ctx.dry_run, ctx.force);
        let components = &claude_config.components;

        // Build include filter based on enabled components
        let include = move |path: &Path| -> bool {
            // Get the first path component to determine which component this belongs to
            let first = match path.components().next() {
                Some(c) => c.as_os_str().to_string_lossy().to_string(),
                None => return true,
            };

            match first.as_str() {
                "CLAUDE.md" => components.claude_md,
                "skills" => components.skills,
                "agents" => components.agents,
                "rules" => components.rules,
                _ => true,
            }
        };

        // User scope: templates/claude/user/.claude/ -> ~/.claude/
        if claude_config.user {
            let user_templates = TEMPLATES.get_dir("user/.claude");
            let dest = ctx.home_dir.join(".claude");

            if let Some(src_dir) = user_templates {
                output::info("user scope (~/.claude/)");

                // For user scope, rules are skipped (they only make sense in repo scope)
                let user_include = |path: &Path| -> bool {
                    let first = match path.components().next() {
                        Some(c) => c.as_os_str().to_string_lossy().to_string(),
                        None => return true,
                    };
                    if first == "rules" {
                        return false;
                    }
                    include(path)
                };

                let sync_result = engine.sync_embedded_dir(src_dir, &dest, Some(&user_include))?;
                apply_sync_result(&sync_result, &mut report);
            }
        }

        // Repo scope: templates/claude/repo/.claude/ -> <repo_root>/.claude/
        if claude_config.repo {
            let repo_root = match &claude_config.repo_root {
                Some(p) => expand_tilde(&p.to_string_lossy())?,
                None => std::env::current_dir()?,
            };
            let repo_templates = TEMPLATES.get_dir("repo/.claude");
            let dest = repo_root.join(".claude");

            if let Some(src_dir) = repo_templates {
                output::info(&format!("repo scope ({})", dest.display()));
                let sync_result = engine.sync_embedded_dir(src_dir, &dest, Some(&include))?;
                apply_sync_result(&sync_result, &mut report);
            }
        }

        output::summary(&report);
        Ok(report)
    }
}

fn apply_sync_result(sync_result: &crate::sync::SyncResult, report: &mut SectionReport) {
    for path in &sync_result.copied {
        output::success(&format!("{}", path.display()));
        report.succeeded.push(path.display().to_string());
    }
    for path in &sync_result.unchanged {
        output::skipped(&format!("{} (unchanged)", path.display()));
        report.skipped.push(path.display().to_string());
    }
    for path in &sync_result.conflicts {
        output::conflict(&format!("{} (conflict, use --force)", path.display()));
        report.conflicts.push(path.display().to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::exec::RealRunner;
    use tempfile::TempDir;

    #[test]
    fn test_user_scope_copies_templates() {
        let tmp = TempDir::new().unwrap();
        let config: Config = toml::from_str(&format!(
            r#"
[claude]
user = true
repo = false
"#
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

        let report = Claude.apply(&ctx).unwrap();

        // Should have copied some files
        assert!(!report.succeeded.is_empty());
        // .claude directory should exist
        assert!(tmp.path().join(".claude").exists());
        // CLAUDE.md should exist
        assert!(tmp.path().join(".claude/CLAUDE.md").exists());
        // Skills should exist
        assert!(tmp.path().join(".claude/skills").is_dir());
        // Rules should NOT exist in user scope
        let has_rules = report
            .succeeded
            .iter()
            .any(|p| p.starts_with("rules"));
        assert!(!has_rules);
    }

    #[test]
    fn test_disabled_components_are_skipped() {
        let tmp = TempDir::new().unwrap();
        let config: Config = toml::from_str(
            r#"
[claude]
user = true
repo = false
components = { claude_md = true, skills = false, agents = false, rules = false }
"#,
        )
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        Claude.apply(&ctx).unwrap();

        // CLAUDE.md should be copied
        assert!(tmp.path().join(".claude/CLAUDE.md").exists());
        // Skills should NOT be copied
        assert!(!tmp.path().join(".claude/skills").exists());
        // Agents should NOT be copied
        assert!(!tmp.path().join(".claude/agents").exists());
    }

    #[test]
    fn test_idempotent_second_run() {
        let tmp = TempDir::new().unwrap();
        let config: Config = toml::from_str(
            r#"
[claude]
user = true
repo = false
"#,
        )
        .unwrap();

        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        // First run
        let report1 = Claude.apply(&ctx).unwrap();
        assert!(!report1.succeeded.is_empty());

        // Second run — everything should be unchanged
        let report2 = Claude.apply(&ctx).unwrap();
        assert!(report2.succeeded.is_empty());
        assert_eq!(report2.skipped.len(), report1.succeeded.len());
    }

    #[test]
    fn test_no_claude_config_returns_empty() {
        let config: Config = toml::from_str("").unwrap();
        let runner = RealRunner;
        let ctx = Context {
            config: &config,
            home_dir: std::path::PathBuf::from("/tmp"),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Claude.apply(&ctx).unwrap();
        assert!(report.succeeded.is_empty());
        assert!(report.skipped.is_empty());
    }
}
