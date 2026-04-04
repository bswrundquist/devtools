use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Top-level config. Every section is optional so the TOML can include any subset.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub meta: Option<Meta>,
    pub directories: Option<DirectoriesConfig>,
    pub repositories: Option<Vec<RepoConfig>>,
    pub dotfiles: Option<DotfilesConfig>,
    pub git: Option<GitConfig>,
    pub ssh: Option<SshConfig>,
    pub packages: Option<PackagesConfig>,
    pub claude: Option<ClaudeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub description: Option<String>,
}

// --- Directories ---

#[derive(Debug, Deserialize)]
pub struct DirectoriesConfig {
    pub paths: Vec<String>,
}

// --- Repositories ---

#[derive(Debug, Deserialize)]
pub struct RepoConfig {
    pub path: String,
    pub remote: String,
    pub branch: Option<String>,
    #[serde(default)]
    pub pull: bool,
}

// --- Dotfiles ---

#[derive(Debug, Deserialize)]
pub struct DotfilesConfig {
    pub source_dir: Option<String>,
    pub links: Vec<DotfileLink>,
}

#[derive(Debug, Deserialize)]
pub struct DotfileLink {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub absolute: bool,
}

// --- Git ---

#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub global: Option<BTreeMap<String, String>>,
}

// --- SSH ---

#[derive(Debug, Deserialize)]
pub struct SshConfig {
    pub keys: Option<Vec<SshKeyConfig>>,
    pub agent: Option<SshAgentConfig>,
    pub config_entries: Option<Vec<SshConfigEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct SshKeyConfig {
    pub path: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SshAgentConfig {
    #[serde(default)]
    pub keychain: bool,
}

#[derive(Debug, Deserialize)]
pub struct SshConfigEntry {
    pub host: String,
    pub identity_file: Option<String>,
    #[serde(default)]
    pub add_keys_to_agent: bool,
    #[serde(default)]
    pub use_keychain: bool,
}

// --- Packages ---

#[derive(Debug, Deserialize)]
pub struct PackagesConfig {
    #[serde(default)]
    pub formulae: Vec<String>,
    #[serde(default)]
    pub casks: Vec<String>,
    #[serde(default)]
    pub runtimes: Vec<RuntimeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub check: String,
    pub install_cmd: String,
}

// --- Claude ---

#[derive(Debug, Deserialize)]
pub struct ClaudeConfig {
    #[serde(default = "default_true")]
    pub user: bool,
    #[serde(default)]
    pub repo: bool,
    #[serde(default)]
    pub components: ClaudeComponents,
    pub repo_root: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeComponents {
    #[serde(default = "default_true")]
    pub claude_md: bool,
    #[serde(default = "default_true")]
    pub skills: bool,
    #[serde(default = "default_true")]
    pub agents: bool,
    #[serde(default = "default_true")]
    pub rules: bool,
}

impl Default for ClaudeComponents {
    fn default() -> Self {
        Self {
            claude_md: true,
            skills: true,
            agents: true,
            rules: true,
        }
    }
}

fn default_true() -> bool {
    true
}
