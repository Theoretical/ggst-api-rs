pub mod error;
pub mod requests;

use chrono::prelude::*;
use error::*;
// Reexport the functions and structs from requests.rs
pub use requests::*;
use std::collections::HashMap;
use std::fmt;

/// Player information associated with a match
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Player {
    id: String,
    name: String,
    character: Character,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}) as {}", self.name, self.id, self.character)
    }
}

/// Indicates which player won a match
#[derive(Hash, PartialEq, Eq, Debug)]
enum Winner {
    Player1,
    Player2,
}

/// A match received by the get_replay API
/// Use requests::get_replays() to query for replays to get a set of this struct
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Match {
    floor: Floor,
    timestamp: DateTime<Utc>,
    players: (Player, Player),
    winner: Winner,
}

impl Match {
    /// Get the player information about the winner
    pub fn winner(&self) -> &Player {
        match self.winner {
            Winner::Player1 => &self.players.0,
            Winner::Player2 => &self.players.1,
        }
    }

    /// Get the player information about the winner
    pub fn loser(&self) -> &Player {
        match self.winner {
            Winner::Player1 => &self.players.1,
            Winner::Player2 => &self.players.0,
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match {} on floor {:?} {{\n  Winner: {}\n  Loser: {}\n}}",
            self.timestamp,
            self.floor,
            self.winner(),
            self.loser()
        )
    }
}

/// Enum for characters in the game
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Character {
    Sol,
    Ky,
    May,
    Axl,
    Chipp,
    Potemkin,
    Faust,
    Millia,
    Zato,
    Ramlethal,
    Leo,
    Nagoriyuki,
    Giovanna,
    Anji,
    Ino,
    Goldlewis,
    Jacko,
    HappyChaos,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Character::Sol => write!(f, "Sol Badguy"),
            Character::Ky => write!(f, "Ky Kiske"),
            Character::May => write!(f, "May"),
            Character::Axl => write!(f, "Axl Low"),
            Character::Leo => write!(f, "Leo Whitefang"),
            Character::Ino => write!(f, "I-no"),
            Character::Zato => write!(f, "Zato=1"),
            Character::Anji => write!(f, "Anji Mito"),
            Character::Chipp => write!(f, "Chipp Zanuff"),
            Character::Faust => write!(f, "Faust"),
            Character::Potemkin => write!(f, "Potemkin"),
            Character::Millia => write!(f, "Millia Rage"),
            Character::Ramlethal => write!(f, "Ramlethal Valentine"),
            Character::Giovanna => write!(f, "Giovanna"),
            Character::Nagoriyuki => write!(f, "Nagoriyuki"),
            Character::Goldlewis => write!(f, "Goldlewis Dickinson"),
            Character::Jacko => write!(f, "Jack-o"),
            Character::HappyChaos => write!(f, "Happy Chaos"),
        }
    }
}

impl Character {
    /// Convert a byte into a Character enum.
    /// 00: Sol 01: Ky 02: May 03: Axl 04: Chipp 05: Pot 06: Faust 07: Millia
    /// 08: Zato-1 09: Ram 0a: Leo 0b: Nago 0c: Gio 0d: Anji 0e: I-No 0f: Goldlewis 10: Jack-O
    ///
    /// See https://github.com/optix2000/totsugeki/issues/35#issuecomment-922516535
    pub fn from_u8(c: u8) -> Result<Self> {
        match c {
            0x00 => Ok(Character::Sol),
            0x01 => Ok(Character::Ky),
            0x02 => Ok(Character::May),
            0x03 => Ok(Character::Axl),
            0x04 => Ok(Character::Chipp),
            0x05 => Ok(Character::Potemkin),
            0x06 => Ok(Character::Faust),
            0x07 => Ok(Character::Millia),
            0x08 => Ok(Character::Zato),
            0x09 => Ok(Character::Ramlethal),
            0x0a => Ok(Character::Leo),
            0x0b => Ok(Character::Nagoriyuki),
            0x0c => Ok(Character::Giovanna),
            0x0d => Ok(Character::Anji),
            0x0e => Ok(Character::Ino),
            0x0f => Ok(Character::Goldlewis),
            0x10 => Ok(Character::Jacko),
            0x11 => Ok(Character::HappyChaos),
            _ => Err(Error::InvalidArguments(format!(
                "{:x} is not a valid character code",
                c
            ))),
        }
    }

