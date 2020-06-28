use crate::request::{Endpoint, Request};
use crate::util::StatefulList;

pub enum Mode {
    Search,
    Browse,
    Read,
}

pub struct App<'a> {
    pub title: &'a str,
    pub mode: Mode,
    pub search_input: String,
    pub search_input_last: String,
    pub search_cursor_x_max: u16,
    pub search_cursor_x: u16,
    pub search_cursor_y: u16,
    pub pages: StatefulList<String>,
    pub toc: Vec<String>,
    pub page: Vec<String>,
    pub page_scroll: u16,
    pub url: String,
}

const URL: &str = "https://en.wikipedia.org";

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            mode: Mode::Search,
            search_input: String::default(),
            search_input_last: String::default(),
            search_cursor_x_max: 0,
            search_cursor_x: 0,
            search_cursor_y: 0,
            pages: StatefulList::default(),
            toc: Vec::new(),
            page: Vec::new(),
            page_scroll: 0,
            url: URL.to_owned(),
        }
    }

    pub fn on_key(&mut self, key: char) {
        if let Mode::Search = self.mode {
            if self.search_input.len() < self.search_cursor_x_max as usize {
                self.search_input.push(key);
            }
        }
    }

    pub fn on_backspace(&mut self) {
        if let Mode::Search = self.mode {
            self.search_input.pop();
        }
    }

    pub fn on_enter(&mut self) {
        match self.mode {
            Mode::Search => {
                if self.search_input == "" {
                    return;
                }
                if self.search_input == self.search_input_last {
                    self.mode = Mode::Browse;
                    return;
                }
                let ps = Request::new(&self.search_input, Endpoint::PrefixSearch).fetch();
                self.mode = Mode::Browse;
                self.search_input_last = self.search_input.clone();
                self.pages = StatefulList::with_items(ps.page_list);
            }
            Mode::Browse => {
                if let Some(i) = self.pages.state.selected() {
                    let page = &self.pages.items[i];
                    let ps = Request::new(page, Endpoint::PageParse).fetch();
                    self.toc = ps.toc_list;
                    self.mode = Mode::Read;
                    self.page = ps.page;
                    self.url = format!("{}/wiki/{}", URL, page.as_str().replace(" ", "_"));
                }
            }
            Mode::Read => {}
        }
    }

    pub fn on_escape(&mut self) {
        match self.mode {
            Mode::Search => {}
            Mode::Browse => self.mode = Mode::Search,
            Mode::Read => self.mode = Mode::Browse,
        }
    }

    pub fn on_up(&mut self) {
        match self.mode {
            Mode::Search => {}
            Mode::Browse => self.pages.previous(1),
            Mode::Read => self.scroll_up(1),
        }
    }

    pub fn on_down(&mut self) {
        match self.mode {
            Mode::Search => {}
            Mode::Browse => self.pages.next(1),
            Mode::Read => self.scroll_down(1),
        }
    }

    pub fn on_left(&mut self) {
        match self.mode {
            Mode::Search => {}
            Mode::Browse => self.pages.previous(5),
            Mode::Read => self.scroll_up(5),
        }
    }

    pub fn on_right(&mut self) {
        match self.mode {
            Mode::Search => {}
            Mode::Browse => self.pages.next(5),
            Mode::Read => self.scroll_down(5),
        }
    }

    fn scroll_up(&mut self, increment: u16) {
        self.page_scroll += if self.page_scroll > 0 { increment } else { 0 }
    }

    fn scroll_down(&mut self, increment: u16) {
        self.page_scroll -= if self.page_scroll < self.page.len() as u16 {
            increment
        } else {
            0
        }
    }
}
