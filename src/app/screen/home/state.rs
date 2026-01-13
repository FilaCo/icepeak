use crate::app::screen::home::Action;

#[derive(Debug)]
pub struct Home {}

impl Home {
    pub fn new() -> (Self, Action) {
        (Self {}, Action::None)
    }
}
