# Message

## Message Structure
Every message has this consistent structure:
```ts
{
  info: MessageV2.Info,      // Different for user vs assistant
  parts: MessageV2.Part[]    // Same type for both
}
```
## Info Types (Different)

### User Info (MessageV2.User):
```ts
{
  id: string,
  sessionID: string,
  role: "user",
  time: { created: number }
}
```

### Assistant Info (MessageV2.Assistant):
```ts

{
  id: string,
  sessionID: string,
  role: "assistant",
  time: { created: number, completed?: number },
  modelID: string,
  providerID: string,
  system: string[],
  cost: number,
  tokens: { ... },
  path: { ... },
  error?: ErrorType,
  // ... much more metadata
}
```

## Parts Type (Same)
Both user and assistant messages use the exact same MessageV2.Part union type:

```ts
type Part =
  | TextPart           // Plain text
  | ToolPart           // Tool calls/results
  | FilePart           // File attachments
  | ReasoningPart      // Thinking blocks
  | AgentPart          // Sub-agent calls
  | StepStartPart      // Execution boundaries
  | StepFinishPart     // Execution boundaries
  | SnapshotPart       // File snapshots
  | PatchPart          // File diffs
```

