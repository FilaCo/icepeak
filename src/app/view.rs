use iced::{Element, window};

use crate::app::{App, Message, screen::Screen};

impl App {
    pub fn view(&self, _: window::Id) -> Element<'_, Message> {
        match &self.screen {
            Screen::Loading => todo!(),
            Screen::Home(home) => home.view().map(Message::Home),
        }
    }
}
