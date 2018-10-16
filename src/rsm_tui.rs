use std::io;
//use std::boxed::FnBox;

extern crate termion;
extern crate tui;
extern crate failure;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;

// 默认terminal
type TermionTerm = Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>>;

pub enum InteractiveMode {
    MAIN,
    SEARCH,
    DELETE,
    ADD,
    UPDDATE,
}

pub struct App {
    mode: InteractiveMode,
    size: Rect,
    terminal: TermionTerm,
}

impl Default for App {
    fn default() -> App {
        App {
            mode: InteractiveMode::MAIN,
            size: Rect::default(),
            terminal: App::init().unwrap(),
        }
    }
}

impl App {
    pub fn init() -> Result<TermionTerm, failure::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(terminal)
    }

    // 主ui
    pub fn main_ui(&mut self) {
        self.terminal.draw(|mut f|{

        });
    }

    // 搜索ui
    pub fn search_ui(&self) -> Box<FnOnce(TermionTerm)> {
        Box::new(|mut f: TermionTerm| {})
    }

    // 删除ui
    pub fn delete_ui(&self) {

    }

    // 添加ui
    pub fn add_ui(&self) {

    }

    // 更新ui
    pub fn update_ui(&self) {

    }

    // key event
    pub fn key_event() {

    }
}

