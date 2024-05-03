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
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub str: Option<String>,
    pub set: bool,
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
    /// Is value overriden or not
    pub set: bool,
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
            set,
        }: OptionRaw,
    ) -> crate::Result<Keyword> {
        let int_exists = int.is_some() as u8;
        let float_exists = float.is_some() as u8;
        let str_exists = str.is_some() as u8;

        // EXPLANATION: if at least two types of value is exists then we stop execution.
        if int_exists + float_exists + str_exists > 1 {
            hypr_err!("Expected single value type, but received more than one! Please open an issue with hyprland-rs with the information: Option {{ option: {option}, int: {int:?}, float: {float:?}, str: {str:?}, set: {set} }}!");
        }

        let value = match (int, float, str) {
            (Some(int), _, _) => OptionValue::Int(int),
            (_, Some(float), _) => OptionValue::Float(float),
            (_, _, Some(str)) => OptionValue::String(str),
            (int, float, str) => hypr_err!("Expected either an 'int', a 'float' or a 'str', but none of them is not received! Please open an issue with hyprland-rs with the information: Option {{ option: {option}, int: {int:?}, float: {float:?}, str: {str:?}, set: {set} }}!"),
        };

        Ok(Keyword { option, value, set })
    }

    /// This function sets a keyword's value
    pub fn set<Str: ToString, Opt: Into<OptionValue>>(key: Str, value: Opt) -> crate::Result<()> {
        let _ = write_to_socket_sync(
            SocketType::Command,
            keyword!((key.to_string()), (value.into().to_string())),
        )?;
        Ok(())
    }
    /// This function sets a keyword's value (async)
    pub async fn set_async<Str: ToString, Opt: Into<OptionValue>>(
        key: Str,
        value: Opt,
    ) -> crate::Result<()> {
        let _ = write_to_socket(
            SocketType::Command,
            keyword!((key.to_string()), (value.into().to_string())),
        )
        .await?;
        Ok(())
    }
    /// This function returns the value of a keyword
    pub fn get<Str: ToString>(key: Str) -> crate::Result<Self> {
        let data = write_to_socket_sync(SocketType::Command, keyword!(g(key.to_string())))?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let keyword = Keyword::parse_opts(deserialized)?;
        Ok(keyword)
    }
    /// This function returns the value of a keyword (async)
    pub async fn get_async<Str: ToString>(key: Str) -> crate::Result<Self> {
        let data = write_to_socket(SocketType::Command, keyword!(g(key.to_string()))).await?;
        let deserialized: OptionRaw = serde_json::from_str(&data)?;
        let keyword = Keyword::parse_opts(deserialized)?;
        Ok(keyword)
    }
}
