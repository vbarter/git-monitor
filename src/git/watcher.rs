use crate::event::Event;
use color_eyre::Result;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, Debouncer};
use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Git file watcher using notify-rs with debouncing
pub struct GitWatcher {
    _watcher: Debouncer<RecommendedWatcher>,
}

impl GitWatcher {
    pub fn new(repo_path: PathBuf, sender: mpsc::UnboundedSender<Event>) -> Result<Self> {
        let (tx, rx) = std_mpsc::channel();

        // Create debounced watcher with 200ms debounce
        let mut debouncer = new_debouncer(Duration::from_millis(200), tx)?;

        // Watch the repository directory
        debouncer
            .watcher()
            .watch(&repo_path, RecursiveMode::Recursive)?;

        // Spawn a thread to forward events
        let sender_clone = sender.clone();
        let repo_path_clone = repo_path.clone();
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(Ok(events)) => {
                        // Collect changed file paths
                        let mut changed_paths: Vec<String> = Vec::new();
                        let mut has_git_change = false;

                        for e in events.iter() {
                            let path = &e.path;
                            let path_str = path.to_string_lossy();

                            // Skip directories we don't care about
                            if path_str.contains(".git/objects")
                                || path_str.contains(".git/logs")
                                || path_str.contains(".git/FETCH_HEAD")
                                || path_str.contains("/target/")
                                || path_str.contains("/.git/hooks")
                                || path_str.ends_with(".swp")
                                || path_str.ends_with(".swo")
                                || path_str.ends_with("~")
                            {
                                continue;
                            }

                            // Check if it's a git metadata change
                            if path_str.contains(".git/index") || path_str.contains(".git/HEAD") {
                                has_git_change = true;
                                continue;
                            }

                            // It's a working directory file
                            if !path_str.contains("/.git/") {
                                // Get relative path from repo root
                                if let Ok(rel_path) = path.strip_prefix(&repo_path_clone) {
                                    changed_paths.push(rel_path.to_string_lossy().to_string());
                                } else {
                                    changed_paths.push(path_str.to_string());
                                }
                                has_git_change = true;
                            }
                        }

                        if has_git_change {
                            let _ = sender_clone.send(Event::GitChange(changed_paths));
                        }
                    }
                    Ok(Err(_e)) => {
                        // Ignore watch errors silently
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            _watcher: debouncer,
        })
    }
}
