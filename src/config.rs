//! # Hyprland Configuration in Rust
//!

use crate::dispatch::{DispatchType, gen_dispatch_str};
use crate::keyword::Keyword;

/// Module providing stuff for adding an removing keybinds
pub mod binds {
    use super::*;
    use crate::default_instance;
    use crate::instance::Instance;

    trait Join: IntoIterator {
        fn join(&self) -> String;
    }

    /// Type for a key held by a bind
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Key<'a> {
        /// Variant for if the bind holds a modded key
        Mod(
            /// Mods
            &'a [Mod],
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

    pub use crate::shared::Mod;

    impl Join for Vec<Mod> {
        fn join(&self) -> String {
            let mut buf = String::new();
            for i in self {
                buf.push_str(&i.to_string());
            }
            buf
        }
    }

    impl Join for &[Mod] {
        fn join(&self) -> String {
            let mut buf = String::new();
            for i in *self {
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
        #[display("l")]
        l,
        /// Activates on release
        #[display("r")]
        r,
        /// Repeats when held
        #[display("e")]
        e,
        /// Non-consuming, key/mouse events will be passed to the active window in addition to triggering the dispatcher.
        #[display("n")]
        n,
        /// Used for mouse binds
        #[display("m")]
        m,
        /// Transparent, cannot be shadowed by other binds.
        #[display("t")]
        t,
        /// Ignore mods, will ignore modifiers.
        #[display("i")]
        i,
        /// Separate, will arbitrarily combine keys between each mod/key
        #[display("s")]
        s,
        /// Has description, will allow you to write a description for your bind.
        #[display("d")]
        d,
        /// Bypasses the app's requests to inhibit keybinds.
        #[display("p")]
        p,
    }

    impl Join for Vec<Flag> {
        fn join(&self) -> String {
            let mut buf = String::new();
            for f in self {
                buf.push_str(&f.to_string());
            }
            buf
        }
    }

    impl Join for &[Flag] {
        fn join(&self) -> String {
            let mut buf = String::new();
            for f in *self {
                buf.push_str(&f.to_string());
            }
            buf
        }
    }

    /// A struct used for indentifying bindings
    #[derive(Debug, Clone)]
    pub struct PartialBind<'a> {
        /// The modifiers used
        pub mods: &'a [Mod],
        /// The main key used
        pub key: Key<'a>,
    }

    /// A struct providing a key bind
    #[derive(Debug, Clone)]
    pub struct Binding<'a> {
        /// All the mods
        pub mods: &'a [Mod],
        /// The key
        pub key: Key<'a>,
        /// Bind flags
        pub flags: &'a [Flag],
        /// The dispatcher to be called once complete
        pub dispatcher: DispatchType<'a>,
    }

    /// Struct to hold methods for adding and removing binds
    pub struct Binder;

    impl Binder {
        pub(crate) fn gen_str_partial(PartialBind { mods, key }: PartialBind) -> String {
            format!("{},{key}", (&mods).join())
        }

        pub(crate) fn gen_str(
            Binding {
                mods,
                key,
                dispatcher,
                ..
            }: Binding,
        ) -> crate::Result<String> {
            Ok(format!(
                "{partial},{dispatcher}",
                partial = Self::gen_str_partial(PartialBind { mods, key }),
                dispatcher = gen_dispatch_str(dispatcher, false)?.data
            ))
        }

        /// Binds a keybinding
        pub fn bind(binding: Binding) -> crate::Result<()> {
            Self::instance_bind(default_instance()?, binding)
        }

        /// Unbinds a keybinding
        pub fn unbind(binding: PartialBind) -> crate::Result<()> {
            Self::instance_unbind(default_instance()?, binding)
        }

        /// Unbinds a keybinding
        pub fn instance_unbind(instance: &Instance, binding: PartialBind) -> crate::Result<()> {
            Keyword::instance_set(instance, "unbind", Self::gen_str_partial(binding))
        }

        /// Unbinds a keybinding (async)
        #[cfg(any(feature = "async-lite", feature = "tokio"))]
        pub async fn unbind_async(binding: PartialBind<'_>) -> crate::Result<()> {
            Self::instance_unbind_async(default_instance()?, binding).await
        }

        /// Unbinds a keybinding (async)
        #[cfg(any(feature = "async-lite", feature = "tokio"))]
        pub async fn instance_unbind_async(
            instance: &Instance,
            binding: PartialBind<'_>,
        ) -> crate::Result<()> {
            Keyword::instance_set_async(instance, "unbind", Self::gen_str_partial(binding)).await
        }

        /// Binds a keybinding
        pub fn instance_bind(instance: &Instance, binding: Binding) -> crate::Result<()> {
            Keyword::instance_set(
                instance,
                format!("bind{}", (&binding.flags).join()),
                Self::gen_str(binding)?,
            )
        }

        /// Binds a keybinding (async)
        #[cfg(any(feature = "async-lite", feature = "tokio"))]
        pub async fn bind_async(binding: Binding<'_>) -> crate::Result<()> {
            Self::instance_bind_async(default_instance()?, binding).await
        }

        /// Binds a keybinding (async)
        #[cfg(any(feature = "async-lite", feature = "tokio"))]
        pub async fn instance_bind_async(
            instance: &Instance,
            binding: Binding<'_>,
        ) -> crate::Result<()> {
            Keyword::instance_set_async(
                instance,
                format!("bind{}", (&binding.flags).join()),
                Self::gen_str(binding)?,
            )
            .await
        }
    }

    /// Very macro basic abstraction over [Binder] for internal use, **Dont use this instead use [crate::bind]**
    ///
    /// ```rust
    /// # use hyprland::{bind_raw, default_instance, default_instance_panic, dispatch::DispatchType, Result};
    /// #[tokio::main(flavor = "current_thread")]
    /// # async fn test() -> Result<()> {
    ///   let instance = default_instance()?;
    ///   bind_raw!(instance , &[Mod::SHIFT] , Key::Key("m")  ,  &[Flag::l, Flag::r, Flag::m] ,  DispatchType::Exit )?;
    ///   bind_raw!(&[Mod::SHIFT] , Key::Key("m")  ,  &[Flag::l, Flag::r, Flag::m] ,  DispatchType::Exit )?;
    ///   bind_raw!(async, instance, &[Mod::SHIFT] , Key::Key("m")  ,  &[Flag::l, Flag::r, Flag::m] ,  DispatchType::Exit).await?;
    ///   bind_raw!(async, &[Mod::SHIFT] , Key::Key("m")  ,  &[Flag::l, Flag::r, Flag::m] ,  DispatchType::Exit).await?;
    ///   Ok(())
    /// # }
    /// ```
    #[macro_export]
    macro_rules! bind_raw {
        (async, $instance:expr,$mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::instance_bind_async($instance, binding)
        }};
        (async, $mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::bind_async(binding)
        }};
        ($instance:expr,$mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::instance_bind($instance, binding)
        }};
        ($mods:expr,$key:expr,$flags:expr,$dis:expr ) => {{
            use $crate::config::binds::*;
            let binding = Binding {
                mods: $mods,
                key: $key,
                flags: $flags,
                dispatcher: $dis,
            };
            Binder::bind(binding)
        }};
    }

    /// Macro abstraction over [Binder]
    ///
    /// ```rust
    /// # use hyprland::{bind, default_instance, dispatch::DispatchType, Result};
    /// # use hyprland::instance::Instance;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// # async fn test() -> Result<()> {
    ///     let instance = default_instance()?;
    ///     bind!(instance, l r m | SHIFT, Key, "m" => Exit);
    ///     bind!(SHIFT ALT, Key, "b" => CenterWindow);
    ///     bind!(async ; l r m | SHIFT, Key, "m" => Exit);
    ///     bind!(async ; instance, SUPER, Key, "l" => CenterWindow);
    ///     bind!(async ; SHIFT ALT, Key, "b" => CenterWindow);
    ///     Ok(())
    /// # }
    /// ```
    #[macro_export]
    macro_rules! bind {
        (async ; $instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                async,
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                async,
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        (async ; $instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                async,
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                async,
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis
            )
        };
        (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                async,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                async,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                async,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis( $($arg),* )
            )
        };
        (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                async,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis
            )
        };
        ($instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        ($instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                $instance,
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis
            )
        };
        ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[$(Flag::$flag), *],
                DispatchType::$dis
            )
        };
        ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => {
            $crate::bind_raw!(
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis( $($arg),* )
            )
        };
        ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => {
            $crate::bind_raw!(
                &[$(Mod::$mod), *],
                Key::$keyt( $( $key ), * ),
                &[],
                DispatchType::$dis
            )
        };
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::shared::Mod;

        #[test]
        fn test_vec_mod_join() {
            let mods = vec![Mod::SUPER, Mod::SHIFT, Mod::CTRL];
            assert_eq!(mods.join(), "SUPERSHIFTCTRL");
        }

        #[test]
        fn test_vec_flag_join() {
            let flags = vec![Flag::l, Flag::r, Flag::m];
            assert_eq!(flags.join(), "lrm");
        }

        #[test]
        fn test_slice_flag_join() {
            let flags: &[Flag] = &[Flag::n, Flag::t, Flag::i];
            assert_eq!(flags.join(), "nti");
        }

        #[test]
        fn test_vec_flag_join_empty() {
            let flags: Vec<Flag> = vec![];
            assert_eq!(flags.join(), "");
        }
    }
}

