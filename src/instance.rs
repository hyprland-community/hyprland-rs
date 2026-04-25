use crate::error::hypr_err;
use crate::shared::{CommandContent, get_hypr_path};
use std::path::{Path, PathBuf};

/// This is the sync version of the Hyprland Instance.
/// It holds the event streams connected to the sockets of one running Hyprland instance.
#[derive(Debug, Clone)]
pub struct Instance {
    instance: String,
    /// .socket.sock
    stream: Box<Path>,
    /// .hyprpaper.sock
    #[cfg(feature = "hyprpaper")]
    hyprpaper_stream: Box<Path>,
    /// .socket2.sock
    #[cfg(feature = "listener")]
    event_socket_path: Box<Path>,
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
        Self::from_base_socket_path(path)
    }

    /// Uses the name to determine the sockets to use
    ///
    /// Example name: `9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_214680212`
    pub fn from_instance(name: String) -> crate::Result<Self> {
        let mut path = get_hypr_path()?;
        path.push(&name);
        Self::from_base_socket_path(path)
    }

    /// Uses the path to determine the sockets to use
    ///
    /// Example path: `/run/user/1000/hypr/9958d297641b5c84dcff93f9039d80a5ad37ab00_1752788564_21468021`
    pub fn from_base_socket_path(path: PathBuf) -> crate::Result<Self> {
        let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
            hypr_err!("Could not get instance name from path: {}", path.display());
        };
        if !path.exists() {
            hypr_err!("Hyprland instance path does not exist: {}", path.display());
        }
        Ok(Self {
            instance: name,
            stream: path.join(".socket.sock").into_boxed_path(),
            #[cfg(feature = "listener")]
            event_socket_path: path.join(".socket2.sock").into_boxed_path(),
            #[cfg(feature = "hyprpaper")]
            hyprpaper_stream: path.join(".hyprpaper.sock").into_boxed_path(),
        })
    }
}

impl Instance {
    pub(crate) fn write_to_socket(&self, content: CommandContent) -> crate::Result<String> {
        use std::io::{Read, Write};
        let mut stream = std::os::unix::net::UnixStream::connect(&self.stream)?;
        stream.write_all(&content.as_bytes())?;
        let mut response = Vec::new();
        stream.read_to_end(&mut response)?;
        Ok(String::from_utf8_lossy(&response).to_string())
    }

    #[cfg(any(feature = "async-lite", feature = "tokio"))]
    pub(crate) async fn write_to_socket_async(
        &self,
        content: CommandContent,
    ) -> crate::Result<String> {
        use crate::async_import::{AsyncReadExt, AsyncWriteExt};
        let mut stream = crate::async_import::UnixStream::connect(&self.stream).await?;
        stream.write_all(&content.as_bytes()).await?;
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;
        Ok(String::from_utf8_lossy(&response).to_string())
    }

    #[cfg(feature = "hyprpaper")]
    pub(crate) fn write_to_hyprpaper_socket(
        &self,
        content: CommandContent,
    ) -> crate::Result<String> {
        use std::io::{Read, Write};
        let mut stream = std::os::unix::net::UnixStream::connect(&self.hyprpaper_stream)?;
        stream.write_all(content.data.as_bytes())?;

        let mut response = Vec::new();
        const BUFFER_SIZE: usize = 4096;
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = stream.read(&mut buf[..])?;
            response.extend_from_slice(&buf[..n]);
            if n < BUFFER_SIZE {
                break;
            }
        }
        Ok(String::from_utf8_lossy(&response).to_string())
    }

    #[cfg(all(feature = "hyprpaper", any(feature = "async-lite", feature = "tokio")))]
    pub(crate) async fn write_to_hyprpaper_socket_async(
        &self,
        content: CommandContent,
    ) -> crate::Result<String> {
        use crate::async_import::{AsyncReadExt, AsyncWriteExt};
        let mut stream = crate::async_import::UnixStream::connect(&self.hyprpaper_stream).await?;
        stream.write_all(content.data.as_bytes()).await?;

        let mut response = Vec::new();
        const BUFFER_SIZE: usize = 4096;
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = stream.read(&mut buf[..]).await?;
            response.extend_from_slice(&buf[..n]);
            if n < BUFFER_SIZE {
                break;
            }
        }
        Ok(String::from_utf8_lossy(&response).to_string())
    }

    #[cfg(feature = "listener")]
    pub(crate) fn get_event_stream(&self) -> crate::Result<std::os::unix::net::UnixStream> {
        let stream = std::os::unix::net::UnixStream::connect(&self.event_socket_path)?;
        Ok(stream)
    }

    #[cfg(all(feature = "listener", any(feature = "async-lite", feature = "tokio")))]
    pub(crate) async fn get_event_stream_async(
        &self,
    ) -> crate::Result<crate::async_import::UnixStream> {
        let stream = crate::async_import::UnixStream::connect(&self.event_socket_path).await?;
        Ok(stream)
    }
}

fn get_env_name() -> crate::Result<String> {
    let instance = match std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(var) => var,
        Err(std::env::VarError::NotPresent) => {
            hypr_err!("Could not get socket path! (Is Hyprland running??)")
        }
        Err(std::env::VarError::NotUnicode(_)) => {
            hypr_err!("Corrupted Hyprland socket variable: Invalid unicode!")
        }
    };
    Ok(instance)
}

#[cfg(test)]
mod tests {

    /// Test that from_utf8_lossy handles non-UTF-8 bytes correctly.
    /// This is the fix from PR #379 - non-UTF-8 bytes (like Windows-1252 encoded
    /// characters) should be replaced with U+FFFD, not cause an error.
    #[test]
    fn test_from_utf8_lossy_handles_invalid_bytes() {
        // Simulate bytes that might come from a Windows-1252 encoded window title
        // The ® character in Windows-1252 is byte 0xAE, which is invalid in UTF-8
        let bytes_with_invalid_utf8: Vec<u8> = vec![
            0x46, 0x61, 0x72, 0x43, 0x72, 0x79, // "FarCry"
            0xAE, // Windows-1252 ® (invalid in UTF-8)
            0x36, // "6"
        ];

        // This should NOT error - it should replace invalid bytes with U+FFFD
        let result = String::from_utf8_lossy(&bytes_with_invalid_utf8);
        let string = result.to_string();

        // The string should contain "FarCry" and "6"
        assert!(string.contains("FarCry"));
        assert!(string.contains("6"));
        // The invalid byte should be replaced with � (U+FFFD)
        assert!(string.contains('�'));
    }

    /// Test that the write_to_socket error handling works with the lossy conversion.
    /// Since we can't easily mock a UnixSocket, we test the conversion directly.
    #[test]
    fn test_response_conversion_is_lossy() {
        // Test with various invalid UTF-8 sequences
        let test_cases: Vec<Vec<u8>> = vec![
            vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0xAE], // "Hello" + invalid byte
            vec![0xC0, 0x80],                         // Invalid UTF-8 sequence
            vec![0xED, 0xA0, 0x80],                   // Surrogate half
        ];

        for bytes in test_cases {
            // This should never panic or error
            let result = String::from_utf8_lossy(&bytes);
            let _string = result.to_string(); // Should always succeed
        }
    }
}
