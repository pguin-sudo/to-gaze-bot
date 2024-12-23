use teloxide::prelude::{Bot, Dispatcher};

use crate::tg::handle_tree;

pub async fn run(bot: Bot) {
    Dispatcher::builder(bot, handle_tree())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
