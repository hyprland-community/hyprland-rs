mod sync {
    use crate::error::hypr_err;
    use crate::instance::get_env_name;
    use crate::shared::{get_hypr_path, CommandContent};
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::OnceLock;

    /// This is the sync version of the Hyprland Instance.
    ///
    /// It holds Locks to all the sockets that Hyprland uses.
    pub struct Instance {
        instance: String,
        /// .socket.sock
        socket: OnceLock<UnixStream>,
        /// .socket2.sock
        event_socket: OnceLock<UnixStream>,
        /// .hyprpaper.sock
        hyprpaper_socket: OnceLock<UnixStream>,
    }

    impl PartialEq<Self> for Instance {
        fn eq(&self, other: &Self) -> bool {
            self.instance == other.instance
        }
    }

    impl Instance {
        /// uses the $HYPRLAND_INSTANCE_SIGNATURE env variable
        pub fn from_current_env() -> crate::Result<Self> {
            let mut path = get_hypr_path()?;
            let name = get_env_name()?;
            path.push(&name);
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock"))?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock"))?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock"))?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }

        /// instance: 9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_214680212
        pub fn from_instance(name: String) -> crate::Result<Self> {
            let mut path = get_hypr_path()?;
            path.push(&name);
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock"))?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock"))?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock"))?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }

        /// /run/user/$UID/hypr/9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_214680212
        pub fn from_base_socket_path(path: PathBuf) -> crate::Result<Self> {
            let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
                hypr_err!("Could not get instance name from path: {}", path.display());
            };
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock"))?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock"))?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock"))?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }
    }

    impl Instance {
        pub(crate) fn write_to_socket(&self, content: CommandContent) -> crate::Result<String> {
            use std::io::{Read, Write};
            let Some(mut stream) = self.socket.get() else {
                hypr_err!("Socket not initialized!");
            };
            stream.write_all(&content.as_bytes())?;
            // stream.flush()?;
            let mut response = Vec::new();
            stream.read_to_end(&mut response)?;
            Ok(String::from_utf8(response)?)
        }

        pub(crate) fn get_event_stream(&mut self) -> crate::Result<&mut UnixStream> {
            let Some(stream) = self.event_socket.get_mut() else {
                hypr_err!("Socket not initialized!");
            };
            Ok(stream)
        }

        pub(crate) fn write_to_hyprpaper_socket(
            &self,
            content: CommandContent,
        ) -> crate::Result<String> {
            use std::io::{Read, Write};
            let Some(mut stream) = self.hyprpaper_socket.get() else {
                hypr_err!("Hyprpaper socket not initialized!");
            };
            stream.write_all(content.data.as_bytes())?;
            // stream.flush()?;
            let mut response = Vec::new();
            stream.read_to_end(&mut response)?;
            Ok(String::from_utf8(response)?)
        }
    }
}
mod r#async {
    use crate::async_import::*;
    use crate::error::hypr_err;
    use crate::instance::get_env_name;
    use crate::shared::{get_hypr_path, CommandContent};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    /// This is the async version of the Hyprland Instance.
    ///
    /// It holds Locks to all the sockets that Hyprland uses.
    pub struct AsyncInstance {
        instance: String,
        /// .socket.sock
        socket: OnceLock<UnixStream>,
        /// .socket2.sock
        event_socket: OnceLock<UnixStream>,
        /// .hyprpaper.sock
        hyprpaper_socket: OnceLock<UnixStream>,
    }

    impl PartialEq<Self> for AsyncInstance {
        fn eq(&self, other: &Self) -> bool {
            self.instance == other.instance
        }
    }

    impl AsyncInstance {
        /// uses the $HYPRLAND_INSTANCE_SIGNATURE env variable
        pub async fn from_current_env() -> crate::Result<Self> {
            let mut path = get_hypr_path()?;
            let name = get_env_name()?;
            path.push(&name);
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock")).await?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock")).await?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock")).await?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }

        /// instance: 9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_214680212
        pub async fn from_instance(name: String) -> crate::Result<Self> {
            let mut path = get_hypr_path()?;
            path.push(&name);
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock")).await?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock")).await?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock")).await?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }

        /// /run/user/$UID/hypr/9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_214680212
        pub async fn from_base_socket_path(path: PathBuf) -> crate::Result<Self> {
            let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
                hypr_err!("Could not get instance name from path: {}", path.display());
            };
            if !path.exists() {
                hypr_err!("Hyprland instance path does not exist: {}", path.display());
            }
            let socket = OnceLock::new();
            let _ = socket.set(UnixStream::connect(path.join(".socket.sock")).await?);
            let event_socket = OnceLock::new();
            let _ = event_socket.set(UnixStream::connect(path.join(".socket2.sock")).await?);
            let hyprpaper_socket = OnceLock::new();
            let _ = hyprpaper_socket.set(UnixStream::connect(path.join(".hyprpaper.sock")).await?);
            Ok(Self {
                instance: name,
                socket,
                event_socket,
                hyprpaper_socket,
            })
        }
    }

    impl AsyncInstance {
        pub(crate) async fn write_to_socket(
            &mut self,
            content: CommandContent,
        ) -> crate::Result<String> {
            let Some(stream) = self.socket.get_mut() else {
                hypr_err!("Socket not initialized!");
            };
            stream.write_all(&content.as_bytes()).await?;
            // stream.flush().await?;
            let mut response = Vec::new();
            stream.read_to_end(&mut response).await?;
            Ok(String::from_utf8(response)?)
        }

        pub(crate) fn get_event_stream(&mut self) -> crate::Result<&mut UnixStream> {
            let Some(stream) = self.event_socket.get_mut() else {
                hypr_err!("Socket not initialized!");
            };
            Ok(stream)
        }

        pub(crate) async fn write_to_hyprpaper_socket(
            &mut self,
            content: CommandContent,
        ) -> crate::Result<String> {
            let Some(stream) = self.hyprpaper_socket.get_mut() else {
                hypr_err!("Hyprpaper socket not initialized!");
            };
            stream.write_all(content.data.as_bytes()).await?;
            // stream.flush()?;
            let mut response = Vec::new();
            stream.read_to_end(&mut response).await?;
            Ok(String::from_utf8(response)?)
        }
    }
}

use crate::error::hypr_err;
pub use r#async::*;
use std::env;
pub use sync::*;

fn get_env_name() -> crate::Result<String> {
    let instance = match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(var) => var,
        Err(env::VarError::NotPresent) => {
            hypr_err!("Could not get socket path! (Is Hyprland running??)")
        }
        Err(env::VarError::NotUnicode(_)) => {
            hypr_err!("Corrupted Hyprland socket variable: Invalid unicode!")
        }
    };
    Ok(instance)
}
