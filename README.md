# schetube
A library to fetch upcoming live streams from a YouTube channel.

## 📦 Installation
```toml
[dependencies]
schetube = "0.1"
```

## 💚 Example
```
$ cd ./example
$ cargo run [Channel ID]
```

## 🔌 API
```rust
pub async fn fetch_upcoming_videos(channel_id: &str) -> Result<Vec<Video>, Box<dyn Error>>;
```
