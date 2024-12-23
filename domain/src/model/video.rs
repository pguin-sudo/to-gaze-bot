use std::ops::Deref;

use anyhow::Error;

pub type VideoID = String;
pub type Title = String;
pub type Description = String;
pub type VideoBytes = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoLink(String);

impl VideoLink {
    pub fn new(raw: &str) -> Result<Self, Error> {
        Ok(VideoLink(raw.to_string()))
    }
}

impl Deref for VideoLink {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    pub upc: VideoID,
    pub title: Title,
    pub description: Description,
    pub video_bytes: VideoBytes,
    pub video_link: VideoLink,
}

impl Video {
    pub fn new(
        upc: VideoID,
        title: Title,
        description: Description,
        video_bytes: VideoBytes,
        video_link: VideoLink,
    ) -> Self {
        Video {
            upc,
            title,
            description,
            video_bytes,
            video_link,
        }
    }
}
