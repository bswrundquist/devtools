pub mod schema;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub use schema::Config;

/// Default config location: ~/.config/devtools/devtools.toml
pub fn default_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("could not determine home directory")?;
    Ok(home.join(".config").join("devtools").join("devtools.toml"))
}

/// Load and parse config from the given path.
pub fn load_config(path: &Path) -> Result<Config> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let config: Config =
        toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
    Ok(config)
}

/// Expand `~` at the start of a path string to the user's home directory.
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = dirs::home_dir().context("could not determine home directory")?;
        Ok(home.join(rest))
    } else if path == "~" {
        dirs::home_dir().context("could not determine home directory")
    } else {
        Ok(PathBuf::from(path))
    }
}

/// Starter config content for `devtools init`.
pub fn starter_config() -> &'static str {
    r#"[meta]
description = "My machine bootstrap config"

# 1. Directories — ensure these exist
[directories]
paths = [
  "~/code",
  "~/code/work",
  "~/code/personal",
]

# 2. Repositories — clone if missing
# [[repositories]]
# path = "~/code/personal/dotfiles"
# remote = "git@github.com:user/dotfiles.git"
# pull = true

# 3. Dotfiles — symlink management
# [dotfiles]
# source_dir = "~/code/personal/dotfiles"
#
# [[dotfiles.links]]
# source = "zsh/.zshrc"
# target = "~/.zshrc"

# 4. Git — global config values
# [git.global]
# "user.name" = "Your Name"
# "user.email" = "you@example.com"
# "init.defaultBranch" = "main"

# 5. SSH — key generation
# [[ssh.keys]]
# path = "~/.ssh/id_ed25519"
# type = "ed25519"
# comment = "you@example.com"

# 6. Packages — Homebrew
# [packages]
# formulae = ["git", "gh", "jq", "ripgrep", "fd", "bat", "fzf"]
# casks = ["ghostty", "raycast", "visual-studio-code"]

# 7. Claude — scaffolding sync
# [claude]
# user = true
# repo = false
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let home = dirs::home_dir().unwrap();
        assert_eq!(expand_tilde("~/code").unwrap(), home.join("code"));
        assert_eq!(expand_tilde("~").unwrap(), home);
        assert_eq!(
            expand_tilde("/absolute/path").unwrap(),
            PathBuf::from("/absolute/path")
        );
        assert_eq!(
            expand_tilde("relative/path").unwrap(),
            PathBuf::from("relative/path")
        );
    }

    #[test]
    fn test_parse_starter_config() {
        let config: Config = toml::from_str(starter_config()).unwrap();
        let dirs = config.directories.unwrap();
        assert_eq!(dirs.paths.len(), 3);
        assert_eq!(dirs.paths[0], "~/code");
    }

    #[test]
    fn test_parse_full_config() {
        let toml_str = r#"
[meta]
description = "test config"

[directories]
paths = ["~/code", "~/bin"]

[[repositories]]
path = "~/code/myrepo"
remote = "git@github.com:user/repo.git"
pull = true

[[repositories]]
path = "~/code/other"
remote = "git@github.com:user/other.git"

[dotfiles]
source_dir = "~/dotfiles"

[[dotfiles.links]]
source = "zsh/.zshrc"
target = "~/.zshrc"

[git.global]
"user.name" = "Test"
"user.email" = "test@test.com"

[[ssh.keys]]
path = "~/.ssh/id_ed25519"
type = "ed25519"
comment = "test@test.com"

[packages]
formulae = ["git", "jq"]
casks = ["docker"]

[[packages.runtimes]]
name = "rust"
check = "rustup --version"
install_cmd = "curl https://sh.rustup.rs | sh"

[claude]
user = true
repo = false
components = { claude_md = true, skills = true, agents = false, rules = true }
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.meta.is_some());
        assert_eq!(config.directories.unwrap().paths.len(), 2);
        assert_eq!(config.repositories.unwrap().len(), 2);
        assert_eq!(config.dotfiles.unwrap().links.len(), 1);
        assert_eq!(config.git.unwrap().global.unwrap().len(), 2);
        assert_eq!(config.ssh.unwrap().keys.unwrap().len(), 1);
        let pkgs = config.packages.unwrap();
        assert_eq!(pkgs.formulae.len(), 2);
        assert_eq!(pkgs.casks.len(), 1);
        assert_eq!(pkgs.runtimes.len(), 1);
        let claude = config.claude.unwrap();
        assert!(claude.user);
        assert!(!claude.repo);
        assert!(!claude.components.agents);
    }
}
