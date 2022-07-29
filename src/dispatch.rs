use crate::shared::{get_socket_path, write_to_socket, HyprAddress, SocketType};
use std::io;

enum Keywords {
    DisplaySize,
}

enum HyprWindowIdentifier {
    Address(HyprAddress),
    ClassRegularExpression(String),
    Title(String),
}

enum DispatchType {
    Exec(String),
    KillActive,
    Keyword(Keywords),
}

async fn dispatch_cmd(_cmd: DispatchType) -> io::Result<String> {
    let socket_path = get_socket_path(SocketType::Command);
    write_to_socket(socket_path, b"exec kitty").await?;

    Ok("".to_string())
}
