use iced::Element;

use crate::app::screen::home::{Home, Message};

impl Home {
    pub fn title(&self) -> String {
        String::from("IcePeak - Home")
    }
}
