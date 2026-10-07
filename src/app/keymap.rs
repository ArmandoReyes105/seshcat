use crossterm::event::KeyCode;

use super::action::Action;

pub fn normal_action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('o') => Some(Action::OpenSelected),
        KeyCode::Char('O') => Some(Action::OpenCurrentDir),
        KeyCode::Char('f') => Some(Action::EnterFavoritesLeader),
        KeyCode::Char('g') => Some(Action::EnterGotoLeader),
        KeyCode::Char('r') => Some(Action::StartRename),
        _ => None,
    }
}

pub fn goto_action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('c') => Some(Action::GoToConfig),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_o_opens_selected() {
        assert_eq!(
            normal_action(KeyCode::Char('o')),
            Some(Action::OpenSelected)
        );
    }

    #[test]
    fn normal_capital_o_opens_current_dir() {
        assert_eq!(
            normal_action(KeyCode::Char('O')),
            Some(Action::OpenCurrentDir)
        );
    }

    #[test]
    fn normal_f_enters_favorites_leader() {
        assert_eq!(
            normal_action(KeyCode::Char('f')),
            Some(Action::EnterFavoritesLeader)
        );
    }

    #[test]
    fn normal_g_enters_goto_leader() {
        assert_eq!(
            normal_action(KeyCode::Char('g')),
            Some(Action::EnterGotoLeader)
        );
    }

    #[test]
    fn normal_r_starts_rename() {
        assert_eq!(normal_action(KeyCode::Char('r')), Some(Action::StartRename));
    }

    #[test]
    fn normal_navigation_keys_fall_through() {
        assert_eq!(normal_action(KeyCode::Char('j')), None);
        assert_eq!(normal_action(KeyCode::Char('k')), None);
        assert_eq!(normal_action(KeyCode::Char('l')), None);
        assert_eq!(normal_action(KeyCode::Char('h')), None);
    }

    #[test]
    fn goto_c_goes_to_config() {
        assert_eq!(goto_action(KeyCode::Char('c')), Some(Action::GoToConfig));
    }

    #[test]
    fn goto_unknown_key_cancels() {
        assert_eq!(goto_action(KeyCode::Char('x')), None);
        assert_eq!(goto_action(KeyCode::Esc), None);
    }
}
