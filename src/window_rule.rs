use crate::instance::Instance;
use crate::lua::{write_bool_field, write_raw_field, write_string_field};
use crate::{command, default_instance};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Write;

/// This struct holds a keyword
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowRule {
    /// The name of the rule
    pub name: Option<String>,
    /// The match options
    pub r#match: Vec<WindowMatch>,
    /// The effects
    pub effects: Vec<WindowEffect>,
}
impl fmt::Display for WindowRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("hl.window_rule({")?;

        if let Some(name) = &self.name {
            write_string_field(f, "name", name)?;
            f.write_str(", ")?;
        }

        f.write_str("match = {")?;

        for m in &self.r#match {
            m.fmt_lua_pair(f)?;
            f.write_str(", ")?;
        }

        f.write_str("}, ")?;

        for effect in &self.effects {
            effect.fmt_lua_pair(f)?;
            f.write_str(", ")?;
        }

        f.write_str("})")
    }
}
impl WindowRule {
    /// This function sets a keyword's value
    pub fn apply(self) -> crate::Result<()> {
        self.instance_apply(default_instance()?)
    }

    /// This function sets a keyword's value
    pub fn instance_apply(self, instance: &Instance) -> crate::Result<()> {
        let lua = self.to_string();
        instance.write_to_socket(command!(Empty, "eval {}", lua))?;
        Ok(())
    }

    /// This function sets a keyword's value (async)
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn apply_async(self) -> crate::Result<()> {
        self.instance_apply_async(default_instance()?).await
    }

    /// This function sets a keyword's value (async)
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn instance_apply_async(self, instance: &Instance) -> crate::Result<()> {
        let lua = self.to_string();
        instance
            .write_to_socket_async(command!(Empty, "eval {}", lua))
            .await?;
        Ok(())
    }
}

/// Enum containing all match options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WindowMatch {
    Class(String),
    Title(String),
    InitialClass(String),
    InitialTitle(String),
    Tag(String),
    XWayland(bool),
    Float(bool),
    Fullscreen(bool),
    Pin(bool),
    Focus(bool),
    Group(bool),
    Modal(bool),
    FullscreenStateClient(u8),
    FullscreenStateInternal(u8),
    Workspace(String),
    Content(String),
    XdgTag(String),
}
impl WindowMatch {
    fn fmt_lua_pair(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowMatch::Class(v) => write_string_field(f, "class", v),
            WindowMatch::Title(v) => write_string_field(f, "title", v),
            WindowMatch::InitialClass(v) => write_string_field(f, "initial_class", v),
            WindowMatch::InitialTitle(v) => write_string_field(f, "initial_title", v),
            WindowMatch::Tag(v) => write_string_field(f, "tag", v),
            WindowMatch::XWayland(v) => write_bool_field(f, "xwayland", *v),
            WindowMatch::Float(v) => write_bool_field(f, "float", *v),
            WindowMatch::Fullscreen(v) => write_bool_field(f, "fullscreen", *v),
            WindowMatch::Pin(v) => write_bool_field(f, "pin", *v),
            WindowMatch::Focus(v) => write_bool_field(f, "focus", *v),
            WindowMatch::Group(v) => write_bool_field(f, "group", *v),
            WindowMatch::Modal(v) => write_bool_field(f, "modal", *v),
            WindowMatch::FullscreenStateClient(v) => {
                write_raw_field(f, "fullscreen_state_client", *v)
            }
            WindowMatch::FullscreenStateInternal(v) => {
                write_raw_field(f, "fullscreen_state_internal", *v)
            }
            WindowMatch::Workspace(v) => write_string_field(f, "workspace", v),
            WindowMatch::Content(v) => write_string_field(f, "content", v),
            WindowMatch::XdgTag(v) => write_string_field(f, "xdg_tag", v),
        }
    }
}

