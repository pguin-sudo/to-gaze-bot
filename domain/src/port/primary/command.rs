use std::future::Future;

use crate::model::video::VideoLink;

pub struct VideoResponse {
    pub video: Option<Vec<u8>>,
    pub message: String,
}

impl VideoResponse {
    pub fn new(bytes: Option<Vec<u8>>, message: &str) -> Self {
        VideoResponse {
            video: bytes,
            message: message.to_string(),
        }
    }
}

pub trait VideoCommandPort {
    fn get_response(&self, link: VideoLink) -> impl Future<Output = VideoResponse>;
}
