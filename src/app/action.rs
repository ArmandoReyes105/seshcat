#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    OpenSelected,
    OpenCurrentDir,
    EnterFavoritesLeader,
    EnterGotoLeader,
    StartRename,
    GoToConfig,
}
