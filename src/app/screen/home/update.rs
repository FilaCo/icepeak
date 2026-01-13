use iced::Element;

use crate::app::screen::home::{Action, Home, Message};

impl Home {
    pub fn update(&mut self, msg: Message) -> Action {
        Action::None
    }
}
