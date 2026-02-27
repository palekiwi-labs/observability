# Event Types

## Available event types:

1. "session.idle" - When a session becomes idle
• Properties: { sessionID: string }
2. "session.updated" - When session information is updated
• Properties: { info: Session }
3. "session.deleted" - When a session is deleted
• Properties: { info: Session }
4. "session.error" - When a session encounters an error
• Properties: { sessionID?: string, error?: ProviderAuthError | UnknownError | MessageOutputLengthError | MessageAbortedError }
5. "message.updated" - When a message is updated
• Properties: { info: Message }
6. "message.removed" - When a message is removed
• Properties: { sessionID: string, messageID: string }
7. "message.part.updated" - When a message part is updated
• Properties: { part: Part }
8. "message.part.removed" - When a message part is removed
• Properties: { sessionID: string, messageID: string, partID: string }
9. "permission.updated" - When permissions are updated
• Properties: Permission object
10. "permission.replied" - When a permission is replied to
• Properties: { sessionID: string, permissionID: string, response: string }
11. "file.edited" - When a file is edited
• Properties: { file: string }
12. "server.connected" - When server connects
• Properties: { [key: string]: unknown }
13. "installation.updated" - When installation is updated
• Properties: { version: string }
14. "lsp.client.diagnostics" - LSP diagnostics events
• Properties: { serverID: string, path: string }

## Other Plugin Hooks

Beyond the event hook, plugins can also implement:
• "chat.message" - Called when a new message is received
• "chat.params" - Modify parameters sent to LLM (temperature, topP, etc.)
• "permission.ask" - Handle permission requests
• "tool.execute.before" - Called before tool execution
• "tool.execute.after" - Called after tool execution
• config - Handle configuration updates
• auth - Custom authentication providers

## Key Event Types for Agent Responses
 • "message.updated": Fired when the entire message is updated/completed
 • "message.part.updated": Fired as individual parts (text, tool calls) are generated
 • "session.updated": Fired when session metadata changes
