use iced::{Task, window};

use crate::app::{Message, screen::Screen};

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(window::Settings::default());
        (
            Self {
                screen: Screen::Loading,
            },
            Task::batch(vec![open.map(Message::WindowOpened)]),
        )
    }
}
