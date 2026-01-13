use iced::window;

use crate::app::screen::home;

#[derive(Debug, Clone)]
pub enum Message {
    WindowOpened(window::Id),
    Home(home::Message),
}
