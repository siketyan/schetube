mod api;
mod client;
mod video;

use chrono::Local;
use std::env::args;
use std::error::Error;

use crate::api::response_to_videos;
use crate::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = args().collect::<Vec<_>>();
    if arguments.len() < 2 {
        return Ok(println!(
            "Usage: {} [Channel ID]",
            arguments.get(0).unwrap_or(&String::from("./schetube"))
        ));
    }

    let channel_id = arguments.get(1).unwrap();
    let client = Client::build().await?;
    let response = client.fetch_upcoming_live_streams(channel_id).await?;
    let videos =
        response_to_videos(response).expect("Failed to determine videos from the response.");

    for video in videos {
        println!(
            "{}: {} ({})",
            video.id,
            video.title,
            video.scheduled_at.with_timezone(&Local)
        );
    }

    Ok(())
}
