import re

with open('src/dispatch.rs', 'r') as f:
    content = f.read()

# Replace the whole WorkspaceIdentifierWithSpecial enum
old_enum = """pub enum WorkspaceIdentifierWithSpecial<'a> {
    /// The workspace Id
    Id(WorkspaceId),
    /// The workspace relative to the current workspace
    #[display("{}", format_relative(*_0, ""))]
    Relative(i32),
    /// The workspace on the monitor relative to the current workspace
    #[display("{}", format_relative(*_0, "m"))]
    RelativeMonitor(i32),
    /// The workspace on the monitor relative to the current workspace, including empty workspaces
    #[display("{}", format_relative(*_0, "r"))]
    RelativeMonitorIncludingEmpty(i32),
    /// The open workspace relative to the current workspace
    #[display("{}", format_relative(*_0, "e"))]
    RelativeOpen(i32),
    /// The previous Workspace
    #[display("previous")]
    Previous,
    /// The previous Workspace
    #[display("previous_per_monitor")]
    PreviousPerMonitor,
    /// The first available empty workspace
    #[display("{}", format!("empty{}", _0))]
    Empty(FirstEmpty),
    /// The name of the workspace
    #[display("name:{_0}")]
    Name(&'a str),
    /// The special workspace
    #[display("special{}", format_special_workspace_ident(_0))]
    Special(Option<&'a str>),
}"""

new_enum = """pub enum WorkspaceIdentifierWithSpecial<'a> {
    /// A regular workspace identifier
    #[display("{_0}")]
    Regular(WorkspaceIdentifier<'a>),
    /// The previous Workspace per monitor
    #[display("previous_per_monitor")]
    PreviousPerMonitor,
    /// The first available empty workspace
    #[display("{}", format!("empty{}", _0))]
    Empty(FirstEmpty),
    /// The special workspace
    #[display("special{}", format_special_workspace_ident(_0))]
    Special(Option<&'a str>),
}

impl<'a> From<WorkspaceIdentifier<'a>> for WorkspaceIdentifierWithSpecial<'a> {
    fn from(ident: WorkspaceIdentifier<'a>) -> Self {
        Self::Regular(ident)
    }
}"""

content = content.replace(old_enum, new_enum)

with open('src/dispatch.rs', 'w') as f:
    f.write(content)

