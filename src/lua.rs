//! Helpers for the Lua snippets Hyprland parses on a Lua config
//!
//! Since 0.55 Hyprland can read its config as Lua, and on such a config every dispatcher
//! and every config value is a Lua snippet, so each string that ends up in one goes
//! through a Lua string literal: a workspace name, a monitor description, a shell command
//! to run.
//!
//! Such a string is rarely under the caller's control. A window title is picked by
//! the application, a workspace name may come from a user template. One `"` in it is
//! enough to make the snippet fail to parse, and a crafted one makes the compositor
//! run whatever Lua follows.
//!
//! These helpers are here so that the escaping is written once, in the library,
//! rather than in every crate that talks to Hyprland. [`quote_string`](crate::lua::quote_string)
//! is the one to reach for: it writes the quotes itself, so its result cannot land in the
//! wrong kind of literal.

/// Escapes `value` so it can be embedded in a double quoted Lua string literal.
///
/// The returned string carries no quotes of its own, so it can be dropped into a
/// literal that is already being built; [`quote_string`] returns the whole literal.
///
/// `\`, `"` and the ASCII control characters are escaped; everything else — non ASCII
/// text included — is left untouched.
///
/// # Warning
///
/// The result is escaped for the `"..."` form **only**. In a `'...'` literal it protects
/// nothing: `'` goes through untouched, so a value can still close the literal and run
/// what follows. Prefer [`quote_string`] unless the surrounding literal is yours.
///
/// ```
/// use hyprland::lua::escape_string;
///
/// assert_eq!(escape_string("1:kitty"), "1:kitty");
/// assert_eq!(escape_string(r#"1:"foo""#), r#"1:\"foo\""#);
/// assert_eq!(escape_string("1:💻 ✨"), "1:💻 ✨");
/// ```
pub fn escape_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str(r"\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str(r"\n"),
            '\r' => escaped.push_str(r"\r"),
            '\t' => escaped.push_str(r"\t"),
            // The remaining control characters are written as `\ddd`, which Lua reads as
            // a byte. The three digits are mandatory: `\0` followed by a digit of the
            // string itself would be read as part of the escape.
            ch if ch.is_ascii_control() => escaped.push_str(&format!("\\{:03}", ch as u8)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

/// Turns `value` into a complete, double quoted Lua string literal.
///
/// This is [`escape_string`] surrounded by `"`, which is what a generated snippet
/// needs most of the time, and the form to prefer: it cannot be dropped into the
/// wrong kind of literal.
///
/// ```
/// use hyprland::lua::quote_string;
///
/// assert_eq!(quote_string("1:kitty"), r#""1:kitty""#);
/// assert_eq!(quote_string(r#"say "hi""#), r#""say \"hi\"""#);
/// ```
pub fn quote_string(value: &str) -> String {
    let escaped = escape_string(value);
    let mut literal = String::with_capacity(escaped.len() + 2);
    literal.push('"');
    literal.push_str(&escaped);
    literal.push('"');
    literal
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cases of hyprland-autoname-workspaces#138, which carries its own copy of
    /// this escaping because the library had none.
    #[test]
    fn test_escape_string_autoname_cases() {
        assert_eq!(escape_string("1:kitty"), "1:kitty");
        assert_eq!(escape_string(r#"1:"foo""#), r#"1:\"foo\""#);
        assert_eq!(escape_string(r"1:\slash"), r"1:\\slash");
        assert_eq!(escape_string("1:\nline"), r"1:\nline");
        assert_eq!(escape_string("1:\rline"), r"1:\rline");
        assert_eq!(escape_string("1:💻 ✨"), "1:💻 ✨");
        assert_eq!(escape_string(""), "");
    }

    /// The output is meant for a `"..."` literal, which is why `'` goes through
    /// untouched — [`quote_string`] is the form that cannot be misplaced.
    #[test]
    fn test_escape_string_targets_double_quotes() {
        assert_eq!(escape_string("Tom's"), "Tom's");
        assert_eq!(quote_string("Tom's"), r#""Tom's""#);
    }

    #[test]
    fn test_escape_string_control_characters() {
        assert_eq!(escape_string("a\tb"), r"a\tb");
        assert_eq!(escape_string("a\0b"), r"a\000b");
        // Zero padded: an unpadded `\1` would read the `1` of the name as its second
        // digit, and `"a\11b"` is three bytes where the caller passed four.
        assert_eq!(escape_string("a\u{1}1b"), r"a\0011b");
        assert_eq!(escape_string("a\u{7f}b"), r"a\127b");
        // Only the ASCII controls are escaped, the rest of Unicode goes through.
        assert_eq!(escape_string("a\u{85}b"), "a\u{85}b");
    }

    /// Locks the escaped set as a whole, so it cannot drift: every ASCII control is
    /// escaped, and nothing outside `\`, `"` and the controls is.
    #[test]
    fn test_escape_string_escapes_every_ascii_control() {
        for byte in 0u8..=0x7f {
            let ch = byte as char;
            let escaped = escape_string(&ch.to_string());
            if ch.is_ascii_control() || ch == '"' || ch == '\\' {
                assert!(
                    escaped.starts_with('\\') && escaped.len() >= 2,
                    "{byte:#04x} was not escaped: {escaped:?}"
                );
                assert!(
                    !escaped.chars().any(|c| c.is_ascii_control()),
                    "{byte:#04x} kept a raw control character"
                );
            } else {
                assert_eq!(escaped, ch.to_string(), "{byte:#04x} should go through");
            }
        }
        // A digit of the string right after an escaped control stays out of the escape.
        assert_eq!(escape_string("\u{1b}7"), r"\0277");
        assert_eq!(escape_string("\r\n"), r"\r\n");
    }

    /// A name closing the literal and opening a call of its own, which is what the
    /// escaping is there to defuse.
    #[test]
    fn test_quote_string_closes_nothing() {
        assert_eq!(
            quote_string(r#"a" }) print("pwned") --"#),
            r#""a\" }) print(\"pwned\") --""#
        );
        assert_eq!(quote_string(r"trailing\"), r#""trailing\\""#);
    }

    #[test]
    fn test_quote_string_wraps_escape_string() {
        for value in ["", "plain", "\"\\\n\r\t", "💻", "a\0b"] {
            assert_eq!(quote_string(value), format!("\"{}\"", escape_string(value)));
        }
    }
}
