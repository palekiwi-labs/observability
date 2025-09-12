# Observability Service - Implementation Context

## Overview
This service provides observability for agent interactions by tracking Sessions and Messages in a simplified CRUD design.

## Database Schema

### Sessions Table
Sessions represent agent conversation sessions.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,              // UUID
    pub project_id: String,      // From agent spec
    pub directory: String,       // From agent spec
    pub title: Option<String>,   // User-friendly title
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Messages Table
Messages belong to sessions and represent individual user/assistant exchanges.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: String,              // UUID
    pub session_id: String,      // Foreign key to sessions.id
    pub role: MessageRole,       // user, assistant
    pub content: String,         // JSON string of parts array
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>, // For assistant messages
    pub tokens_used: Option<i32>, // For assistant messages
    pub model_id: Option<String>, // For assistant messages
    pub summary: Option<String>, // Summary of the message content
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_role", rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}
```

## API Endpoints

### Sessions CRUD
- `GET /sessions` - List all sessions (with pagination)
- `GET /sessions/{id}` - Get session by ID
- `POST /sessions` - Create new session
- `PUT /sessions/{id}` - Update session
- `DELETE /sessions/{id}` - Delete session
- `POST /sessions/{id}/complete` - Mark session as completed

### Messages CRUD
- `GET /sessions/{session_id}/messages` - Get messages for session
- `GET /messages/{id}` - Get message by ID
- `POST /sessions/{session_id}/messages` - Create message in session
- `PUT /messages/{id}` - Update message
- `DELETE /messages/{id}` - Delete message

## Request/Response DTOs

### Session DTOs
```rust
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub project_id: String,
    pub directory: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionRequest {
    pub title: Option<String>,
}
```

### Message DTOs
```rust
#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub role: MessageRole,
    pub content: String, // JSON string
    pub model_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: Option<String>,
}


```

## Design Decisions

1. **Simplified Schema**: The models are simplified from the full agent specification to focus on essential observability data.

2. **JSON Content Storage**: Message content is stored as a JSON string to provide flexibility for different message part types without complex normalization.

3. **Foreign Key Relationship**: Messages reference sessions via `session_id` to maintain data integrity.

4. **Observability Focus**: Key metrics like tokens_used, cost, and timing are captured for analysis.

5. **Timestamp Tracking**: Sessions track creation and last update times.

6. **Separate Completion Endpoint**: Sessions can be marked as completed via dedicated endpoint for workflow tracking.
