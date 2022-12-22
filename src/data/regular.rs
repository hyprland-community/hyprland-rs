use super::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// This private function is to call socket commands
async fn call_hyprctl_data_cmd_async(cmd: DataCommands) -> String {
    let cmd_string = match cmd {
        DataCommands::Monitors => "monitors".to_string(),
        DataCommands::ActiveWindow => "activewindow".to_string(),
        DataCommands::Clients => "clients".to_string(),
        DataCommands::Devices => "devices".to_string(),
        DataCommands::Layers => "layers".to_string(),
        DataCommands::Workspaces => "workspaces".to_string(),
        DataCommands::Version => "version".to_string(),
        //DataCommands::Keyword(key) => format!("getoption {key}"),
    };

    let socket_path = get_socket_path(SocketType::Command);

    match write_to_socket(socket_path, format!("j/{cmd_string}").as_bytes()).await {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    }
}

fn call_hyprctl_data_cmd(cmd: DataCommands) -> String {
    let cmd_string = match cmd {
        DataCommands::Monitors => "monitors".to_string(),
        DataCommands::ActiveWindow => "activewindow".to_string(),
        DataCommands::Clients => "clients".to_string(),
        DataCommands::Devices => "devices".to_string(),
        DataCommands::Layers => "layers".to_string(),
        DataCommands::Workspaces => "workspaces".to_string(),
        DataCommands::Version => "version".to_string(),
        //DataCommands::Keyword(key) => format!("getoption {key}"),
    };

    let socket_path = get_socket_path(SocketType::Command);

    match write_to_socket_sync(socket_path, format!("j/{cmd_string}").as_bytes()) {
        Ok(data) => data,
        Err(e) => panic!(
            "A error occured while parsing the output from the hypr socket: {:?}",
            e
        ),
    }
}

/// This pub(crate) enum holds every socket command that returns data
#[derive(Debug)]
pub(crate) enum DataCommands {
    Monitors,
    Workspaces,
    Clients,
    ActiveWindow,
    Layers,
    Devices,
    Version,
    //Keyword(String),
}

/// This struct holds a basic identifier for a workspace often used in other structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceBasic {
    /// The workspace Id
    #[serde(deserialize_with = "de_work_id")]
    pub id: WorkspaceType,
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
    /// idk what this is lol
    pub transform: Transforms,
    /// a string that identifies if the display is active
    pub focused: bool,
}

#[async_trait]
impl HyprDataActive for Monitor {
    fn get_active() -> HResult<Self> {
        let all = Monitors::get()?;
        if let Some(it) = all.vec().iter().find(|item| item.focused) {
            Ok(it.clone())
        } else {
            panic!("No active monitor?")
        }
    }
    async fn get_active_async() -> HResult<Self> {
        let all = Monitors::get_async().await?;
        if let Some(it) = all.vec().iter().find(|item| item.focused) {
            Ok(it.clone())
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct WorkspaceRaw {
    /// The workspace Id
    pub id: i8,
    /// The workspace's name
    pub name: String,
    /// The monitor the workspace is on
    pub monitor: String,
    /// The amount of windows in the workspace
    pub windows: u8,
    /// A bool that shows if there is a fullscreen window in the workspace
    #[serde(rename = "hasfullscreen")]
    pub fullscreen: bool,
}

/// This struct holds information for a workspace
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    /// The workspace Id
    pub id: WorkspaceType,
    /// The workspace's name
    pub name: String,
    /// The monitor the workspace is on
    pub monitor: String,
    /// The amount of windows in the workspace
    pub windows: u8,
    /// A bool that shows if there is a fullscreen window in the workspace
    #[serde(rename = "hasfullscreen")]
    pub fullscreen: bool,
}

impl From<WorkspaceRaw> for Workspace {
    fn from(raw: WorkspaceRaw) -> Self {
        Workspace {
            id: match raw.id {
                -99 => WorkspaceType::Special,
                0.. => WorkspaceType::Unnamed(match raw.id.try_into() {
                    Ok(num) => num,
                    Err(e) => panic!("Issue with parsing id (i8) as u8: {e}"),
                }),
                _ => panic!("Unrecognised id"),
            },
            name: raw.name,
            monitor: raw.monitor,
            windows: raw.windows,
            fullscreen: raw.fullscreen,
        }
    }
}

#[async_trait]
impl HyprDataActive for Workspace {
    fn get_active() -> HResult<Self> {
        let all = Workspaces::get()?;
        let mon = Monitor::get_active()?;

        if let Some(it) = all
            .vec()
            .iter()
            .find(|item| item.id == mon.active_workspace.id)
        {
            Ok(it.clone())
        } else {
            panic!("No active monitor?")
        }
    }
    async fn get_active_async() -> HResult<Self> {
        let all = Workspaces::get_async().await?;
        let mon = Monitor::get_active_async().await?;

        if let Some(it) = all
            .vec()
            .iter()
            .find(|item| item.id == mon.active_workspace.id)
        {
            Ok(it.clone())
        } else {
            panic!("No active monitor?")
        }
    }
}

fn parse_works(data: String) -> HResult<Vec<Workspace>> {
    let deserialized: Vec<WorkspaceRaw> = serde_json::from_str(&data)?;
    let new = deserialized
        .iter()
        .map(|work| Workspace::from(work.clone()));
    Ok(new.collect())
}

create_data_struct!(
    vecp Workspaces,
    DataCommands::Workspaces,
    parse_works,
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
    /// The monitor the window is on
    pub monitor: u8,
    /// The window class
    pub class: String,
    /// The window title
    pub title: String,
    /// The process Id of the client
    pub pid: u32,
    /// Is this window running under XWayland?
    pub xwayland: bool,
}

//impl_on!();

// #[async_trait]
// impl HyprDataActive for Client {
//     fn get_active() -> HResult<Self> {
//
//     }
//     async fn get_active_async() -> HResult<Self> {}
// }

create_data_struct!(
    vec Clients,
    DataCommands::Clients,
    Client,
    "This struct holds a vector of clients"
);

//pub type Clients = Vec<Client>;

/// This enum holds the information for the active window
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveWindow(
    /// The client data
    #[serde(deserialize_with = "object_empty_as_none")]
    pub Option<Client>,
);

impl_on!(ActiveWindow, DataCommands::ActiveWindow);

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

create_data_struct!(
    sing Layers,
    DataCommands::Layers,
    HashMap<String, LayerDisplay>,
    "This struct holds a hashmap of all current displays, and their layer surfaces"
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
impl_on!(Devices, DataCommands::Devices);

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
impl_on!(Version, DataCommands::Version);
