use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PlayerOrInfo {
    Players(Vec<Player>),
    Info(Info),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TheShip {
    /// Indicates the game mode
    pub mode: TheShipMode,

    /// The number of witnesses necessary to have a player arrested.
    pub witnesses: u8,

    /// Time (in seconds) before a player is arrested while being witnessed.
    pub duration: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[repr(u8)]
pub enum TheShipMode {
    Hunt = 0,
    Elimination = 1,
    Duel = 2,
    Deathmatch = 3,
    VIPTeam = 4,
    TeamElimination = 5,
    Unknown = 255,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtendedServerInfo {
    /// The server's game port number.
    /// Available if edf & 0x80 is true
    pub port: Option<u16>,

    /// Server's SteamID.
    /// Available if edf & 0x10 is true
    pub steam_id: Option<u64>,

    /// Tags that describe the game according to the server (for future use.)
    /// Available if edf & 0x20 is true
    pub keywords: Option<String>,

    /// The server's 64-bit GameID. If this is present, a more accurate AppID is present in the low 24 bits.
    /// The earlier AppID could have been truncated as it was forced into 16-bit storage.
    /// Avaialble if edf & 0x01 is true
    pub game_id: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceTVInfo {
    /// Spectator port number for SourceTV.
    pub port: u16,

    /// Name of the spectator server for SourceTV.
    pub name: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[repr(u8)]
pub enum ServerType {
    Dedicated = b'd',
    NonDedicated = b'i',
    SourceTV = b'p',
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServerType::Dedicated => "Dedicated",
            ServerType::NonDedicated => "Non-Dedicated",
            ServerType::SourceTV => "SourceTV",
        };

        write!(f, "{}", value.to_string())
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[repr(u8)]
pub enum ServerOS {
    Linux = b'l',
    Windows = b'w',
    Mac = b'm',
}

impl std::fmt::Display for ServerOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ServerOS::Linux => "Linux",
            ServerOS::Windows => "Windows",
            ServerOS::Mac => "Mac",
        };

        write!(f, "{}", value.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    /// Protocol version used by the server.
    pub protocol: u8,

    /// Name of the server.
    pub name: String,

    /// Map the server has currently loaded.
    pub map: String,

    /// Name of the folder containing the game files.
    pub folder: String,

    /// Full name of the game.
    pub game: String,

    /// Steam Application ID of game.
    pub app_id: u16,

    /// Number of players on the server.
    pub players: u8,

    /// Maximum number of players the server reports it can hold.
    pub max_players: u8,

    /// Number of bots on the server.
    pub bots: u8,

    /// Indicates the type of server
    /// Rag Doll Kung Fu servers always return 0 for "Server type."
    pub server_type: ServerType,

    /// Indicates the operating system of the server
    pub server_os: ServerOS,

    /// Indicates whether the server requires a password
    pub visibility: bool,

    /// Specifies whether the server uses VAC
    pub vac: bool,

    /// These fields only exist in a response if the server is running The Ship
    pub the_ship: Option<TheShip>,

    /// Version of the game installed on the server.
    pub version: String,

    /// If present, this specifies which additional data fields will be included.
    pub edf: u8,

    pub extended_server_info: ExtendedServerInfo,

    /// Available if edf & 0x40 is true
    pub source_tv: Option<SourceTVInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    /// Index of player chunk starting from 0.
    /// This seems to be always 0?
    pub index: u8,

    /// Name of the player.
    pub name: String,

    /// Player's score (usually "frags" or "kills".)
    pub score: i32,

    /// Time (in seconds) player has been connected to the server.
    pub duration: f32,

    /// The Ship additional player info
    pub the_ship: Option<TheShipPlayer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TheShipPlayer {
    pub deaths: u32,

    pub money: u32,
}
