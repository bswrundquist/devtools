use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Result of a merge-copy sync operation.
pub struct SyncResult {
    pub copied: Vec<PathBuf>,
    pub unchanged: Vec<PathBuf>,
    pub conflicts: Vec<PathBuf>,
}

impl SyncResult {
    pub fn new() -> Self {
        Self {
            copied: Vec::new(),
            unchanged: Vec::new(),
            conflicts: Vec::new(),
        }
    }
}

/// Merge-copy engine: walks source, compares with dest, copies what's needed.
pub struct SyncEngine {
    pub dry_run: bool,
    pub force: bool,
}

impl SyncEngine {
    pub fn new(dry_run: bool, force: bool) -> Self {
        Self { dry_run, force }
    }

    /// Sync from one filesystem directory to another.
    /// `include` is an optional filter: if provided, only files where include(relative_path)
    /// returns true are synced. `.gitkeep` files are always skipped.
    pub fn sync_fs_dir(
        &self,
        source: &Path,
        dest: &Path,
        include: Option<&dyn Fn(&Path) -> bool>,
    ) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        if !source.exists() {
            return Ok(result);
        }

        let mut entries: Vec<_> = walkdir::WalkDir::new(source)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        entries.sort_by(|a, b| a.path().cmp(b.path()));

        for entry in entries {
            let src_path = entry.path();
            let rel_path = src_path
                .strip_prefix(source)
                .context("stripping source prefix")?;

            // Skip .gitkeep
            if rel_path
                .file_name()
                .map(|n| n == ".gitkeep")
                .unwrap_or(false)
            {
                continue;
            }

            // Apply include filter
            if let Some(filter) = &include {
                if !filter(rel_path) {
                    continue;
                }
            }

            let dst_path = dest.join(rel_path);

            if !dst_path.exists() {
                // Copy missing file
                if !self.dry_run {
                    if let Some(parent) = dst_path.parent() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("creating {}", parent.display()))?;
                    }
                    fs::copy(src_path, &dst_path).with_context(|| {
                        format!("copying {} -> {}", src_path.display(), dst_path.display())
                    })?;
                }
                result.copied.push(rel_path.to_path_buf());
            } else if files_identical(src_path, &dst_path)? {
                // Skip identical
                result.unchanged.push(rel_path.to_path_buf());
            } else if self.force {
                // Overwrite with --force
                if !self.dry_run {
                    fs::copy(src_path, &dst_path).with_context(|| {
                        format!("overwriting {}", dst_path.display())
                    })?;
                }
                result.copied.push(rel_path.to_path_buf());
            } else {
                // Conflict
                result.conflicts.push(rel_path.to_path_buf());
            }
        }

        Ok(result)
    }

    /// Sync from an embedded include_dir directory to a filesystem path.
    /// File paths from include_dir are relative to the root Dir, so we strip
    /// the source dir's own path prefix to get paths relative to `source`.
    pub fn sync_embedded_dir(
        &self,
        source: &include_dir::Dir<'static>,
        dest: &Path,
        include: Option<&dyn Fn(&Path) -> bool>,
    ) -> Result<SyncResult> {
        let mut result = SyncResult::new();
        let base_prefix = source.path();

        let mut files: Vec<_> = collect_embedded_files(source);
        files.sort_by(|a, b| a.0.cmp(&b.0));

        for (full_path, content) in files {
            // Strip the source dir prefix to get the relative path
            let rel_path = full_path
                .strip_prefix(base_prefix)
                .unwrap_or(&full_path)
                .to_path_buf();
            // Skip .gitkeep
            if rel_path
                .file_name()
                .map(|n| n == ".gitkeep")
                .unwrap_or(false)
            {
                continue;
            }

            // Apply include filter
            if let Some(filter) = &include {
                if !filter(&rel_path) {
                    continue;
                }
            }

            let dst_path = dest.join(&rel_path);

            if !dst_path.exists() {
                if !self.dry_run {
                    if let Some(parent) = dst_path.parent() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("creating {}", parent.display()))?;
                    }
                    fs::write(&dst_path, content)
                        .with_context(|| format!("writing {}", dst_path.display()))?;
                }
                result.copied.push(rel_path);
            } else {
                let existing =
                    fs::read(&dst_path).with_context(|| format!("reading {}", dst_path.display()))?;
                if existing == content {
                    result.unchanged.push(rel_path);
                } else if self.force {
                    if !self.dry_run {
                        fs::write(&dst_path, content)
                            .with_context(|| format!("overwriting {}", dst_path.display()))?;
                    }
                    result.copied.push(rel_path);
                } else {
                    result.conflicts.push(rel_path);
                }
            }
        }

        Ok(result)
    }
}

