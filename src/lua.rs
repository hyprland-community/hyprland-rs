use std::fmt;
use std::fmt::Write;

pub fn write_lua_string(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    f.write_char('"')?;
    for ch in s.chars() {
        match ch {
            '\\' => f.write_str("\\\\")?,
            '"' => f.write_str("\\\"")?,
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            c => write!(f, "{c}")?,
        }
    }
    f.write_char('"')
}

pub fn write_string_field(f: &mut fmt::Formatter<'_>, key: &str, value: &str) -> fmt::Result {
    write!(f, "{key} = ")?;
    write_lua_string(f, value)
}

pub fn write_bool_field(f: &mut fmt::Formatter<'_>, key: &str, value: bool) -> fmt::Result {
    write!(f, "{key} = {}", if value { "true" } else { "false" })
}

pub fn write_raw_field<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    key: &str,
    value: T,
) -> fmt::Result {
    write!(f, "{key} = {value}")
}
