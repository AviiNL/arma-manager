pub mod authenticated_base;
pub mod blank;
pub mod log;
pub mod login;
pub mod presets;
pub mod register;

pub use self::authenticated_base::*;
pub use self::blank::*;
pub use self::log::*;
pub use self::login::*;
pub use self::presets::*;
pub use self::register::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Login,
    Register,
    Home,
    Dashboard,
    Profile,
    Logs,
    Presets,
    Missions,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Login => "/",
            Self::Register => "/register",
            Self::Home => "/console",
            Self::Dashboard => "",
            Self::Profile => "profile",
            Self::Logs => "logs",
            Self::Presets => "presets",
            Self::Missions => "missions",
        }
    }
}
