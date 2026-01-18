use color_eyre::Result;
use git2::{DiffOptions, Repository, Status, StatusOptions};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// File status types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Conflicted,
}

impl FileStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            FileStatus::Modified => "M",
            FileStatus::Added => "A",
            FileStatus::Deleted => "D",
            FileStatus::Renamed => "R",
            FileStatus::Untracked => "U",
            FileStatus::Conflicted => "!",
        }
    }
}

/// Represents a changed file
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub status: FileStatus,
    pub staged: bool,
    /// Number of lines added
    pub additions: i32,
    /// Number of lines deleted
    pub deletions: i32,
    /// File modification time from filesystem
    pub modified_time: Option<SystemTime>,
}

/// Git repository wrapper
pub struct GitRepository {
    repo: Repository,
    path: PathBuf,
}

impl GitRepository {
    pub fn new(path: PathBuf) -> Result<Self> {
        let repo = Repository::discover(&path)?;
        let path = repo.workdir().unwrap_or(path.as_path()).to_path_buf();
        Ok(Self { repo, path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head()?;
        let name = head
            .shorthand()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "HEAD".to_string());
        Ok(name)
    }

    pub fn get_status(&self) -> Result<Vec<FileChange>> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_ignored(false);

        let statuses = self.repo.statuses(Some(&mut opts))?;
        let mut files = Vec::new();

        for entry in statuses.iter() {
            let path = entry.path().unwrap_or("").to_string();
            let status = entry.status();

            let (file_status, staged) = Self::parse_status(status);

            if let Some(file_status) = file_status {
                let (additions, deletions) = self.get_line_changes(&path).unwrap_or((0, 0));
                // Get file modification time from filesystem
                let modified_time = self.get_file_mtime(&path);
                files.push(FileChange {
                    path,
                    status: file_status,
                    staged,
                    additions,
                    deletions,
                    modified_time,
                });
            }
        }

        // Sort by modification time (newest first), then by path for files without mtime
        files.sort_by(|a, b| {
            match (&a.modified_time, &b.modified_time) {
                // Both have modification times: sort by time descending (newest first)
                (Some(t_a), Some(t_b)) => t_b.cmp(t_a),
                // Only a has time: a comes first
                (Some(_), None) => std::cmp::Ordering::Less,
                // Only b has time: b comes first
                (None, Some(_)) => std::cmp::Ordering::Greater,
                // Neither has time: sort by path
                (None, None) => a.path.cmp(&b.path),
            }
        });

        Ok(files)
    }

    /// Get file modification time from filesystem
    fn get_file_mtime(&self, path: &str) -> Option<SystemTime> {
        let full_path = self.path.join(path);
        std::fs::metadata(&full_path)
            .ok()
            .and_then(|m| m.modified().ok())
    }

    fn parse_status(status: Status) -> (Option<FileStatus>, bool) {
        // Check for conflicts first
        if status.is_conflicted() {
            return (Some(FileStatus::Conflicted), false);
        }

        // Check staged changes
        if status.is_index_new() {
            return (Some(FileStatus::Added), true);
        }
        if status.is_index_modified() {
            return (Some(FileStatus::Modified), true);
        }
        if status.is_index_deleted() {
            return (Some(FileStatus::Deleted), true);
        }
        if status.is_index_renamed() {
            return (Some(FileStatus::Renamed), true);
        }

        // Check working tree changes
        if status.is_wt_new() {
            return (Some(FileStatus::Untracked), false);
        }
        if status.is_wt_modified() {
            return (Some(FileStatus::Modified), false);
        }
        if status.is_wt_deleted() {
            return (Some(FileStatus::Deleted), false);
        }
        if status.is_wt_renamed() {
            return (Some(FileStatus::Renamed), false);
        }

        (None, false)
    }

    fn get_line_changes(&self, path: &str) -> Result<(i32, i32)> {
        let mut opts = DiffOptions::new();
        opts.pathspec(path);

        let diff = self.repo.diff_index_to_workdir(None, Some(&mut opts))?;
        let stats = diff.stats()?;

        Ok((stats.insertions() as i32, stats.deletions() as i32))
    }

    pub fn get_diff(&self, path: &str) -> Result<String> {
        let mut opts = DiffOptions::new();
        opts.pathspec(path);

        // Try to get diff from index to workdir first
        let diff = self.repo.diff_index_to_workdir(None, Some(&mut opts))?;

        let mut output = String::new();

        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let prefix = match line.origin() {
                '+' => "+",
                '-' => "-",
                ' ' => " ",
                '@' => "@",
                '>' => ">",
                '<' => "<",
                'F' => "", // File header
                'H' => "", // Hunk header
                'B' => "", // Binary
                _ => "",
            };

            if !prefix.is_empty() || line.origin() == 'F' || line.origin() == 'H' {
                if let Ok(content) = std::str::from_utf8(line.content()) {
                    output.push_str(prefix);
                    output.push_str(content);
                }
            }

            true
        })?;

        // If no workdir diff, try HEAD to index
        if output.is_empty() {
            let head = self.repo.head()?.peel_to_tree()?;
            let diff = self.repo.diff_tree_to_index(Some(&head), None, Some(&mut opts))?;

            diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                let prefix = match line.origin() {
                    '+' => "+",
                    '-' => "-",
                    ' ' => " ",
                    '@' => "@",
                    _ => "",
                };

                if !prefix.is_empty() {
                    if let Ok(content) = std::str::from_utf8(line.content()) {
                        output.push_str(prefix);
                        output.push_str(content);
                    }
                }

                true
            })?;
        }

        Ok(output)
    }

    pub fn stage_file(&self, path: &str) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        Ok(())
    }

    pub fn unstage_file(&self, path: &str) -> Result<()> {
        let head = self.repo.head()?.peel_to_commit()?;
        self.repo
            .reset_default(Some(head.as_object()), [Path::new(path)])?;
        Ok(())
    }
}
