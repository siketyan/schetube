mod api;
mod client;
mod video;

use client::Client;
use std::error::Error;

use crate::api::response_to_videos;
use crate::video::Video;

pub async fn fetch_upcoming_videos(channel_id: &str) -> Result<Vec<Video>, Box<dyn Error>> {
    let client = Client::build().await?;
    let response = client.fetch_upcoming_live_streams(channel_id).await?;
    let videos = response_to_videos(response).ok_or("Failed to transform the response.")?;

    Ok(videos)
}
