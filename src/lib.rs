mod api;
mod channel;
mod client;
mod video;

use std::error::Error;

pub use crate::api::{response_to_channel, response_to_videos, ApiResponse};
pub use crate::channel::Channel;
pub use crate::client::Client;
pub use crate::video::Video;

pub async fn fetch_upcoming_videos(
    channel_id: &str,
) -> Result<(Channel, Vec<Video>), Box<dyn Error>> {
    let client = Client::build().await?;
    let response = client.fetch_upcoming_live_streams(channel_id).await?;
    let channel = response_to_channel(&response).ok_or("Failed to transform the response.")?;
    let videos = response_to_videos(&response).ok_or("Failed to transform the response.")?;

    Ok((channel, videos))
}
