use crate::event::Event;
use crate::git::{FileChange, FileStatus, GitRepository, GitWatcher};
use color_eyre::Result;
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::mpsc;

/// Active panel in the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    FileList,
    DiffView,
}

/// Application state
pub struct App {
    /// Is the application running
    running: bool,
    /// Git repository wrapper
    repo: GitRepository,
    /// File watcher
    watcher: Option<GitWatcher>,
    /// Current file statuses
    pub files: Vec<FileChange>,
    /// Selected file index
    pub selected_index: usize,
    /// Active panel
    pub active_panel: ActivePanel,
    /// Current branch name
    pub branch_name: String,
    /// Last update time
    pub last_update: Instant,
    /// Recently changed files (for animation)
    pub recently_changed: Vec<(String, Instant)>,
    /// Scroll offset for diff view
    pub diff_scroll: usize,
    /// File list area for mouse events (x, y, width, height)
    pub file_list_area: Option<(u16, u16, u16, u16)>,
    /// Diff view area for mouse events (x, y, width, height)
    pub diff_view_area: Option<(u16, u16, u16, u16)>,
}

impl App {
    pub fn new(repo_path: PathBuf) -> Result<Self> {
        let repo = GitRepository::new(repo_path)?;
        let branch_name = repo.current_branch()?;
        let files = repo.get_status()?;

        Ok(Self {
            running: true,
            repo,
            watcher: None,
            files,
            selected_index: 0,
            active_panel: ActivePanel::FileList,
            branch_name,
            last_update: Instant::now(),
            recently_changed: Vec::new(),
            diff_scroll: 0,
            file_list_area: None,
            diff_view_area: None,
        })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn tick(&mut self) {
        // Animation cleanup is handled by is_recently_changed() and get_change_progress()
        // We keep recently_changed records for stable sorting by modification time
    }

    pub fn start_watcher(&mut self, sender: mpsc::UnboundedSender<Event>) -> Result<()> {
        let watcher = GitWatcher::new(self.repo.path().to_path_buf(), sender)?;
        self.watcher = Some(watcher);
        Ok(())
    }

    pub fn refresh_status(&mut self) -> Result<()> {
        self.refresh_status_with_paths(Vec::new())
    }

    pub fn refresh_status_with_paths(&mut self, changed_paths: Vec<String>) -> Result<()> {
        // Get current selected file path to maintain selection after sort
        let selected_path = self.files.get(self.selected_index).map(|f| f.path.clone());

        self.files = self.repo.get_status()?;
        self.branch_name = self.repo.current_branch()?;
        self.last_update = Instant::now();

        let now = Instant::now();

        // Mark changed files for animation
        for path in &changed_paths {
            // Find matching file in the list (handle both exact and partial matches)
            for file in &self.files {
                if file.path == *path || file.path.ends_with(path) || path.ends_with(&file.path) {
                    // Remove old entry for this path and add new one
                    self.recently_changed.retain(|(p, _)| p != &file.path);
                    self.recently_changed.push((file.path.clone(), now));
                    break;
                }
            }
        }

        // Build a map of file paths to their modification times for sorting
        let change_times: std::collections::HashMap<String, Instant> = self
            .recently_changed
            .iter()
            .cloned()
            .collect();

        // Sort files: by modification time (newest first), then by path for files without recorded time
        self.files.sort_by(|a, b| {
            let a_time = change_times.get(&a.path);
            let b_time = change_times.get(&b.path);

            match (a_time, b_time) {
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

        // Clean up modification time records for files that no longer exist
        let current_paths: std::collections::HashSet<&str> =
            self.files.iter().map(|f| f.path.as_str()).collect();
        self.recently_changed
            .retain(|(path, _)| current_paths.contains(path.as_str()));

        // Restore selection to the same file if possible
        if let Some(path) = selected_path {
            if let Some(idx) = self.files.iter().position(|f| f.path == path) {
                self.selected_index = idx;
            }
        }

        // Adjust selection if needed
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        }

        Ok(())
    }

    pub fn select_next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
            self.diff_scroll = 0;
        }
    }

    pub fn select_previous(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = self
                .selected_index
                .checked_sub(1)
                .unwrap_or(self.files.len() - 1);
            self.diff_scroll = 0;
        }
    }

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::FileList => ActivePanel::DiffView,
            ActivePanel::DiffView => ActivePanel::FileList,
        };
    }

    pub fn toggle_stage(&mut self) -> Result<()> {
        if let Some(file) = self.files.get(self.selected_index) {
            let path = file.path.clone();
            let is_staged = file.staged;
            if is_staged {
                self.repo.unstage_file(&path)?;
            } else {
                self.repo.stage_file(&path)?;
            }
            self.refresh_status()?;
        }
        Ok(())
    }

    pub fn scroll_diff_down(&mut self) {
        self.diff_scroll = self.diff_scroll.saturating_add(1);
    }

    pub fn scroll_diff_up(&mut self) {
        self.diff_scroll = self.diff_scroll.saturating_sub(1);
    }

    pub fn selected_file(&self) -> Option<&FileChange> {
        self.files.get(self.selected_index)
    }

    pub fn get_diff(&self) -> Option<String> {
        self.selected_file()
            .and_then(|f| self.repo.get_diff(&f.path).ok())
    }

    pub fn is_recently_changed(&self, path: &str) -> bool {
        let now = Instant::now();
        self.recently_changed
            .iter()
            .any(|(p, time)| p == path && now.duration_since(*time).as_millis() < 800)
    }

    /// Get animation progress for a recently changed file (0.0 to 1.0)
    pub fn get_change_progress(&self, path: &str) -> Option<f64> {
        let now = Instant::now();
        self.recently_changed.iter().find_map(|(p, time)| {
            if p == path {
                let elapsed = now.duration_since(*time).as_millis() as f64;
                if elapsed < 800.0 {
                    Some(elapsed / 800.0)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn staged_count(&self) -> usize {
        self.files.iter().filter(|f| f.staged).count()
    }

    pub fn modified_count(&self) -> usize {
        self.files
            .iter()
            .filter(|f| f.status == FileStatus::Modified && !f.staged)
            .count()
    }

    pub fn untracked_count(&self) -> usize {
        self.files
            .iter()
            .filter(|f| f.status == FileStatus::Untracked)
            .count()
    }

    pub fn seconds_since_update(&self) -> f64 {
        self.last_update.elapsed().as_secs_f64()
    }
}
