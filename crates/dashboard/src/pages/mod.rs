pub mod blank;
pub mod home;
pub mod layout;
pub mod log;
pub mod login;
pub mod register;

pub use self::{blank::*, home::*, layout::*, login::*, register::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Profile,
    Logs,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::Profile => "/profile",
            Self::Logs => "/logs",
        }
    }
}
