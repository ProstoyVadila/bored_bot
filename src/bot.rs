use std::error::Error;
use teloxide::prelude::*;

use crate::{api, config};

// #[derive(BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "get some advice for a bored person")]
//     Help,
//     #[command(description = "get a random activity advice")]
//     Activity,
// }

// async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
//     match cmd {
//         Command::Help => {
//             let usage = Command::descriptions().to_string();
//             bot.send_message(msg.chat.id, usage).send().await?;
//         }
//         Command::Activity => {
//             let activity = api::BoredActivity::get_random().await.unwrap();
//             bot.send_message(msg.chat.id, activity.get_pretty())
//                 .send()
//                 .await?;
//         }
//     };
//     Ok(())
// }

pub async fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let bot = Bot::new(config.token);

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        log::info!("Sending an advice to bored person...");
        let activity = match api::BoredActivity::get_random().await {
            Ok(activity) => activity,
            Err(err) => {
                log::error!("Error getting an activity: {:?}", err);
                bot.send_message(message.chat.id, "Sorry I can't help you now")
                    .send()
                    .await?;
                return Ok(());
            }
        };

        match bot
            .send_message(message.chat.id, activity.get_pretty())
            .send()
            .await
        {
            Ok(msg) => log::info!("Sent a message: {:?}", msg),
            Err(err) => log::error!("Error sending a message: {:?}", err),
        };
        Ok(())
    })
    .await;
    Ok(())
}
