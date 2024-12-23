use lazy_static::lazy_static;

use crate::db::in_memory::InMemoryVideoRepository;
use domain::usecases::video::VideoCommandUseCase;

lazy_static! {
    pub static ref VIDEO_COMMAND_PORT: VideoCommandUseCase =
        VideoCommandUseCase::new(Box::new(InMemoryVideoRepository::new()));
}
