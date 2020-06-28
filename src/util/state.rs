use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self, increment: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - increment {
                    0
                } else {
                    i + increment
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, increment: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - increment
                } else {
                    i - increment
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}
