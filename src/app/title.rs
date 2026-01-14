use iced::window;

use crate::app::{App, screen::Screen};

impl App {
    pub fn title(&self, _: window::Id) -> String {
        match &self.screen {
            Screen::Loading => String::from("IcePeak"),
            Screen::Home(home) => home.title(),
        }
    }
}