/// Enum containing all effects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WindowEffect {
    Float(bool),
    Tile(bool),
    Fullscreen(bool),
    Maximize(bool),
    FullscreenState(String),
    Move(String),
    Size(String),
    Center(bool),
    Pseudo(bool),
    Monitor(String),
    Workspace(String),
    NoInitialFocus(bool),
    Pin(bool),
    Group(String),
    SuppressEvent(String),
    Content(String),
    NoCloseFor(i64),
    ScrollingWidth(i64),
}
impl WindowEffect {
    fn fmt_lua_pair(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowEffect::Float(v) => write_bool_field(f, "float", *v),
            WindowEffect::Tile(v) => write_bool_field(f, "tile", *v),
            WindowEffect::Fullscreen(v) => write_bool_field(f, "fullscreen", *v),
            WindowEffect::Maximize(v) => write_bool_field(f, "maximize", *v),
            WindowEffect::FullscreenState(v) => write_string_field(f, "fullscreen_state", v),
            WindowEffect::Move(v) => write_string_field(f, "move", v),
            WindowEffect::Size(v) => write_string_field(f, "size", v),
            WindowEffect::Center(v) => write_bool_field(f, "center", *v),
            WindowEffect::Pseudo(v) => write_bool_field(f, "pseudo", *v),
            WindowEffect::Monitor(v) => write_string_field(f, "monitor", v),
            WindowEffect::Workspace(v) => write_string_field(f, "workspace", v),
            WindowEffect::NoInitialFocus(v) => write_bool_field(f, "no_initial_focus", *v),
            WindowEffect::Pin(v) => write_bool_field(f, "pin", *v),
            WindowEffect::Group(v) => write_string_field(f, "group", v),
            WindowEffect::SuppressEvent(v) => write_string_field(f, "suppress_event", v),
            WindowEffect::Content(v) => write_string_field(f, "content", v),
            WindowEffect::NoCloseFor(v) => write_raw_field(f, "no_close_for", *v),
            WindowEffect::ScrollingWidth(v) => write_raw_field(f, "scrolling_width", *v),
        }
    }
}

#[test]
fn test_window_rules() {
    let rules = vec![
        (
            WindowRule {
                name: Some("apply-something".into()),
                r#match: vec![WindowMatch::Class("my-class".into())],
                effects: vec![WindowEffect::Center(false), WindowEffect::Float(true)],
            },
            r#"hl.window_rule({name = "apply-something", match = {class = "my-class", }, center = false, float = true, })"#,
        ),
        (
            WindowRule {
                name: None,
                r#match: vec![WindowMatch::Focus(false), WindowMatch::Tag("my-tag".into())],
                effects: vec![
                    WindowEffect::NoCloseFor(230434),
                    WindowEffect::FullscreenState("1 2".into()),
                    WindowEffect::Content("video".into()),
                ],
            },
            r#"hl.window_rule({match = {focus = false, tag = "my-tag", }, no_close_for = 230434, fullscreen_state = "1 2", content = "video", })"#,
        ),
        (
            WindowRule {
                name: Some("some-layer-rule".into()),
                r#match: vec![
                    WindowMatch::InitialClass("test".into()),
                    WindowMatch::XWayland(false),
                    WindowMatch::Content("video".into()),
                ],
                effects: vec![
                    WindowEffect::Monitor("1".into()),
                    WindowEffect::Workspace("2".into()),
                    WindowEffect::Group("3".into()),
                    WindowEffect::Pin(true),
                    WindowEffect::NoInitialFocus(true),
                    WindowEffect::ScrollingWidth(234234),
                ],
            },
            r#"hl.window_rule({name = "some-layer-rule", match = {initial_class = "test", xwayland = false, content = "video", }, monitor = "1", workspace = "2", group = "3", pin = true, no_initial_focus = true, scrolling_width = 234234, })"#,
        ),
    ];
    for (rule, lua) in rules {
        assert_eq!(rule.to_string(), lua);
    }
}

/// This struct holds a keyword
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerRule {
    /// The name of the rule
    pub name: Option<String>,
    /// The match options
    pub r#match: Vec<LayerMatch>,
    /// The effects
    pub effects: Vec<LayerEffect>,
}
impl fmt::Display for LayerRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("hl.layer_rule({")?;

        if let Some(name) = &self.name {
            write_string_field(f, "name", name)?;
            f.write_str(", ")?;
        }

        f.write_str("match = {")?;

        for m in &self.r#match {
            m.fmt_lua_pair(f)?;
            f.write_str(", ")?;
        }

        f.write_str("}, ")?;

        for effect in &self.effects {
            effect.fmt_lua_pair(f)?;
            f.write_str(", ")?;
        }

        f.write_str("})")
    }
}
impl LayerRule {
    /// This function sets a keyword's value
    pub fn apply(self) -> crate::Result<()> {
        self.instance_apply(default_instance()?)
    }

