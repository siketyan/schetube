use serde::Deserialize;

use crate::channel::Channel;
use crate::video::Video;

#[derive(Deserialize)]
struct Thumbnail {
    width: u32,
    height: u32,
    url: String,
}

#[derive(Deserialize)]
struct ThumbnailCollection {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize)]
struct Run {
    text: String,
}

#[derive(Deserialize)]
struct Title {
    runs: Vec<Run>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpcomingEventData {
    start_time: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridVideoRenderer {
    video_id: String,
    title: Title,
    thumbnail: ThumbnailCollection,
    upcoming_event_data: UpcomingEventData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridItem {
    grid_video_renderer: GridVideoRenderer,
}

#[derive(Deserialize)]
struct GridRenderer {
    items: Vec<GridItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemSectionEntry {
    grid_renderer: GridRenderer,
}

#[derive(Deserialize)]
struct ItemSectionRenderer {
    contents: Vec<ItemSectionEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SectionListEntry {
    item_section_renderer: ItemSectionRenderer,
}

#[derive(Deserialize)]
struct SectionListRenderer {
    contents: Vec<SectionListEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize)]
struct TabRenderer {
    content: Option<Content>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tab {
    tab_renderer: Option<TabRenderer>,
}

#[derive(Deserialize)]
struct TwoColumnBrowseResultsRenderer {
    tabs: Vec<Tab>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contents {
    two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize)]
struct Avatar {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelMetadataRenderer {
    title: String,
    avatar: Avatar,
    description: String,
    keywords: String,
    channel_url: String,
    external_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    channel_metadata_renderer: ChannelMetadataRenderer,
}

#[derive(Deserialize)]
struct Response {
    contents: Contents,
    metadata: Metadata,
}

#[derive(Deserialize)]
pub struct Entry {
    response: Option<Response>,
}

pub type ApiResponse = Vec<Entry>;

pub fn response_to_videos(response: &ApiResponse) -> Option<Vec<Video>> {
    response
        .last()?
        .response
        .as_ref()?
        .contents
        .two_column_browse_results_renderer
        .tabs
        .get(1)?
        .tab_renderer
        .as_ref()?
        .content
        .as_ref()?
        .section_list_renderer
        .contents
        .first()?
        .item_section_renderer
        .contents
        .first()?
        .grid_renderer
        .items
        .iter()
        .map(|item| {
            Some(Video::new(
                &item.grid_video_renderer.video_id,
                &item.grid_video_renderer.title.runs.first().unwrap().text,
                &item
                    .grid_video_renderer
                    .thumbnail
                    .thumbnails
                    .iter()
                    .max_by_key(|&t| t.width * t.height)?
                    .url,
                item.grid_video_renderer
                    .upcoming_event_data
                    .start_time
                    .parse()
                    .ok()?,
            ))
        })
        .collect::<Option<Vec<_>>>()
}

pub fn response_to_channel(response: &ApiResponse) -> Option<Channel> {
    let metadata = &response
        .last()?
        .response
        .as_ref()?
        .metadata
        .channel_metadata_renderer;

    Some(Channel::new(
        &metadata.external_id,
        &metadata.title,
        &metadata.description,
        &metadata.channel_url,
        &metadata.avatar.thumbnails.first()?.url,
        metadata.keywords.split(" ").collect(),
    ))
}