#[test]
fn test_binds() {
    use binds::*;
    let binding = Binding {
        mods: &[Mod::SUPER],
        key: Key::Key("v"),
        flags: &[],
        dispatcher: DispatchType::ToggleFloating(None),
    };
    let built_bind = match Binder::gen_str(binding) {
        Ok(v) => v,
        Err(e) => panic!("Error occured: {e}"), // Note to greppers: this is in a test!
    };
    assert_eq!(built_bind, "SUPER,v,togglefloating");
}

#[test]
fn test_key_mod_display() {
    use binds::*;
    let key = Key::Mod(&[Mod::SUPER, Mod::SHIFT], "V");
    assert_eq!(key.to_string(), "SUPERSHIFT_V");
}

#[test]
fn test_partial_bind() {
    use binds::*;
    let partial = PartialBind {
        mods: &[Mod::SUPER],
        key: Key::Key("v"),
    };
    let result = Binder::gen_str_partial(partial);
    assert_eq!(result, "SUPER,v");
}

#[test]
fn test_binding_with_flags() -> crate::Result<()> {
    use binds::*;
    let binding = Binding {
        mods: &[Mod::SUPER, Mod::SHIFT],
        key: Key::Key("v"),
        flags: &[Flag::l, Flag::r],
        dispatcher: DispatchType::ToggleFloating(None),
    };
    let result = Binder::gen_str(binding)?;
    assert!(result.contains("SUPERSHIFT,v"));
    assert!(result.contains("togglefloating"));
    Ok(())
}

