use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub enum TaskError {
    InvalidTimestampFormat,
    EmptyDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i64,
    pub details: String,
    pub timestamp: Option<DateTime<Utc>>,
}
