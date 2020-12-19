use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;

use crate::api::ApiResponse;

#[derive(Debug)]
struct ClientBuildError;

impl Display for ClientBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to build a client.")
    }
}

impl Error for ClientBuildError {}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn create(name: &str, version: &str) -> Result<Self, Box<dyn Error>> {
        let mut builder = reqwest::Client::builder();
        let mut headers = HeaderMap::new();

        headers.insert("X-YouTube-Client-Name", HeaderValue::from_str(name)?);
        headers.insert("X-YouTube-Client-Version", HeaderValue::from_str(version)?);
        builder = builder.default_headers(headers);

        Ok(Self::new(builder.build()?))
    }

    #[allow(clippy::clone_double_ref)]
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let html = reqwest::get("https://www.youtube.com/")
            .await?
            .text()
            .await?;

        let pattern = Regex::new(r#""clientVersion":"([\d.]+)""#)?;
        let captures = pattern.captures(&html).unwrap();
        let version = captures.get(1).ok_or(ClientBuildError {})?.as_str();

        Self::create("1", version.clone())
    }

    pub async fn fetch_upcoming_live_streams(
        &self,
        channel_id: &str,
    ) -> Result<ApiResponse, Box<dyn Error>> {
        let url = format!(
            "https://www.youtube.com/channel/{}/videos?view=2&live_view=502&pbj=1",
            channel_id
        );

        Ok(self.client.get(&url).send().await?.json().await?)
    }
}
