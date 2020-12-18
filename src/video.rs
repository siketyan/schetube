use chrono::{DateTime, TimeZone, Utc};

pub struct Video {
    pub id: String,
    pub title: String,
    pub thumbnail_url: String,
    pub scheduled_at: DateTime<Utc>,
}

impl Video {
    pub(crate) fn new(id: &str, title: &str, thumbnail_url: &str, scheduled_at: u64) -> Self {
        Self {
            id: String::from(id),
            title: String::from(title),
            thumbnail_url: String::from(thumbnail_url),
            scheduled_at: Utc.timestamp(scheduled_at as i64, 0),
        }
    }
}
