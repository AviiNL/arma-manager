pub mod authenticated_base;
pub mod blank;
pub mod log;
pub mod login;
pub mod register;

pub use self::{authenticated_base::*, blank::*, login::*, register::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Profile,
    Logs,
    Mods,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/console",
            Self::Login => "/",
            Self::Register => "/register",
            Self::Profile => "/profile",
            Self::Logs => "/logs",
            Self::Mods => "/mods",
        }
    }
}
