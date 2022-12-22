use crate::shared::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OptionRaw {
    pub option: String,
    pub int: i64,
    pub float: f64,
    pub str: String,
}

/// This enum holds the possible values of a keyword/option
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OptionValue {
    /// A integer (64-bit)
    Int(i64),
    /// A floating point (64-point)
    Float(f64),
    /// A string
    String(String),
}

impl ToString for OptionValue {
    fn to_string(&self) -> String {
        match self {
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::String(v) => v.to_string(),
        }
    }
}

/// This struct holds a keyword
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    /// The identifier (or name) of the keyword
    pub option: String,
    /// The value of the keyword/option
    pub value: OptionValue,
}

macro_rules! keyword {
    ($k:tt,$v:tt) => {
        format!("keyword {} {}", $k, $v)
    };
    (g $l:tt) => {
        format!("j/getoption {}", $l)
    };
}

impl Keyword {
    /// This function sets a keyword's value
    pub fn set(key: String, value: OptionValue) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let _ = write_to_socket_sync(socket_path, keyword!(key, (value.to_string())).as_bytes())?;
        Ok(())
    }
    /// This function sets a keyword's value (async)
    pub async fn set_async(key: String, value: OptionValue) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let _ = write_to_socket(socket_path, keyword!(key, (value.to_string())).as_bytes()).await?;
        Ok(())
    }
    /// This function returns the value of a keyword
    pub fn get(key: String) -> HResult<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let data = write_to_socket_sync(socket_path, keyword!(g key).as_bytes())?;
        dbg!(&data);
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let dc = deserialized.clone();
        let keyword = Keyword {
            option: deserialized.option,
            value: if deserialized.int != -1 {
                OptionValue::Int(deserialized.int)
            } else if deserialized.float != -1.0 {
                OptionValue::Float(deserialized.float)
            } else if deserialized.str != *"".to_string() {
                OptionValue::String(deserialized.str)
            } else {
                panic!("The option returned data that was unrecognized: {dc:#?}")
            },
        };
        Ok(keyword)
    }
    /// This function returns the value of a keyword (async)
    pub async fn get_async(key: String) -> HResult<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let data = write_to_socket(socket_path, keyword!(g key).as_bytes()).await?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let dc = deserialized.clone();
        let keyword = Keyword {
            option: deserialized.option,
            value: if deserialized.int != -1 {
                OptionValue::Int(deserialized.int)
            } else if deserialized.float != -1.0 {
                OptionValue::Float(deserialized.float)
            } else if deserialized.str != *"".to_string() {
                OptionValue::String(deserialized.str)
            } else {
                panic!("The option returned data that was unrecognized: {dc:#?}")
            },
        };
        Ok(keyword)
    }
}