#[test]
fn test_key_mod_with_multiple_mods() {
    use binds::*;
    let key = Key::Mod(&[Mod::SUPER, Mod::CTRL, Mod::ALT], "Return");
    let result = key.to_string();
    assert_eq!(result, "SUPERCTRLALT_Return");
}

#[test]
fn test_flag_display() {
    use binds::Flag;
    assert_eq!(Flag::l.to_string(), "l");
    assert_eq!(Flag::r.to_string(), "r");
    assert_eq!(Flag::e.to_string(), "e");
    assert_eq!(Flag::n.to_string(), "n");
    assert_eq!(Flag::m.to_string(), "m");
    assert_eq!(Flag::t.to_string(), "t");
    assert_eq!(Flag::i.to_string(), "i");
    assert_eq!(Flag::s.to_string(), "s");
    assert_eq!(Flag::d.to_string(), "d");
    assert_eq!(Flag::p.to_string(), "p");
}

#[test]
fn test_binding_with_all_flag_types() -> crate::Result<()> {
    use binds::*;
    let binding = Binding {
        mods: &[Mod::SUPER],
        key: Key::Key("a"),
        flags: &[
            Flag::l,
            Flag::r,
            Flag::e,
            Flag::n,
            Flag::m,
            Flag::t,
            Flag::i,
            Flag::s,
            Flag::d,
            Flag::p,
        ],
        dispatcher: DispatchType::ToggleFloating(None),
    };
    let result = Binder::gen_str(binding)?;
    assert!(result.contains("SUPER,a"));
    assert!(result.contains("togglefloating"));
    Ok(())
}
