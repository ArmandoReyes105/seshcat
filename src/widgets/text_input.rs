use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct TextInput {
    value: String,
    cursor: usize,
}

impl TextInput {
    pub fn new(initial: &str) -> Self {
        Self {
            value: initial.to_string(),
            cursor: initial.len(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn insert(&mut self, c: char) {
        self.value.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn delete_back(&mut self) {
        if self.cursor == 0 {
            return;
        }

        if let Some((prev, _)) = self.value[..self.cursor].grapheme_indices(true).next_back() {
            self.value.replace_range(prev..self.cursor, "");
            self.cursor = prev;
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor >= self.value.len() {
            return;
        }

        if let Some(grapheme) = self.value[self.cursor..].graphemes(true).next() {
            let end = self.cursor + grapheme.len();
            self.value.replace_range(self.cursor..end, "");
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        if let Some((prev, _)) = self.value[..self.cursor].grapheme_indices(true).next_back() {
            self.cursor = prev;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor >= self.value.len() {
            return;
        }

        if let Some(grapheme) = self.value[self.cursor..].graphemes(true).next() {
            self.cursor += grapheme.len();
        }
    }

    pub fn end(&mut self) {
        self.cursor = self.value.len();
    }

    pub fn cursor_display_col(&self) -> usize {
        self.value[..self.cursor].chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cursor_at_end() {
        let input = TextInput::new("año");

        assert_eq!(input.cursor, 4);
        assert_eq!(input.value(), "año");
    }

    #[test]
    fn empty_cursor_at_zero() {
        let input = TextInput::default();
        assert_eq!(input.cursor, 0);
        assert_eq!(input.value(), "");
    }

    #[test]
    fn insert_ascii() {
        let mut input = TextInput::default();
        input.insert('a');
        assert_eq!(input.value(), "a");
        assert_eq!(input.cursor, 1);
    }

    #[test]
    fn insert_multibyte() {
        let mut input = TextInput::default();
        input.insert('ñ');
        assert_eq!(input.value(), "ñ");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn insert_emoji() {
        let mut input = TextInput::default();
        input.insert('🦀');
        assert_eq!(input.value(), "🦀");
        assert_eq!(input.cursor, 4);
    }

    #[test]
    fn insert_in_middle() {
        let mut input = TextInput::new("hello");
        input.home();
        input.insert('!');
        assert_eq!(input.value(), "!hello");
        assert_eq!(input.cursor, 1);
    }

    #[test]
    fn delete_back_ascii() {
        let mut input = TextInput::new("hello");
        input.delete_back();
        assert_eq!(input.value(), "hell");
        assert_eq!(input.cursor, 4);
    }

    #[test]
    fn delete_back_multibyte_char() {
        let mut input = TextInput::new("año");
        input.delete_back();
        assert_eq!(input.value(), "añ");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn delete_back_emoji_grapheme_stays_complete() {
        let mut input = TextInput::new("🦀");
        assert_eq!(input.cursor, 4);
        input.delete_back();

        assert_eq!(input.value(), "");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn delete_back_at_start_does_nothing() {
        let mut input = TextInput::new("hello");
        input.home();
        input.delete_back();
        assert_eq!(input.value(), "hello");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn delete_forward_from_start() {
        let mut input = TextInput::new("hello");
        input.home();
        input.delete_forward();
        assert_eq!(input.value(), "ello");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn delete_forward_emoji() {
        let mut input = TextInput::new("🦀");
        input.home();
        input.delete_forward();
        assert_eq!(input.value(), "");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn move_left_over_multibyte() {
        let mut input = TextInput::new("año");
        input.move_left();
        assert_eq!(input.cursor, 3);
        input.move_left();
        assert_eq!(input.cursor, 1);
        input.move_left();
        assert_eq!(input.cursor, 0);
        input.move_left();
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn move_left_treats_emoji_as_one_grapheme() {
        let mut input = TextInput::new("🦀");
        input.move_left();
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn move_right_over_multibyte() {
        let mut input = TextInput::new("año");
        input.home();
        input.move_right();
        assert_eq!(input.cursor, 1);
        input.move_right();
        assert_eq!(input.cursor, 3);
        input.move_right();
        assert_eq!(input.cursor, 4);
        input.move_right();
        assert_eq!(input.cursor, 4);
    }

    #[test]
    fn home_and_end() {
        let mut input = TextInput::new("hello");
        input.home();
        assert_eq!(input.cursor, 0);
        input.end();
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn cursor_col_ascii() {
        let mut input = TextInput::new("hello");
        input.home();
        assert_eq!(input.cursor_display_col(), 0);
        input.move_right();
        assert_eq!(input.cursor_display_col(), 1);
    }

    #[test]
    fn cursor_col_multibyte_char_is_one_column() {
        let input = TextInput::new("ñ");
        assert_eq!(input.cursor_display_col(), 1);
    }
}
