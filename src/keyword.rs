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
//! ```

use crate::shared::*;
use derive_more::Display;
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
#[derive(Serialize, Deserialize, Debug, Clone, Display)]
pub enum OptionValue {
    /// A integer (64-bit)
    #[display(fmt = "{}", "_0")]
    Int(i64),
    /// A floating point (64-point)
    #[display(fmt = "{}", "_0")]
    Float(f64),
    /// A string
    #[display(fmt = "{}", "_0")]
    String(String),
}

impl From<OptionValue> for String {
    fn from(opt: OptionValue) -> Self {
        opt.to_string()
    }
}

trait IsString {}
impl IsString for String {}
impl IsString for &str {}

impl<Str: ToString + IsString> From<Str> for OptionValue {
    fn from(str: Str) -> Self {
        OptionValue::String(str.to_string())
    }
}

macro_rules! ints_to_opt {
    ($($ty:ty), *) => {
        $(
            impl From<$ty> for OptionValue {
                fn from(num: $ty) -> Self {
                    OptionValue::Int(num.as_())
                }
            }
        )*
    };
}

ints_to_opt!(u8, i8, u16, i16, u32, i32, u64, i64);

macro_rules! floats_to_opt {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for OptionValue {
                fn from(num: $ty) -> Self {
                    OptionValue::Float(num.as_())
                }
            }
        )*
    };
}

floats_to_opt!(f32, f64);

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
        CommandContent {
            flag: CommandFlag::Empty,
            data: format!("keyword {} {}", $k, $v),
        }
    };
    (g $l:tt) => {
        CommandContent {
            flag: CommandFlag::JSON,
            data: format!("getoption {}", $l),
        }
    };
}

impl Keyword {
    fn parse_opts(
        OptionRaw {
            option,
            int,
            float,
            str,
        }: OptionRaw,
    ) -> Keyword {
        const HYPR_UNSET_FLOAT: f64 = -340282346638528859811704183484516925440.0;
        const HYPR_UNSET_INT: i64 = -9223372036854775807;

        let value = if float == HYPR_UNSET_FLOAT {
            if int == HYPR_UNSET_INT {
                OptionValue::String(str)
            } else {
                OptionValue::Int(int)
            }
        } else {
            OptionValue::Float(float)
        };

        Keyword { option, value }
    }

    /// This function sets a keyword's value
    pub fn set<Str: ToString, Opt: Into<OptionValue>>(key: Str, value: Opt) -> crate::Result<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let _ = write_to_socket_sync(
            socket_path,
            keyword!((key.to_string()), (value.into().to_string())),
        )?;
        Ok(())
    }
    /// This function sets a keyword's value (async)
    pub async fn set_async<Str: ToString, Opt: Into<OptionValue>>(
        key: Str,
        value: Opt,
    ) -> crate::Result<()> {
        let socket_path = get_socket_path(SocketType::Command);
        let _ = write_to_socket(
            socket_path,
            keyword!((key.to_string()), (value.into().to_string())),
        )
        .await?;
        Ok(())
    }
    /// This function returns the value of a keyword
    pub fn get<Str: ToString>(key: Str) -> crate::Result<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let data = write_to_socket_sync(socket_path, keyword!(g(key.to_string())))?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let keyword = Keyword::parse_opts(deserialized);
        Ok(keyword)
    }
    /// This function returns the value of a keyword (async)
    pub async fn get_async<Str: ToString>(key: Str) -> crate::Result<Self> {
        let socket_path = get_socket_path(SocketType::Command);
        let data = write_to_socket(socket_path, keyword!(g(key.to_string()))).await?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let keyword = Keyword::parse_opts(deserialized);
        Ok(keyword)
    }
}
