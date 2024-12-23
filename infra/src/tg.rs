mod endpoints;

use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    dptree,
    types::Update,
};

use endpoints::video_handler;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

pub fn handle_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry().branch(Update::filter_message().endpoint(video_handler))
}
