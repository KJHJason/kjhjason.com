use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub enum State {
    On,
    Off,
}

impl State {
    pub fn get_state(&self) -> bool {
        match self {
            State::On => true,
            State::Off => false,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            State::On => "on".to_string(),
            State::Off => "off".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            State::On => "on",
            State::Off => "off",
        };
        serializer.serialize_str(s)
    }
}

// deserialize the State enum from a string
impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<State, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "on" => Ok(State::On),
            "off" => Ok(State::Off),
            _ => Err(serde::de::Error::custom("invalid state")),
        }
    }
}
