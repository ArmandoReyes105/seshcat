pub mod view;

use std::path::Path;

use crate::features::navigation::ParentSnapshot;
use crate::models::FsEntry;

/// Read-only, infallible view of the parent-directory pane. Built once per
/// render from an already-resolved `ParentSnapshot` - no IO happens here.
pub enum ParentView<'a> {
    /// The current directory has no parent (filesystem root).
    Root,
    /// The parent's entries, plus the index of the entry matching the
    /// current directory (if any), to be highlighted.
    Listed {
        entries: &'a [FsEntry],
        highlight: Option<usize>,
    },
    /// The parent directory could not be listed; carries the error message.
    Error(&'a str),
}

/// Returns the index of the entry in `entries` whose path exactly equals
/// `current_path`, or `None` if no entry matches.
///
/// Pure, no IO. Deliberately uses exact path equality: Windows path-casing
/// and symlink identity fragility are explicitly out of scope for this
/// feature (see design doc).
pub fn highlight_index(entries: &[FsEntry], current_path: &Path) -> Option<usize> {
    entries.iter().position(|entry| entry.path == current_path)
}

/// Maps an already-resolved `ParentSnapshot` into a `ParentView`, computing
/// the highlight index for the `Listed` case. Infallible - the IO already
/// happened when the snapshot was captured.
pub fn build_view<'a>(snapshot: ParentSnapshot<'a>, current_path: &Path) -> ParentView<'a> {
    match snapshot {
        ParentSnapshot::Root => ParentView::Root,
        ParentSnapshot::Error(message) => ParentView::Error(message),
        ParentSnapshot::Listed(entries) => ParentView::Listed {
            entries,
            highlight: highlight_index(entries, current_path),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fs_entry(name: &str, path: &str, is_dir: bool) -> FsEntry {
        FsEntry {
            name: name.to_string(),
            path: PathBuf::from(path),
            is_dir,
        }
    }

    #[test]
    fn highlight_index_matches_exact_path() {
        let entries = vec![
            fs_entry("a", "/root/a", true),
            fs_entry("b", "/root/b", true),
        ];

        assert_eq!(
            highlight_index(&entries, &PathBuf::from("/root/b")),
            Some(1)
        );
    }

    #[test]
    fn highlight_index_returns_none_when_no_entry_matches() {
        let entries = vec![fs_entry("a", "/root/a", true)];

        assert_eq!(
            highlight_index(&entries, &PathBuf::from("/root/other")),
            None
        );
    }

    #[test]
    fn highlight_index_returns_none_for_empty_entries() {
        let entries: Vec<FsEntry> = Vec::new();

        assert_eq!(highlight_index(&entries, &PathBuf::from("/root/a")), None);
    }

    #[test]
    fn build_view_root_snapshot_yields_root_view() {
        let view = build_view(ParentSnapshot::Root, Path::new("/anything"));

        assert!(matches!(view, ParentView::Root));
    }

    #[test]
    fn build_view_error_snapshot_yields_error_view() {
        let view = build_view(ParentSnapshot::Error("permission denied"), Path::new("/x"));

        match view {
            ParentView::Error(message) => assert_eq!(message, "permission denied"),
            _ => panic!("expected an Error view"),
        }
    }

    #[test]
    fn build_view_listed_snapshot_computes_highlight() {
        let entries = vec![
            fs_entry("a", "/root/a", true),
            fs_entry("b", "/root/b", true),
        ];
        let current = PathBuf::from("/root/b");

        let view = build_view(ParentSnapshot::Listed(&entries), &current);

        match view {
            ParentView::Listed {
                entries: got,
                highlight,
            } => {
                assert_eq!(got.len(), 2);
                assert_eq!(highlight, Some(1));
            }
            _ => panic!("expected a Listed view"),
        }
    }

    #[test]
    fn build_view_listed_snapshot_with_no_match_has_no_highlight() {
        let entries = vec![fs_entry("a", "/root/a", true)];
        let current = PathBuf::from("/root/somewhere-else");

        let view = build_view(ParentSnapshot::Listed(&entries), &current);

        match view {
            ParentView::Listed { highlight, .. } => assert_eq!(highlight, None),
            _ => panic!("expected a Listed view"),
        }
    }
}
