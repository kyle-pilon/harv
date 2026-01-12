/// Represents a captured thought in the system
#[derive(Debug)]
pub struct CapturedThought {
    pub id: String,
    pub raw_input: String,
    pub captured_at: String,         // ISO 8601 format
    pub created_at: String,          // ISO 8601 format
    pub current_state: ThoughtState,
}


/// The possible states a thought can be in
#[derive(Debug)]
pub enum ThoughtState {
    Raw,
    Transformed,
    Exported,
    NeedsReview,
    Archived,
}

impl CapturedThought {
    /// Create a new thought with Raw state
    pub fn new(raw_input: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            raw_input,
            captured_at: now.clone(),
            created_at: now,
            current_state: ThoughtState::Raw,
        }
    }
}

impl ThoughtState {
    /// Convert state to string for database storage
    pub fn as_str(&self) -> &str {
        match self {
            ThoughtState::Raw => "Raw",
            ThoughtState::Transformed => "Transformed",
            ThoughtState::Exported => "Exported",
            ThoughtState::NeedsReview => "NeedsReview",
            ThoughtState::Archived => "Archived",
        }
    }
}