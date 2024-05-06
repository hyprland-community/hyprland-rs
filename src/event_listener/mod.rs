#[macro_use]
mod macros;

use crate::shared::*;

mod shared;
pub use crate::event_listener::shared::*;

mod immutable;
pub use crate::event_listener::immutable::EventListener;

mod async_im;
pub use crate::event_listener::async_im::AsyncEventListener;

add_listener!(workspace_change d, WorkspaceType, "on workspace change", "changed workspace to" => id);
add_listener!(workspace_added, WorkspaceType, "a workspace is created", "workspace was added" => id);
add_listener!(workspace_destroy ed, WorkspaceDestroyedEventData, "a workspace is destroyed", "a workspace was destroyed" => data);
add_listener!(workspace_moved, MonitorEventData, "a workspace is moved", "workspace was moved" => id);
add_listener!(workspace_rename, WorkspaceRenameEventData, "a workspace is renamed", "workspace was renamed" => id);
add_listener!(active_monitor_change d, MonitorEventData, "the active monitor is changed", "Active monitor changed to" => data);
add_listener!(active_window_change d, Option<WindowEventData>, "the active window is changed", "Active window changed" => data);
add_listener!(fullscreen_state_change d, bool, "the active monitor is changed", "Fullscreen is on" => state);
add_listener!(monitor_added, String, "a new monitor is added", "Monitor added" => data);
add_listener!(monitor_removed, String, "a monitor is removed", "Monitor removed" => data);
add_listener!(window_open, WindowOpenEvent, "a window is opened", "Window opened" => data);
add_listener!(window_close, Address, "a window is closed", "Window closed" => data);
add_listener!(window_moved, WindowMoveEvent, "a window is moved", "Window moved" => data);
add_listener!(keyboard_layout_change, LayoutEvent, "the keyboard layout is changed", "Layout changed" => data);
add_listener!(sub_map_change d, String, "the sub map is changed", "Submap changed" => data);
add_listener!(layer_open, String, "a new layer is opened", "Layer opened" => data);
add_listener!(layer_closed, String, "a layer is closed", "Layer closed" => data);
add_listener!(float_state, WindowFloatEventData, "the float state of a window is changed", "Float state changed" => data);
add_listener!(urgent_state, Address, "the urgent state of a window is changed", "urgent state changed" => data);
add_listener!(minimize, MinimizeEventData, "the minimize state of a window is changed", "minimize state changed" => data);
add_listener!(window_title_change d, Address, "a window title is changed", "A window title changed" => data);
add_listener!(screencast, ScreencastEventData, "the screencast state of a window is changed", "screencast state changed" => data);
