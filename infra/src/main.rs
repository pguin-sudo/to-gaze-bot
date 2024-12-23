use teloxide::Bot;

use infra::config::Config;
use infra::startup::run;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = Config::new();

    let bot = Bot::new(config.telegram.token);

    run(bot).await;
}
