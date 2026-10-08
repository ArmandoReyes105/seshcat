use crate::features::command::RunTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    OpenSelected,
    OpenCurrentDir,
    EnterFavoritesLeader,
    EnterGotoLeader,
    StartRename,
    GoToConfig,
    StartCommand(RunTarget),
}
