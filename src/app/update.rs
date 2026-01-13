use iced::Task;

use crate::app::{App, Message, action2message::home_action_to_message, screen::Screen};

impl App {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::WindowOpened(id) => todo!(),
            Message::Home(home_msg) => {
                if let Screen::Home(home) = &mut self.screen {
                    return home_action_to_message(home.update(home_msg));
                }
            }
        }

        Task::none()
    }
}
