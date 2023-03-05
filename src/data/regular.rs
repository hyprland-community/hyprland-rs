use super::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// This private function is to call socket commands
async fn call_hyprctl_data_cmd_async(cmd: DataCommands) -> String {
    let cmd_string = cmd.to_string();

    let socket_path = get_socket_path(SocketType::Command);

    match write_to_socket(socket_path, format!("j/{cmd_string}").as_bytes()).await {
        Ok(data) => data,
        Err(e) => panic!("A error occured while parsing the output from the hypr socket: {e:?}"),
    }
}

fn call_hyprctl_data_cmd(cmd: DataCommands) -> String {
    let cmd_string = cmd.to_string();

    let socket_path = get_socket_path(SocketType::Command);

    match write_to_socket_sync(socket_path, format!("j/{cmd_string}").as_bytes()) {
        Ok(data) => data,
        Err(e) => panic!("A error occured while parsing the output from the hypr socket: {e:?}"),
    }
}

/// This pub(crate) enum holds every socket command that returns data
#[derive(Debug, Display)]
pub(crate) enum DataCommands {
    #[display(fmt = "monitors")]
    Monitors,
    #[display(fmt = "workspaces")]
    Workspaces,
    #[display(fmt = "clients")]
    Clients,
    #[display(fmt = "activewindow")]
    ActiveWindow,
    #[display(fmt = "layers")]
    Layers,
    #[display(fmt = "devices")]
    Devices,
    #[display(fmt = "version")]
    Version,
    #[display(fmt = "cursorpos")]
    CursorPosition,
    #[display(fmt = "binds")]
    Binds,
    #[display(fmt = "animations")]
    Animations,
}

/// This struct holds a basic identifier for a workspace often used in other structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceBasic {
    /// The workspace Id
    pub id: WorkspaceId,
    /// The workspace's name
    pub name: String,
}

/// This enum provides the different monitor transforms
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Transforms {
    /// No transform
    Normal = 0,
    /// Rotated 90 degrees
    Normal90 = 1,
    /// Rotated 180 degrees
    Normal180 = 2,
    /// Rotated 270 degrees
    Normal270 = 3,
    /// Flipped
    Flipped = 4,
    /// Flipped and rotated 90 degrees
    Flipped90 = 5,
    /// Flipped and rotated 180 degrees
    Flipped180 = 6,
    /// Flipped and rotated 270 degrees
    Flipped270 = 7,
}

/// This struct holds information for a monitor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    /// The monitor id
    pub id: u8,
    /// The monitor's name
    pub name: String,
    /// The monitor's description
    pub description: String,
    /// The monitor width (in pixels)
    pub width: u16,
    /// The monitor height (in pixels)
    pub height: u16,
    /// The monitor's refresh rate (in hertz)
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
    /// The monitor's position on the x axis (not irl ofc)
    pub x: i32,
    /// The monitor's position on the x axis (not irl ofc)
    pub y: i32,
    /// A basic identifier for the active workspace
    #[serde(rename = "activeWorkspace")]
    pub active_workspace: WorkspaceBasic,
    /// Reserved is the amount of space (in pre-scale pixels) that a layer surface has claimed
    pub reserved: (u8, u8, u8, u8),
    /// The display's scale
    pub scale: f32,
    /// I think like the rotation?
    pub transform: Transforms,
    /// a string that identifies if the display is active
    pub focused: bool,
    /// The dpms status of a monitor
    #[serde(rename = "dpmsStatus")]
    pub dpms_status: bool,
}

#[async_trait]
impl HyprDataActive for Monitor {
    fn get_active() -> HResult<Self> {
        let mut all = Monitors::get()?;
        if let Some(it) = all.find(|item| item.focused) {
            Ok(it)
        } else {
            panic!("No active monitor?")
        }
    }
    async fn get_active_async() -> HResult<Self> {
        let mut all = Monitors::get_async().await?;
        if let Some(it) = all.find(|item| item.focused) {
            Ok(it)
        } else {
            panic!("No active monitor?")
        }
    }
}

