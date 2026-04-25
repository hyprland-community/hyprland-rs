import re

with open('src/dispatch.rs', 'r') as f:
    content = f.read()

# Replace WorkspaceIdentifierWithSpecial with WorkspaceIdentifier
content = content.replace('WorkspaceIdentifierWithSpecial', 'WorkspaceIdentifier')

# Remove the old WorkspaceIdentifier and its Display impl
# Be very careful to match only the right section!
old_ws_pattern = re.compile(r'/// This enum is for identifying workspaces\n#\[derive\(Debug, Clone, Copy, PartialEq, Eq\)\]\npub enum WorkspaceIdentifier<\'a> \{.*?\n}\n\nimpl std::fmt::Display for WorkspaceIdentifier<\'_\> \{\n    fn fmt\(&self, f: &mut std::fmt::Formatter<\'_\>\) -> std::fmt::Result \{.*?\n    \}\n}\n', re.DOTALL)
content = old_ws_pattern.sub('', content)

with open('src/dispatch.rs', 'w') as f:
    f.write(content)

