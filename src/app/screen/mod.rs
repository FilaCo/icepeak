pub mod home;

#[derive(Debug)]
pub enum Screen {
    Loading,
    Home(home::Home),
}