    /// This function sets a keyword's value
    pub fn instance_apply(self, instance: &Instance) -> crate::Result<()> {
        let lua = self.to_string();
        instance.write_to_socket(command!(Empty, "eval {}", lua))?;
        Ok(())
    }

    /// This function sets a keyword's value (async)
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn apply_async(self) -> crate::Result<()> {
        self.instance_apply_async(default_instance()?).await
    }

    /// This function sets a keyword's value (async)
    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub async fn instance_apply_async(self, instance: &Instance) -> crate::Result<()> {
        let lua = self.to_string();
        instance
            .write_to_socket_async(command!(Empty, "eval {}", lua))
            .await?;
        Ok(())
    }
}

/// Enum containing all match options
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LayerMatch {
    Namespace(String),
}

impl LayerMatch {
    fn fmt_lua_pair(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayerMatch::Namespace(v) => write_string_field(f, "namespace", v),
        }
    }
}

/// Enum containing all effects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LayerEffect {
    NoAnim(bool),
    Blur(bool),
    BlurPopups(bool),
    IgnoreAlpha(f32),
    DimAround(bool),
    Xray(bool),
    Animation(String),
    Order(i64),
    AboveLock(i64),
    NoScreenShare(bool),
}
impl LayerEffect {
    fn fmt_lua_pair(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayerEffect::NoAnim(v) => write_bool_field(f, "no_anim", *v),
            LayerEffect::Blur(v) => write_bool_field(f, "blur", *v),
            LayerEffect::BlurPopups(v) => write_bool_field(f, "blur_popups", *v),
            LayerEffect::IgnoreAlpha(v) => write_raw_field(f, "ignore_alpha", *v),
            LayerEffect::DimAround(v) => write_bool_field(f, "dim_around", *v),
            LayerEffect::Xray(v) => write_bool_field(f, "xray", *v),
            LayerEffect::Animation(v) => write_string_field(f, "animation", v),
            LayerEffect::Order(v) => write_raw_field(f, "order", *v),
            LayerEffect::AboveLock(v) => write_raw_field(f, "above_lock", *v),
            LayerEffect::NoScreenShare(v) => write_bool_field(f, "no_screen_share", *v),
        }
    }
}

#[test]
fn test_layer_rules() {
    let rules = vec![
        (
            LayerRule {
                name: Some("apply-something".into()),
                r#match: vec![LayerMatch::Namespace("my-layer".into())],
                effects: vec![
                    LayerEffect::Animation("test".into()),
                    LayerEffect::Order(-20),
                ],
            },
            r#"hl.layer_rule({name = "apply-something", match = {namespace = "my-layer", }, animation = "test", order = -20, })"#,
        ),
        (
            LayerRule {
                name: None,
                r#match: vec![LayerMatch::Namespace("my-layer".into())],
                effects: vec![LayerEffect::Xray(false), LayerEffect::DimAround(true)],
            },
            r#"hl.layer_rule({match = {namespace = "my-layer", }, xray = false, dim_around = true, })"#,
        ),
        (
            LayerRule {
                name: Some("some-layer-rule".into()),
                r#match: vec![LayerMatch::Namespace("my-layer-2".into())],
                effects: vec![
                    LayerEffect::IgnoreAlpha(0.53),
                    LayerEffect::Blur(false),
                    LayerEffect::AboveLock(3334),
                ],
            },
            r#"hl.layer_rule({name = "some-layer-rule", match = {namespace = "my-layer-2", }, ignore_alpha = 0.53, blur = false, above_lock = 3334, })"#,
        ),
    ];
    for (rule, lua) in rules {
        assert_eq!(rule.to_string(), lua);
    }
}
