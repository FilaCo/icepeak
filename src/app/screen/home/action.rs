use iced::Task;

use crate::app::screen::home::Message;

pub enum Action {
    None,
    Run(Task<Message>),
}