create_data_struct!(
    vec Monitors,
    DataCommands::Monitors,
    Monitor,
    "This struct holds a vector of monitors"
);

/// This struct holds information for a workspace
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    /// The workspace Id
    pub id: WorkspaceId,
    /// The workspace's name
    pub name: String,
    /// The monitor the workspace is on
    pub monitor: String,
    /// The amount of windows in the workspace
    pub windows: u8,
    /// A bool that shows if there is a fullscreen window in the workspace
    #[serde(rename = "hasfullscreen")]
    pub fullscreen: bool,
    /// The last window's [Address]
    #[serde(rename = "lastwindow")]
    pub last_window: Address,
    /// The last window's title
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
}

#[async_trait]
impl HyprDataActive for Workspace {
    fn get_active() -> HResult<Self> {
        let mut all = Workspaces::get()?;
        let mon = Monitor::get_active()?;

        if let Some(it) = all.find(|item| item.id == mon.active_workspace.id) {
            Ok(it)
        } else {
            panic!("No active monitor?")
        }
    }
    async fn get_active_async() -> HResult<Self> {
        let mut all = Workspaces::get_async().await?;
        let mon = Monitor::get_active_async().await?;

        if let Some(it) = all.find(|item| item.id == mon.active_workspace.id) {
            Ok(it)
        } else {
            panic!("No active monitor?")
        }
    }
}

create_data_struct!(
    vec Workspaces,
    DataCommands::Workspaces,
    Workspace,
    "This type provides a vector of workspaces"
);

/// This struct holds information for a client/window
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    /// The client's [`Address`][crate::shared::Address]
    pub address: Address,
    /// The window location
    pub at: (i16, i16),
    /// The window size
    pub size: (u16, u16),
    /// The workspace its on
    pub workspace: WorkspaceBasic,
    /// Is this window floating?
    pub floating: bool,
    /// Is this window fullscreen?
    pub fullscreen: bool,
    /// What type of fullscreen?
    #[serde(rename = "fullscreenMode")]
    pub fullscreen_mode: i8,
    /// The monitor the window is on
    pub monitor: i8,
    /// The window class
    pub class: String,
    /// The window title
    pub title: String,
    /// The process Id of the client
    pub pid: u32,
    /// Is this window running under XWayland?
    pub xwayland: bool,
    /// Is this window pinned?
    pub pinned: bool,
    /// Group members
    pub grouped: Vec<Box<Self>>,
    /// The swallowed window
    pub swallowing: Option<Box<Self>>,
}

/// This enum holds the information for the active window
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ActiveWindow(
    /// The client data
    #[serde(deserialize_with = "object_empty_as_none")]
    pub Option<Client>,
);

#[async_trait]
impl HyprDataActiveOptional for Client {
    fn get_active() -> HResult<Option<Self>> {
        let data = call_hyprctl_data_cmd(DataCommands::ActiveWindow);
        let deserialized: ActiveWindow = serde_json::from_str(&data)?;
        Ok(deserialized.0)
    }
    async fn get_active_async() -> HResult<Option<Self>> {
        let data = call_hyprctl_data_cmd_async(DataCommands::ActiveWindow).await;
        let deserialized: ActiveWindow = serde_json::from_str(&data)?;
        Ok(deserialized.0)
    }
}

create_data_struct!(
    vec Clients,
    DataCommands::Clients,
    Client,
    "This struct holds a vector of clients"
);

/// This struct holds information about a layer surface/client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerClient {
    /// The layer's [`Address`][crate::shared::Address]
    pub address: Address,
    /// The layer's x position
    pub x: i32,
    /// The layer's y position
    pub y: i32,
    /// The layer's width
    pub w: u16,
    /// The layer's height
    pub h: u16,
    /// The layer's namespace
    pub namespace: String,
}

/// This struct holds all the layer surfaces for a display
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerDisplay {
    /// The different levels of layers
    pub levels: HashMap<String, Vec<LayerClient>>,
}

impl LayerDisplay {
    /// Returns an iterator over the levels map
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<LayerClient>)> {
        self.levels.iter()
    }
}

