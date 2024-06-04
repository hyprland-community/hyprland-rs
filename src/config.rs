//! # Hyprland Configuration in Rust
//!
use crate::dispatch::{gen_dispatch_str, DispatchType};
use crate::keyword::Keyword;

/// Module providing stuff for adding an removing keybinds
pub mod binds {
    use super::*;

    trait Join: IntoIterator {
        fn join(&self) -> String;
    }

    /// Type for a key held by a bind
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Key<'a> {
        /// Variant for if the bind holds a modded key
        Mod(
            /// Mods
            Vec<Mod>,
            /// Key
            &'a str,
        ),
        /// Variant for a regular key
        Key(&'a str),
    }

    impl std::fmt::Display for Key<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Key::Mod(m, s) => format!("{}_{s}", m.join()),
                    Key::Key(s) => s.to_string(),
                }
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
    #[allow(missing_docs)]
    /// Enum for mod keys used in bind combinations
    pub enum Mod {
        #[display(fmt = "SUPER")]
        SUPER,
        #[display(fmt = "SHIFT")]
        SHIFT,
        #[display(fmt = "ALT")]
        ALT,
        #[display(fmt = "CTRL")]
        CTRL,
        #[display(fmt = "")]
        NONE,
    }

    impl Join for Vec<Mod> {
        fn join(&self) -> String {
            let mut buf = String::new();
            for i in self {
                buf.push_str(&i.to_string());
            }
            buf
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
    #[allow(non_camel_case_types)]
    /// Enum for bind flags
    pub enum Flag {
        /// Works when screen is locked
        #[display(fmt = "l")]
        l,
        /// Used for mouse binds
        #[display(fmt = "m")]
        m,
        /// Repeats when held
        #[display(fmt = "e")]
        e,
        /// Activates on release
        #[display(fmt = "r")]
        r,
    }

    impl Join for Vec<Flag> {
        fn join(&self) -> String {
            let mut buf = String::new();
            for i in self {
                buf.push_str(&i.to_string());
            }
            buf
        }
    }

    /// A struct providing a key bind
    #[derive(Debug, Clone)]
    pub struct Binding<'a> {
        /// All the mods
        pub mods: Vec<Mod>,
        /// The key
        pub key: Key<'a>,
        /// Bind flags
        pub flags: Vec<Flag>,
        /// The dispatcher to be called once complete
        pub dispatcher: DispatchType<'a>,
    }

    /// Struct to hold methods for adding and removing binds
    pub struct Binder;

    impl Binder {
        pub(crate) fn gen_str(binding: Binding) -> crate::Result<String> {
            Ok(format!(
                "{mods},{key},{dispatcher}",
                mods = binding.mods.join(),
                key = binding.key,
                dispatcher = gen_dispatch_str(binding.dispatcher, false)?.data
            ))
        }
        /// Binds a keybinding
        pub fn bind(binding: Binding) -> crate::Result<()> {
            Keyword::set(
                format!("bind{}", binding.flags.join()),
                Self::gen_str(binding)?,
            )?;
            Ok(())
        }
        /// Binds a keybinding (async)
        pub async fn bind_async(binding: Binding<'_>) -> crate::Result<()> {
            Keyword::set_async(
                format!("bind{}", binding.flags.join()),
                Self::gen_str(binding)?,
            )
            .await?;
            Ok(())
        }
    }
    /// Very macro basic abstraction over [Binder] for internal use, **Dont use this instead use [crate::bind]**
    #[macro_export]
    #[doc(hidden)]
    macro_rules! bind_raw {
        (sync $mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::bind(binding)
        }};
        ($mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::bind_async(binding)
        }};
    }

    /// Macro abstraction over [Binder]
    #[macro_export]
    macro_rules! bind {
        ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                sync
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                sync
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                sync
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                sync
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![],
                DispatchType::$dis
            )
        };
        (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                vec![$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                vec![],
                DispatchType::$dis
            )
        };
    }
}

#[test]
fn test_binds() {
    use binds::*;
    let binding = Binding {
        mods: vec![Mod::SUPER],
        key: Key::Key("v"),
        flags: vec![],
        dispatcher: DispatchType::ToggleFloating(None),
    };
    let built_bind = match Binder::gen_str(binding) {
        Ok(v) => v,
        Err(e) => panic!("Error occured: {e}"), // Note to greppers: this is in a test!
    };
    assert_eq!(built_bind, "SUPER,v,/togglefloating");
}
