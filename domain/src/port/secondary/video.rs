use crate::model::video::{Video, VideoLink};
use anyhow::Error;
use mockall::*;

#[automock]
pub trait VideoRepository: Sync {
    fn create_video(&self, link: VideoLink) -> Result<Video, Error>;
    fn find(&self, link: VideoLink) -> Result<Video, Error>;
}
