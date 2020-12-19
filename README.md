# schetube
![Rust](https://github.com/siketyan/schetube/workflows/Rust/badge.svg)

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
📝 List of structs and functions are available at [docs.rs](https://docs.rs/schetube/latest/schetube/) .

```rust
pub async fn fetch_upcoming_videos(channel_id: &str) -> Result<(Channel, Vec<Video>), Box<dyn Error>>;
pub fn response_to_videos(response: &ApiResponse) -> Option<Vec<Video>>;
pub fn response_to_channel(response: &ApiResponse) -> Option<Channel>;

impl Client {
    pub fn new(client: reqwest::Client) -> Self;
    pub fn create(name: &str, version: &str) -> Result<Self, Box<dyn Error>>;
    pub async fn build() -> Result<Self, Box<dyn Error>>;
    pub async fn fetch_upcoming_live_streams(&self, channel_id: &str) -> Result<ApiResponse, Box<dyn Error>>;
}
```
