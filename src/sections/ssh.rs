use anyhow::{Context as AnyhowContext, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

use super::{Context, Section, SectionReport};
use crate::config::expand_tilde;
use crate::output;

pub struct Ssh;

impl Section for Ssh {
    fn name(&self) -> &'static str {
        "ssh"
    }

    fn apply(&self, ctx: &Context) -> Result<SectionReport> {
        let mut report = SectionReport::new("ssh");

        let ssh_config = match &ctx.config.ssh {
            Some(c) => c,
            None => return Ok(report),
        };

        output::section_header("ssh");

        // Generate keys
        if let Some(keys) = &ssh_config.keys {
            for key in keys {
                let key_path = expand_tilde(&key.path)?;

                if key_path.exists() {
                    output::skipped(&format!("{} (exists)", key.path));
                    report.skipped.push(key.path.clone());
                } else if ctx.dry_run {
                    output::success(&format!("{} (would generate {} key)", key.path, key.key_type));
                    report.succeeded.push(key.path.clone());
                } else {
                    // Ensure .ssh directory exists with correct permissions
                    if let Some(parent) = key_path.parent() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("creating {}", parent.display()))?;
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            fs::set_permissions(parent, fs::Permissions::from_mode(0o700))?;
                        }
                    }

                    let mut args = vec![
                        "-t",
                        &key.key_type,
                        "-f",
                        key_path.to_str().unwrap_or(&key.path),
                        "-N",
                        "", // no passphrase
                    ];

                    if let Some(comment) = &key.comment {
                        args.push("-C");
                        args.push(comment);
                    }

                    match ctx.runner.run_success("ssh-keygen", &args) {
                        Ok(_) => {
                            output::success(&format!(
                                "{} (generated {} key)",
                                key.path, key.key_type
                            ));
                            report.succeeded.push(key.path.clone());
                        }
                        Err(e) => {
                            output::error(&format!("{}: {}", key.path, e));
                            report.failed.push((key.path.clone(), e.to_string()));
                        }
                    }
                }
            }
        }

        // Write SSH config entries
        if let Some(entries) = &ssh_config.config_entries {
            let ssh_config_path = ctx.home_dir.join(".ssh").join("config");
            let existing_content = if ssh_config_path.exists() {
                fs::read_to_string(&ssh_config_path).unwrap_or_default()
            } else {
                String::new()
            };

            let mut new_entries = Vec::new();

            for entry in entries {
                // Check if this host is already configured
                let host_pattern = format!("Host {}", entry.host);
                if existing_content.contains(&host_pattern) {
                    output::skipped(&format!("ssh config: Host {} (exists)", entry.host));
                    report
                        .skipped
                        .push(format!("ssh config: Host {}", entry.host));
                    continue;
                }

                let mut block = format!("Host {}\n", entry.host);
                if let Some(identity) = &entry.identity_file {
                    let expanded = expand_tilde(identity)?;
                    block.push_str(&format!("  IdentityFile {}\n", expanded.display()));
                }
                if entry.add_keys_to_agent {
                    block.push_str("  AddKeysToAgent yes\n");
                }
                if entry.use_keychain {
                    block.push_str("  UseKeychain yes\n");
                }

                new_entries.push((entry.host.clone(), block));
            }

            if !new_entries.is_empty() {
                if ctx.dry_run {
                    for (host, _) in &new_entries {
                        output::success(&format!("ssh config: Host {} (would add)", host));
                        report
                            .succeeded
                            .push(format!("ssh config: Host {}", host));
                    }
                } else {
                    // Ensure .ssh dir exists
                    let ssh_dir = ctx.home_dir.join(".ssh");
                    fs::create_dir_all(&ssh_dir)?;

                    let mut file = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&ssh_config_path)
                        .with_context(|| {
                            format!("opening {}", ssh_config_path.display())
                        })?;

                    for (host, block) in &new_entries {
                        writeln!(file, "\n{}", block.trim_end())?;
                        output::success(&format!("ssh config: Host {} (added)", host));
                        report
                            .succeeded
                            .push(format!("ssh config: Host {}", host));
                    }

                    // Set correct permissions on ssh config
                    set_file_permissions(&ssh_config_path, 0o600)?;
                }
            }
        }

        output::summary(&report);
        Ok(report)
    }
}

#[cfg(unix)]
fn set_file_permissions(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_file_permissions(_path: &Path, _mode: u32) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::exec::tests::MockRunner;
    use tempfile::TempDir;

    #[test]
    fn test_skips_existing_keys() {
        let tmp = TempDir::new().unwrap();
        let key_path = tmp.path().join(".ssh").join("id_ed25519");
        fs::create_dir_all(key_path.parent().unwrap()).unwrap();
        fs::write(&key_path, "fake key").unwrap();

        let config: Config = toml::from_str(&format!(
            r#"
[[ssh.keys]]
path = "{}"
type = "ed25519"
comment = "test@test.com"
"#,
            key_path.display()
        ))
        .unwrap();

        let runner = MockRunner::new();
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Ssh.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
        assert!(report.succeeded.is_empty());
    }

    #[test]
    fn test_writes_ssh_config_entries() {
        let tmp = TempDir::new().unwrap();
        let ssh_dir = tmp.path().join(".ssh");
        fs::create_dir_all(&ssh_dir).unwrap();

        let config: Config = toml::from_str(
            r#"
[[ssh.config_entries]]
host = "github.com"
identity_file = "/tmp/test_key"
add_keys_to_agent = true
use_keychain = true
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Ssh.apply(&ctx).unwrap();
        assert_eq!(report.succeeded.len(), 1);

        let ssh_config = fs::read_to_string(ssh_dir.join("config")).unwrap();
        assert!(ssh_config.contains("Host github.com"));
        assert!(ssh_config.contains("IdentityFile /tmp/test_key"));
        assert!(ssh_config.contains("AddKeysToAgent yes"));
        assert!(ssh_config.contains("UseKeychain yes"));
    }

    #[test]
    fn test_skips_existing_ssh_config_entries() {
        let tmp = TempDir::new().unwrap();
        let ssh_dir = tmp.path().join(".ssh");
        fs::create_dir_all(&ssh_dir).unwrap();
        fs::write(
            ssh_dir.join("config"),
            "Host github.com\n  IdentityFile ~/.ssh/id_ed25519\n",
        )
        .unwrap();

        let config: Config = toml::from_str(
            r#"
[[ssh.config_entries]]
host = "github.com"
identity_file = "~/.ssh/id_ed25519"
"#,
        )
        .unwrap();

        let runner = MockRunner::new();
        let ctx = Context {
            config: &config,
            home_dir: tmp.path().to_path_buf(),
            dry_run: false,
            force: false,
            runner: &runner,
        };

        let report = Ssh.apply(&ctx).unwrap();
        assert_eq!(report.skipped.len(), 1);
        assert!(report.succeeded.is_empty());
    }
}