/// Byte-for-byte file comparison (equivalent to Python's filecmp.cmp with shallow=False).
fn files_identical(a: &Path, b: &Path) -> Result<bool> {
    let content_a = fs::read(a).with_context(|| format!("reading {}", a.display()))?;
    let content_b = fs::read(b).with_context(|| format!("reading {}", b.display()))?;
    Ok(content_a == content_b)
}

/// Recursively collect all files from an embedded directory.
fn collect_embedded_files(dir: &include_dir::Dir<'static>) -> Vec<(PathBuf, &'static [u8])> {
    let mut files = Vec::new();
    for file in dir.files() {
        files.push((file.path().to_path_buf(), file.contents()));
    }
    for subdir in dir.dirs() {
        files.extend(collect_embedded_files(subdir));
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_source(dir: &Path) {
        fs::create_dir_all(dir.join("skills/git")).unwrap();
        fs::write(dir.join("CLAUDE.md"), "# Claude").unwrap();
        fs::write(dir.join("skills/git/SKILL.md"), "# Git Skill").unwrap();
        fs::write(dir.join(".gitkeep"), "").unwrap();
    }

    #[test]
    fn test_sync_copies_missing_files() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        let engine = SyncEngine::new(false, false);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert_eq!(result.copied.len(), 2);
        assert!(result.unchanged.is_empty());
        assert!(result.conflicts.is_empty());
        assert!(dst.path().join("CLAUDE.md").exists());
        assert!(dst.path().join("skills/git/SKILL.md").exists());
        // .gitkeep should not be copied
        assert!(!dst.path().join(".gitkeep").exists());
    }

    #[test]
    fn test_sync_skips_identical() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        // Pre-populate dest with identical content
        fs::write(dst.path().join("CLAUDE.md"), "# Claude").unwrap();

        let engine = SyncEngine::new(false, false);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert_eq!(result.copied.len(), 1); // only skills/git/SKILL.md
        assert_eq!(result.unchanged.len(), 1); // CLAUDE.md
    }

    #[test]
    fn test_sync_detects_conflicts() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        // Pre-populate dest with different content
        fs::write(dst.path().join("CLAUDE.md"), "# Different").unwrap();

        let engine = SyncEngine::new(false, false);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.copied.len(), 1);
        // Original content should be preserved
        assert_eq!(fs::read_to_string(dst.path().join("CLAUDE.md")).unwrap(), "# Different");
    }

    #[test]
    fn test_sync_force_overwrites_conflicts() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        fs::write(dst.path().join("CLAUDE.md"), "# Different").unwrap();

        let engine = SyncEngine::new(false, true);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert!(result.conflicts.is_empty());
        assert_eq!(result.copied.len(), 2);
        assert_eq!(fs::read_to_string(dst.path().join("CLAUDE.md")).unwrap(), "# Claude");
    }

    #[test]
    fn test_sync_dry_run() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        let engine = SyncEngine::new(true, false);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert_eq!(result.copied.len(), 2);
        // Files should NOT be created in dry run
        assert!(!dst.path().join("CLAUDE.md").exists());
    }

    #[test]
    fn test_sync_with_include_filter() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();
        setup_source(src.path());

        let engine = SyncEngine::new(false, false);
        let include = |path: &Path| -> bool {
            path.starts_with("skills")
        };
        let result = engine.sync_fs_dir(src.path(), dst.path(), Some(&include)).unwrap();

        assert_eq!(result.copied.len(), 1);
        assert!(!dst.path().join("CLAUDE.md").exists());
        assert!(dst.path().join("skills/git/SKILL.md").exists());
    }

    #[test]
    fn test_sync_empty_source() {
        let src = TempDir::new().unwrap();
        let dst = TempDir::new().unwrap();

        let engine = SyncEngine::new(false, false);
        let result = engine.sync_fs_dir(src.path(), dst.path(), None).unwrap();

        assert!(result.copied.is_empty());
        assert!(result.unchanged.is_empty());
        assert!(result.conflicts.is_empty());
    }

    #[test]
    fn test_sync_nonexistent_source() {
        let dst = TempDir::new().unwrap();

        let engine = SyncEngine::new(false, false);
        let result = engine
            .sync_fs_dir(Path::new("/nonexistent/path"), dst.path(), None)
            .unwrap();

        assert!(result.copied.is_empty());
    }
}
