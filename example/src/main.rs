use chrono::Local;
use std::env::args;
use std::error::Error;

use schetube::fetch_upcoming_videos;

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
    let (channel, videos) = fetch_upcoming_videos(channel_id).await?;

    println!("{}: {}", channel.id, channel.name);

    for video in videos {
        println!(
            "\t- {}: {} ({})",
            video.id,
            video.title,
            video.scheduled_at.with_timezone(&Local)
        );
    }

    Ok(())
}
