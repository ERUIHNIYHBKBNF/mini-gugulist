use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub test: String,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(test: String) -> Task {
        Task {
            test,
            created_at: Utc::now(),
        }
    }
}