create_data_struct!(
    sing Layers,
    DataCommands::Layers,
    HashMap<String, LayerDisplay>,
    "This struct holds a hashmap of all current displays, and their layer surfaces",
    iter_item = (&String, &LayerDisplay)
);

/// This struct holds information about a mouse device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mouse {
    /// The mouse's address
    pub address: Address,
    /// The mouse's name
    pub name: String,
}

/// This struct holds information about a keyboard device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyboard {
    /// The keyboard's address
    pub address: Address,
    /// The keyboard's name
    pub name: String,
    /// The keyboard rules
    pub rules: String,
    /// The keyboard model
    pub model: String,
    /// The layout of the keyboard
    pub layout: String,
    /// The keyboard variant
    pub variant: String,
    /// The keyboard options
    pub options: String,
    /// The keyboard's active keymap
    pub active_keymap: String,
}

/// A enum that holds the types of tablets
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TabletType {
    /// The TabletPad type of tablet
    #[serde(rename = "tabletPad")]
    TabletPad,
    /// The TabletTool type of tablet
    #[serde(rename = "tabletTool")]
    TabletTool,
}

/// A enum to match what the tablet belongs to
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TabletBelongsTo {
    /// The belongsTo data if the tablet is of type TabletPad
    TabletPad {
        /// The name of the parent
        name: String,
        /// The address of the parent
        address: Address,
    },
    /// The belongsTo data if the tablet is of type TabletTool
    Address(Address),
}

/// This struct holds information about a tablet device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tablet {
    /// The tablet's address
    pub address: Address,
    /// The tablet type
    #[serde(rename = "type")]
    pub tablet_type: Option<TabletType>,
    /// What the tablet belongs to
    #[serde(rename = "belongsTo")]
    pub belongs_to: Option<TabletBelongsTo>,
    /// The name of the tablet
    pub name: Option<String>,
}

/// This struct holds all current devices
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Devices {
    /// All the mice
    pub mice: Vec<Mouse>,
    /// All the keyboards
    pub keyboards: Vec<Keyboard>,
    /// All the tablets
    pub tablets: Vec<Tablet>,
}
impl_on!(Devices);

/// This struct holds version information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    /// The git branch Hyprland was built on
    pub branch: String,
    /// The git commit Hyprland was built on
    pub commit: String,
    /// This is true if there were unstaged changed when Hyprland was built
    pub dirty: bool,
    /// The git commit message
    pub commit_message: String,
    /// The flags that Hyprland was built with
    pub flags: Vec<String>,
}
impl_on!(Version);

/// This struct holds information on the cursor position
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CursorPosition {
    /// The x position of the cursor
    pub x: i64,
    /// The y position of the cursor
    pub y: i64,
}
impl_on!(CursorPosition);

/// A keybinding returned from the binds command
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bind {
    /// Is it locked?
    pub locked: bool,
    /// Is it a mouse bind?
    pub mouse: bool,
    /// Does it execute on release?
    pub release: bool,
    /// Can it be held?
    pub repeat: bool,
    /// It's modmask
    pub modmask: u8,
    /// The submap its apart of
    pub submap: String,
    /// The key
    pub key: String,
    /// The keycode
    pub keycode: i16,
    /// The dispatcher to be executed
    pub dispatcher: String,
    /// The dispatcher arg
    pub arg: String,
}

create_data_struct!(
    vec Binds,
    DataCommands::Binds,
    Bind,
    "This struct holds a vector of binds"
);

/// Animation styles
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnimationStyle {
    /// Slide animation
    Slide,
    /// Vertical slide animation
    SlideVert,
    /// Popin animation (with percentage)
    PopIn(u8),
    /// Fade animation
    Fade,
    /// Once animation used for gradient animation
    Once,
    /// Loop animation used for gradient animation
    Loop,
    /// No animation style
    None,
    /// Unknown style
    Unknown(String),
}

