use yew::{Html, html};

const SUCCESS: &str = "#4dff4d";
const WARNING: &str = "#ffe400";
const FAILURE: &str = "#ff5050";
const INITIAL: &str = "#ffffff";

enum Status {
    Success,
    Warning,
    Failure,
    Initial,
}

impl Status {
    const fn as_str(&self) -> &'static str {
        match self {
            Status::Success => SUCCESS,
            Status::Warning => WARNING,
            Status::Failure => FAILURE,
            Status::Initial => INITIAL,
        }
    }
}

#[derive(PartialEq)]
pub enum StatusKind {
    Initial,
    Connection,
    ServerDown,
    PlayerNotFound,
    Whitelisted,
    Success,
    InvalidInput,
    Connecting,
    IncorrectPassword,
    RateLimited,
    Unexpected,
}

impl StatusKind {
    pub const fn from_u8(byte: &u8) -> Self {
        match byte {
            0 => Self::ServerDown,
            1 => Self::PlayerNotFound,
            2 => Self::Whitelisted,
            3 => Self::Success,
            4 => Self::IncorrectPassword,
            5 => Self::RateLimited,
            _ => Self::Unexpected,
        }
    }

    const fn status(&self) -> Status {
        match self {
            Self::Success | Self::Whitelisted => Status::Success,
            Self::Connection
            | Self::ServerDown
            | Self::Unexpected
            | Self::PlayerNotFound
            | Self::InvalidInput => Status::Warning,
            Self::IncorrectPassword | Self::RateLimited => Status::Failure,
            Self::Connecting | Self::Initial => Status::Initial,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Connection => "Failed to connect to server",
            Self::ServerDown => "Minecraft server is down",
            Self::PlayerNotFound => "Player doesn't exist",
            Self::Whitelisted => "Already whitelisted",
            Self::Success => "Success",
            Self::InvalidInput => "Invalid input",
            Self::Connecting => "Connecting...",
            Self::IncorrectPassword => "Incorrect password",
            Self::Unexpected => "Unexpected server response",
            Self::RateLimited => "Whoa there! Too many attempts.\nPlease wait a bit and try again",
            _ => unreachable!(),
        }
    }

    pub const fn is_new(&self) -> bool {
        !matches!(self, Self::Initial)
    }

    pub fn as_html(&self) -> Html {
        html! { <p style={ format!("font-size: large; color: {}", self.status().as_str()) } > { self.as_str() } </p> }
    }
}
