use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Key {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Tab,
    Delete,
    Insert,
    F(u8),
    Alt(char),
    Ctrl(char),
    Char(char),
    Esc,
    Unused,
}

impl From<KeyEvent> for Key {
    #[rustfmt::skip]
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            KeyEvent {
                code: KeyCode::Char(char),
                modifiers: KeyModifiers::NONE
            } => Key::Char(char),
            KeyEvent {
                code: KeyCode::Char(char),
                modifiers: KeyModifiers::ALT
            } => Key::Alt(char),
            KeyEvent {
                code: KeyCode::Char(char),
                modifiers: KeyModifiers::CONTROL
            } => Key::Ctrl(char),
            KeyEvent { code: KeyCode::Backspace,  .. } => Key::Backspace,
            KeyEvent { code: KeyCode::Enter,      .. } => Key::Enter,
            KeyEvent { code: KeyCode::Left,       .. } => Key::Left,
            KeyEvent { code: KeyCode::Right,      .. } => Key::Right,
            KeyEvent { code: KeyCode::Up,         .. } => Key::Up,
            KeyEvent { code: KeyCode::Down,       .. } => Key::Down,
            KeyEvent { code: KeyCode::Tab,        .. } => Key::Tab,
            KeyEvent { code: KeyCode::Delete,     .. } => Key::Delete,
            KeyEvent { code: KeyCode::Insert,     .. } => Key::Insert,
            KeyEvent { code: KeyCode::F(u8),      .. } => Key::F(u8),
            KeyEvent { code: KeyCode::Char(char), .. } => Key::Char(char),
            KeyEvent { code: KeyCode::Esc,        .. } => Key::Esc,
            _ => Key::Unused,
        }
    }
}
