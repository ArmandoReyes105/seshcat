mod input;
mod view;

use std::path::{Path, PathBuf};

use crate::models::FsEntry;
use crate::services::filesystem;

/// Cached snapshot of the parent directory, refreshed only at
/// directory-change points (`new`, `go_to`, `enter_selected`,
/// `go_to_parent`) - never during render, and never by `reload_dir`
/// (a child rename never changes the parent's own listing).
pub enum ParentCache {
    /// The current directory has no parent (filesystem root).
    Root,
    /// The parent directory's listing, as of the last refresh.
    Listed(Vec<FsEntry>),
    /// The parent directory could not be listed (e.g. permission denied);
    /// carries the error message.
    Error(String),
}

/// Borrowing view over `NavigationState`'s `parent_cache`, handed to
/// `parent_preview::build_view` without cloning entries.
pub enum ParentSnapshot<'a> {
    Root,
    Listed(&'a [FsEntry]),
    Error(&'a str),
}

pub struct NavigationState {
    current_path: PathBuf,
    entries: Vec<FsEntry>,
    selected: usize,
    parent_cache: ParentCache,
}

impl NavigationState {
    pub fn new(start_path: &Path) -> std::io::Result<Self> {
        let start_entries = filesystem::list_dir(start_path)?;
        let parent_cache = fetch_parent_cache(start_path);

        Ok(Self {
            current_path: start_path.to_path_buf(),
            entries: start_entries,
            selected: 0,
            parent_cache,
        })
    }

    pub fn reload_dir(&mut self) -> std::io::Result<()> {
        self.entries = filesystem::list_dir(&self.current_path)?;

        if !self.entries.is_empty() && self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }
        if self.entries.is_empty() {
            self.selected = 0;
        }
        Ok(())
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn entries(&self) -> &[FsEntry] {
        &self.entries
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_entry(&self) -> Option<&FsEntry> {
        self.entries.get(self.selected)
    }

    /// `true` when the current directory has no parent (filesystem root).
    ///
    /// Kept as a small, independently-tested public predicate even though
    /// the render path no longer calls it directly: the header used to
    /// render `navigation::ROOT_NOTE` here, duplicating the parent-preview
    /// pane's own root indication (see `ParentView::Root`). The pane is now
    /// the single owner of that indication, so this method has no current
    /// caller outside tests.
    #[allow(dead_code)]
    pub fn is_at_root(&self) -> bool {
        is_filesystem_root(&self.current_path)
    }

    /// Borrowing view over the cached parent-directory listing, consumed by
    /// `parent_preview::build_view`. Never triggers IO - the cache is
    /// refreshed only at directory-change points.
    pub fn parent_snapshot(&self) -> ParentSnapshot<'_> {
        match &self.parent_cache {
            ParentCache::Root => ParentSnapshot::Root,
            ParentCache::Listed(entries) => ParentSnapshot::Listed(entries),
            ParentCache::Error(message) => ParentSnapshot::Error(message),
        }
    }

    pub fn go_to(&mut self, path: &Path) -> std::io::Result<()> {
        let entries = filesystem::list_dir(path)?;

        self.current_path = path.to_path_buf();
        self.entries = entries;
        self.selected = 0;
        self.parent_cache = fetch_parent_cache(&self.current_path);

        Ok(())
    }
}

impl NavigationState {
    fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.entries.len();
    }

    fn move_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.selected = (self.selected + self.entries.len() - 1) % self.entries.len();
    }

    fn enter_selected(&mut self) -> std::io::Result<()> {
        let Some(entry) = self.entries.get(self.selected) else {
            return Ok(());
        };

        if !entry.is_dir {
            return Ok(());
        }

        let next_path = entry.path.clone();
        self.entries = filesystem::list_dir(&next_path)?;
        self.current_path = next_path;
        self.selected = 0;
        self.parent_cache = fetch_parent_cache(&self.current_path);

        Ok(())
    }

    fn go_to_parent(&mut self) -> std::io::Result<()> {
        let Some(parent) = self.current_path.parent() else {
            return Ok(());
        };

        let parent_path = parent.to_path_buf();
        self.entries = filesystem::list_dir(&parent_path)?;
        self.current_path = parent_path;
        self.selected = 0;
        self.parent_cache = fetch_parent_cache(&self.current_path);

        Ok(())
    }
}

/// Note shown when the current directory has no parent (filesystem root).
/// Single source of truth for the indication, rendered exclusively by the
/// parent-preview pane's `ParentView::Root` branch (the header must never
/// render it too - that previously caused the note to appear twice).
pub const ROOT_NOTE: &str = "(raíz del filesystem - no hay padre)";

/// Returns `true` when `path` has no parent, i.e. it is a filesystem root
/// (e.g. `C:\` on Windows or `/` on Unix). Pure, no IO.
///
/// Only caller left is `is_at_root` (itself no longer called from the
/// render path - see its doc comment); kept and independently tested.
#[allow(dead_code)]
pub(crate) fn is_filesystem_root(path: &Path) -> bool {
    path.parent().is_none()
}

