pub mod authenticated_base;
pub mod blank;
pub mod config;
pub mod log;
pub mod login;
pub mod missions;
pub mod presets;
pub mod profile;
pub mod register;

pub use authenticated_base::*;
pub use blank::*;
pub use config::*;
pub use log::*;
pub use login::*;
pub use missions::*;
pub use presets::*;
pub use profile::*;
pub use register::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Login,
    Register,
    Home,
    Dashboard,
    Profile,
    Logs,
    Config,
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
            Self::Config => "config",
            Self::Presets => "presets",
            Self::Missions => "missions",
        }
    }
}
