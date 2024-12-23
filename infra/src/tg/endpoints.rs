use teloxide::{
    prelude::{Bot, Requester},
    types::{InputFile, Message},
};

use super::HandlerResult;
use crate::state::VIDEO_COMMAND_PORT;
use domain::{model::video::VideoLink, port::primary::command::VideoCommandPort};

pub async fn video_handler(bot: Bot, message: Message) -> HandlerResult {
    let response = VIDEO_COMMAND_PORT
        .get_response(VideoLink::new(message.text().unwrap()).unwrap())
        .await;

    match response.video {
        Some(bytes) => {
            bot.send_video(message.chat.id, InputFile::memory(bytes))
                .await?;
        }
        None => {
            bot.send_message(message.chat.id, response.message).await?;
        }
    }

    Ok(())
}