/// Resolves the parent-directory cache for `current_path`: `Root` when there
/// is no parent, otherwise the parent's listing, or `Error` describing why
/// it could not be listed (e.g. permission denied). The only IO performed is
/// the directory read itself, run once at a directory-change point.
fn fetch_parent_cache(current_path: &Path) -> ParentCache {
    match current_path.parent() {
        None => ParentCache::Root,
        Some(parent) => match filesystem::list_dir(parent) {
            Ok(entries) => ParentCache::Listed(entries),
            Err(e) => ParentCache::Error(e.to_string()),
        },
    }
}

#[cfg(test)]
mod root_tests {
    use super::*;

    #[test]
    fn detects_filesystem_root() {
        #[cfg(windows)]
        let root = PathBuf::from("C:\\");
        #[cfg(not(windows))]
        let root = PathBuf::from("/");

        assert!(is_filesystem_root(&root));
    }

    #[test]
    fn detects_non_root_path() {
        #[cfg(windows)]
        let non_root = PathBuf::from("C:\\Users");
        #[cfg(not(windows))]
        let non_root = PathBuf::from("/home");

        assert!(!is_filesystem_root(&non_root));
    }
}

#[cfg(test)]
mod parent_cache_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parent_cache_is_root_at_filesystem_root() {
        #[cfg(windows)]
        let root = PathBuf::from("C:\\");
        #[cfg(not(windows))]
        let root = PathBuf::from("/");

        let cache = fetch_parent_cache(&root);

        assert!(matches!(cache, ParentCache::Root));
    }

    #[test]
    fn navigation_state_at_filesystem_root_has_root_parent_snapshot() {
        #[cfg(windows)]
        let root = PathBuf::from("C:\\");
        #[cfg(not(windows))]
        let root = PathBuf::from("/");

        let nav = NavigationState::new(&root).unwrap();

        assert!(nav.is_at_root());
        assert!(matches!(nav.parent_snapshot(), ParentSnapshot::Root));
    }

    #[test]
    fn new_populates_parent_cache_with_siblings() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();
        fs::create_dir(dir.path().join("b")).unwrap();

        let nav = NavigationState::new(&dir.path().join("a")).unwrap();

        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(entries.len(), 2),
            _ => panic!("expected a Listed parent snapshot"),
        }
    }

    #[test]
    fn go_to_refreshes_parent_cache() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();
        fs::create_dir(dir.path().join("a").join("child")).unwrap();
        fs::create_dir(dir.path().join("b")).unwrap();

        let mut nav = NavigationState::new(&dir.path().join("a").join("child")).unwrap();
        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(entries.len(), 1),
            _ => panic!("expected a Listed parent snapshot"),
        }

        nav.go_to(&dir.path().join("b")).unwrap();

        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(
                entries.len(),
                2,
                "parent cache must reflect the new current directory's parent"
            ),
            _ => panic!("expected a Listed parent snapshot"),
        }
    }

    #[test]
    fn enter_selected_refreshes_parent_cache() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();
        fs::create_dir(dir.path().join("a").join("child")).unwrap();
        fs::create_dir(dir.path().join("b")).unwrap();

        let mut nav = NavigationState::new(dir.path()).unwrap();
        // entries() is sorted dirs-first alphabetically: ["a", "b"].
        nav.selected = 0;
        nav.enter_selected().unwrap();

        assert_eq!(nav.current_path(), dir.path().join("a"));
        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(
                entries.len(),
                2,
                "parent cache must reflect the new parent (the previous directory)"
            ),
            _ => panic!("expected a Listed parent snapshot"),
        }
    }

    #[test]
    fn go_to_parent_refreshes_parent_cache() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();
        fs::create_dir(dir.path().join("a").join("child")).unwrap();
        fs::create_dir(dir.path().join("b")).unwrap();

        let mut nav = NavigationState::new(&dir.path().join("a").join("child")).unwrap();
        nav.go_to_parent().unwrap();

        assert_eq!(nav.current_path(), dir.path().join("a"));
        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(
                entries.len(),
                2,
                "parent cache must reflect the grandparent's listing"
            ),
            _ => panic!("expected a Listed parent snapshot"),
        }
    }

    #[test]
    fn reload_dir_does_not_refresh_parent_cache() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();

        let mut nav = NavigationState::new(&dir.path().join("a")).unwrap();
        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(entries.len(), 1),
            _ => panic!("expected a Listed parent snapshot"),
        }

        // A sibling appears after the cache was populated.
        fs::create_dir(dir.path().join("b")).unwrap();
        nav.reload_dir().unwrap();

        match nav.parent_snapshot() {
            ParentSnapshot::Listed(entries) => assert_eq!(
                entries.len(),
                1,
                "reload_dir must not refresh the parent cache"
            ),
            _ => panic!("expected a Listed parent snapshot"),
        }
    }
}
