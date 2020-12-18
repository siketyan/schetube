use chrono::{DateTime, Utc, TimeZone};

pub struct Video {
    pub id: String,
    pub title: String,
    pub scheduled_at: DateTime<Utc>,
}

impl Video {
    pub(crate) fn new(id: &str, title: &str, scheduled_at: u64) -> Self {
        Self {
            id: String::from(id),
            title: String::from(title),
            scheduled_at: Utc.timestamp(scheduled_at as i64, 0),
        }
    }
}