impl<Str: ToString + Clone> From<Str> for AnimationStyle {
    fn from(value: Str) -> Self {
        let string = value.to_string();
        if string.starts_with("popin") {
            let mut iter = string.split(' ');
            iter.next();
            AnimationStyle::PopIn({
                let mut str = iter.next().unwrap_or("100%").to_string();
                str.remove(str.len() - 1);

                str.parse().unwrap_or(100_u8)
            })
        } else {
            match value.to_string().as_str() {
                "slide" => AnimationStyle::Slide,
                "slidevert" => AnimationStyle::SlideVert,
                "fade" => AnimationStyle::Fade,
                "once" => AnimationStyle::Once,
                "loop" => AnimationStyle::Loop,
                "" => AnimationStyle::None,
                _ => AnimationStyle::Unknown(string),
            }
        }
    }
}
/// Bezier identifier
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BezierIdent {
    /// No bezier specified
    #[serde(rename = "")]
    None,
    /// The default bezier
    #[serde(rename = "default")]
    Default,
    /// A specified bezier
    #[serde(rename = "name")]
    Specified(String),
}

impl<Str: ToString + Clone> From<Str> for BezierIdent {
    fn from(value: Str) -> Self {
        let str = value.to_string();
        match str.as_str() {
            "" => BezierIdent::None,
            "default" => BezierIdent::Default,
            _ => BezierIdent::Specified(str),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RawBezierIdent {
    pub name: String,
}

/// A bezier curve
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bezier {
    ///. Name of the bezier
    pub name: String,
    /// X position of first point
    pub x0: f32,
    /// Y position of first point
    pub y0: f32,
    /// X position of second point
    pub x1: f32,
    /// Y position of second point
    pub y1: f32,
}

/// A struct representing a animation
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnimationRaw {
    /// The name of the animation
    pub name: String,
    /// Is it overriden?
    pub overriden: bool,
    /// What bezier does it use?
    pub bezier: String,
    /// Is it enabled?
    pub enabled: bool,
    /// How fast is it?
    pub speed: f32,
    /// The style of animation
    pub style: String,
}

/// A struct representing a animation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    /// The name of the animation
    pub name: String,
    /// Is it overriden?
    pub overriden: bool,
    /// What bezier does it use?
    pub bezier: BezierIdent,
    /// Is it enabled?
    pub enabled: bool,
    /// How fast is it?
    pub speed: f32,
    /// The style of animation
    pub style: AnimationStyle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnimationsRaw(Vec<AnimationRaw>, Vec<RawBezierIdent>);

/// Struct that holds animations and beziers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animations(Vec<Animation>, Vec<BezierIdent>);

#[async_trait]
impl HyprData for Animations {
    fn get() -> HResult<Self>
    where
        Self: Sized,
    {
        let out = call_hyprctl_data_cmd(DataCommands::Animations);
        let des: AnimationsRaw = serde_json::from_str(&out)?;
        let AnimationsRaw(anims, beziers) = des;
        let new_anims: Vec<Animation> = anims
            .iter()
            .map(|item| Animation {
                name: item.name.clone(),
                overriden: item.overriden,
                bezier: item.bezier.clone().into(),
                enabled: item.enabled,
                speed: item.speed,
                style: item.style.clone().into(),
            })
            .collect();
        let new_bezs: Vec<BezierIdent> = beziers
            .iter()
            .map(|item| item.name.clone().into())
            .collect();
        Ok(Animations(new_anims, new_bezs))
    }
    async fn get_async() -> HResult<Self>
    where
        Self: Sized,
    {
        let out = call_hyprctl_data_cmd_async(DataCommands::Animations).await;
        let des: AnimationsRaw = serde_json::from_str(&out)?;
        let AnimationsRaw(anims, beziers) = des;
        let new_anims: Vec<Animation> = anims
            .iter()
            .map(|item| Animation {
                name: item.name.clone(),
                overriden: item.overriden,
                bezier: item.bezier.clone().into(),
                enabled: item.enabled,
                speed: item.speed,
                style: item.style.clone().into(),
            })
            .collect();
        let new_bezs: Vec<BezierIdent> = beziers
            .iter()
            .map(|item| item.name.clone().into())
            .collect();
        Ok(Animations(new_anims, new_bezs))
    }
}

//impl_on!(Animations);