    /// Convert a Character back to its u8 code
    /// 00: Sol 01: Ky 02: May 03: Axl 04: Chipp 05: Pot 06: Faust 07: Millia
    /// 08: Zato-1 09: Ram 0a: Leo 0b: Nago 0c: Gio 0d: Anji 0e: I-No 0f: Goldlewis 10: Jack-O
    ///
    /// See https://github.com/optix2000/totsugeki/issues/35#issuecomment-922516535
    pub fn to_u8(&self) -> u8 {
        match self {
            Character::Sol => 0x00,
            Character::Ky => 0x01,
            Character::May => 0x02,
            Character::Axl => 0x03,
            Character::Chipp => 0x04,
            Character::Potemkin => 0x05,
            Character::Faust => 0x06,
            Character::Millia => 0x07,
            Character::Zato => 0x08,
            Character::Ramlethal => 0x09,
            Character::Leo => 0x0a,
            Character::Nagoriyuki => 0x0b,
            Character::Giovanna => 0x0c,
            Character::Anji => 0x0d,
            Character::Ino => 0x0e,
            Character::Goldlewis => 0x0f,
            Character::Jacko => 0x10,
            Character::HappyChaos => 0x11,
        }
    }

    /// Convert the character enum to the code used by the profile API
    fn to_code(&self) -> &'static str {
        match self {
            Character::Sol => "SOL",
            Character::Ky => "KYK",
            Character::May => "MAY",
            Character::Axl => "AXL",
            Character::Leo => "LEO",
            Character::Ino => "INO",
            Character::Zato => "ZAT",
            Character::Anji => "ANJ",
            Character::Chipp => "CHP",
            Character::Faust => "FAU",
            Character::Potemkin => "POT",
            Character::Millia => "MLL",
            Character::Ramlethal => "RAM",
            Character::Giovanna => "GIO",
            Character::Nagoriyuki => "NAG",
            Character::Goldlewis => "GLD",
            Character::Jacko => "JKO",
            Character::HappyChaos => "COS",
        }
    }

    /// Convert back to the character enum based on the profile API code representation of it
    fn from_code(code: &str) -> Result<Character> {
        match code {
            "SOL" => Ok(Character::Sol),
            "KYK" => Ok(Character::Ky),
            "MAY" => Ok(Character::May),
            "AXL" => Ok(Character::Axl),
            "LEO" => Ok(Character::Leo),
            "INO" => Ok(Character::Ino),
            "ZAT" => Ok(Character::Zato),
            "ANJ" => Ok(Character::Anji),
            "CHP" => Ok(Character::Chipp),
            "FAU" => Ok(Character::Faust),
            "POT" => Ok(Character::Potemkin),
            "MLL" => Ok(Character::Millia),
            "RAM" => Ok(Character::Ramlethal),
            "GIO" => Ok(Character::Giovanna),
            "NAG" => Ok(Character::Nagoriyuki),
            "GLD" => Ok(Character::Goldlewis),
            "JKO" => Ok(Character::Jacko),
            "COS" => Ok(Character::HappyChaos),
            _ => Err(Error::InvalidCharacterCode(code.into())),
        }
    }
}

/// Enum mapping for floors present in the game
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Floor {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    Celestial,
}

impl Floor {
    /// Create a floor from a byte representation
    ///
    /// See https://github.com/optix2000/totsugeki/issues/35#issuecomment-922516535 for mapping
    fn from_u8(c: u8) -> Result<Self> {
        match c {
            0x00 => Ok(Floor::F1),
            0x01 => Ok(Floor::F2),
            0x02 => Ok(Floor::F3),
            0x03 => Ok(Floor::F4),
            0x04 => Ok(Floor::F5),
            0x05 => Ok(Floor::F6),
            0x06 => Ok(Floor::F7),
            0x07 => Ok(Floor::F8),
            0x08 => Ok(Floor::F9),
            0x09 => Ok(Floor::F10),
            0x63 => Ok(Floor::Celestial),
            _ => Err(Error::InvalidArguments(format!(
                "{:x} is not a valid floor code",
                c
            ))),
        }
    }

    /// Similar to to_u8() but it directly returns its string representation for url building
    fn to_hex(&self) -> String {
        match self {
            Floor::F1 => "00".into(),
            Floor::F2 => "01".into(),
            Floor::F3 => "02".into(),
            Floor::F4 => "03".into(),
            Floor::F5 => "04".into(),
            Floor::F6 => "05".into(),
            Floor::F7 => "06".into(),
            Floor::F8 => "07".into(),
            Floor::F9 => "08".into(),
            Floor::F10 => "0a".into(),
            Floor::Celestial => "63".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MatchStats {
    total: usize,
    wins: usize,
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    level: usize,
    wins: usize,
}

#[derive(Clone, Debug)]
pub struct User {
    user_id: String,
    name: String,
    comment: String,
    floor: Floor,
    stats: MatchStats,
    celestial_stats: MatchStats,
    char_stats: HashMap<Character, Stats>,
}