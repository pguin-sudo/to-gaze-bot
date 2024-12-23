use crate::model::video::VideoLink;
use crate::port::primary::command::{VideoCommandPort, VideoResponse};
use crate::port::secondary::video::VideoRepository;

pub struct VideoCommandUseCase {
    pub video_repository: Box<dyn VideoRepository>,
}

impl VideoCommandPort for VideoCommandUseCase {
    async fn get_response(&self, link: VideoLink) -> VideoResponse {
        let result = self.video_repository.create_video(link);
        match result {
            Ok(video) => VideoResponse::new(Some(video.video_bytes), &video.title.to_string()),
            Err(e) => VideoResponse::new(None, &format!("Внутренняя ошибка: ```{}```", e)),
        }
    }
}

impl VideoCommandUseCase {
    pub fn new(video_repository: Box<dyn VideoRepository>) -> VideoCommandUseCase {
        VideoCommandUseCase { video_repository }
    }
}
