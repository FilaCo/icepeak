use iced::Task;

use crate::app::{Message, screen::home};

pub fn home_action_to_message(action: home::Action) -> Task<Message> {
    match action {
        home::Action::None => Task::none(),
        home::Action::Run(task) => task.map(Message::Home),
    }
}
