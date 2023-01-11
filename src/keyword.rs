//! # Keyword module
//!
//! This module is used for setting, and getting keywords
//!
//! ## Usage
//!
//! ```rust, no_run
//! use hyprland::shared::HResult;
//! use hyprland::keyword::Keyword;
//! fn main() -> HResult<()> {
//!    Keyword::get("some_keyword")?;
//!    Keyword::set("another_keyword", "the value to set it to")?;
//!
//!    Ok(())
//! }
//! ````

use crate::shared::*;
use num_traits::AsPrimitive;
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

impl From<OptionValue> for String {
    fn from(opt: OptionValue) -> Self {
        opt.to_string()
    }
}

impl From<String> for OptionValue {
    fn from(str: String) -> Self {
        OptionValue::String(str)
    }
}

impl From<&str> for OptionValue {
    fn from(str: &str) -> Self {
        OptionValue::String(str.to_string())
    }
}

macro_rules! int_to_opt {
    ($ty:ty) => {
        impl From<$ty> for OptionValue {
            fn from(num: $ty) -> Self {
                OptionValue::Int(num.as_())
            }
        }
    };
}

macro_rules! ints_to_opt {
    ( $( $t:ty ),* ) => {
        $(int_to_opt!($t);)*
    };
}

ints_to_opt!(u8, i8, u16, i16, u32, i32, u64, i64);

macro_rules! float_to_opt {
    ($ty:ty) => {
        impl From<$ty> for OptionValue {
            fn from(num: $ty) -> Self {
                OptionValue::Float(num.as_())
            }
        }
    };
}

float_to_opt!(f32);
float_to_opt!(f64);

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
    pub fn set<Str: ToString, Opt: Into<OptionValue>>(key: Str, value: Opt) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let key = key.to_string();
        let _ = write_to_socket_sync(
            socket_path,
            keyword!(key, (value.into().to_string())).as_bytes(),
        )?;
        Ok(())
    }
    /// This function sets a keyword's value (async)
    pub async fn set_async<Str: ToString, Opt: Into<OptionValue>>(
        key: Str,
        value: Opt,
    ) -> HResult<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let key = key.to_string();
        let _ = write_to_socket(
            socket_path,
            keyword!(key, (value.into().to_string())).as_bytes(),
        )
        .await?;
        Ok(())
    }
    /// This function returns the value of a keyword
    pub fn get<Str: ToString>(key: Str) -> HResult<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let key = key.to_string();
        let data = write_to_socket_sync(socket_path, keyword!(g key).as_bytes())?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let dc = deserialized.clone();
        let keyword = Keyword {
            option: deserialized.option,
            value: if deserialized.int.is_positive() {
                OptionValue::Int(deserialized.int)
            } else if deserialized.float.is_sign_positive() {
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
    pub async fn get_async<Str: ToString>(key: Str) -> HResult<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let key = key.to_string();
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
