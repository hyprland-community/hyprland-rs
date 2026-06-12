//! Encoding detection for byte streams that come back from the Hyprland IPC.
//!
//! Hyprland stores window titles verbatim — whatever bytes the X11/Wayland
//! client set via `_NET_WM_NAME` or `xdg_toplevel::set_title`. Some clients
//! (notably OnlyOffice and Java/Swing apps like Jubler) use a non-UTF-8
//! encoding (typically the JVM's default charset or Latin-1) and so the
//! title contains bytes that are not valid UTF-8.
//!
//! Naively calling `String::from_utf8_lossy` would replace every invalid byte
//! with U+FFFD and destroy the actual content (e.g. `tradu��ocoletiva.docx`
//! instead of `traduçãocoletiva.docx`).
//!
//! This module detects the most likely encoding of the byte stream using
//! `chardetng` (the same library Firefox uses) and decodes with
//! `encoding_rs`. If detection fails or the stream is ambiguous, we fall
//! back to UTF-8 lossy — same behavior as before, no regression.

use chardetng::EncodingDetector;

/// Decode arbitrary bytes as a string, trying to detect the encoding.
///
/// Behavior:
/// 1. If the bytes are valid UTF-8, return them as-is (no detection overhead).
/// 2. Otherwise, run `chardetng` to guess the encoding, then decode with
///    `encoding_rs`. Any remaining invalid bytes are replaced with U+FFFD.
/// 3. As a last resort, return `String::from_utf8_lossy(bytes)` (same as the
///    pre-fix behavior — better than crashing or panicking).
pub fn decode_ipc_response(bytes: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_owned();
    }
    let mut det = EncodingDetector::new();
    det.feed(bytes, true);
    let enc = det.guess(None, true);
    let (decoded, _, _) = enc.decode(bytes);
    if decoded.contains('\u{FFFD}') {
        // Detection was wrong / ambiguous — fall back to lossy UTF-8 so the
        // caller at least gets the same behavior as before this fix.
        return String::from_utf8_lossy(bytes).into_owned();
    }
    decoded.into_owned()
}
