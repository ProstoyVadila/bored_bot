use std::str::FromStr;
use teloxide::{prelude::*, types::Update, utils::command::BotCommands};

extern crate pretty_env_logger;
use crate::{api, config, utils};

// type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
// type HandlerResult = ResponseResult<()>;
// type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
// type Schema = UpdateHandler<Box<dyn Error + Send + Sync + 'static>>;
type HandlerResult = Result<(), teloxide::RequestError>;

pub async fn run(config: config::Config) -> HandlerResult {
    let bot = Bot::new(config.token);

    info!("Initing commands...");
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        // .branch(
        //     dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
        //     .endpoint(|bot: Bot, msg: Message| async move {
        //         default_group_handler(bot, msg).await?;
        //         Ok(())
        //     }),
        // )
        .branch(Update::filter_message().endpoint(random_handler));

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Can't set commands");

    info!("Starting mr bored_bot...");
    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Welcome, bored person!")]
    Start,
    #[command(description = "Help info")]
    Help,
    #[command(description = "get a random activity advice")]
    Random,
    #[command(
        description = "get an activity advice by one of the types: (type one of these: education, recreational, social, diy, charity, cooking, relaxation, music, busywork)"
    )]
    Type { activity_type: String },
    #[command(description = "get an activity advice by participants (type number more than 0)")]
    Members { members: u8 },
    #[command(description = "get an activity advice by price (type number between 0 and 1)")]
    Price { price: f32 },
}

// async fn send_msg_if_err(bot: Bot, msg: Message, err: Box<dyn Error>) -> HandlerResult {
//     let err_msg = match err {
//         err if err.is::<api::ActivityTypeNotFound>() => format!(
//             "Sorry I can't find any activity with this type.\nPlease try one of these: {}",
//             api::ActivityType::Diy.get_all()
//         ),
//         err if err.is::<api::ActivityNotFound>() => {
//             "Sorry I can't find any activity with this params.\nPlease try another one".to_string()
//         }
//         _ => "Sorry I something went wrong. Please try a little bit later".to_string(),
//     };
//     bot.send_message(msg.chat.id, err_msg).send().await?;
//     Ok(())
// }

async fn commands_handler(msg: Message, bot: Bot, cmd: Command) -> HandlerResult {
    match cmd {
        Command::Start => start_handler(bot, msg).await,
        Command::Help => help_handler(bot, msg).await,
        Command::Random => random_handler(bot, msg).await,
        Command::Type { activity_type } => activity_type_handler(bot, msg, activity_type).await,
        Command::Members { members } => members_handler(bot, msg, members).await,
        Command::Price { price } => price_handler(bot, msg, price).await,
    }
    .map_err(From::from)
}

async fn start_handler(bot: Bot, msg: Message) -> HandlerResult {
    let hi_msg =
        "Welcome, bored person!\nI can suggest some random activities to you. Just type /random"
            .to_string();
    let usage = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, format!("{}\n\n{}", hi_msg, usage))
        .send()
        .await?;
    Ok(())
}

async fn help_handler(bot: Bot, msg: Message) -> HandlerResult {
    let usage = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, usage).send().await?;
    Ok(())
}

async fn random_handler(bot: Bot, msg: Message) -> HandlerResult {
    let activity = match api::get_random().await {
        Ok(activity) => activity,
        Err(_) => {
            let err_msg = "Sorry I something went wrong. Please try a little bit later".to_string();
            bot.send_message(msg.chat.id, err_msg).send().await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, activity.get_pretty_msg())
        .send()
        .await?;
    Ok(())
}

async fn members_handler(bot: Bot, msg: Message, members: u8) -> HandlerResult {
    if members <= 0 {
        let err_msg =
            "Sorry I can't find any activity with this params.\nPlease try another one".to_string();
        bot.send_message(msg.chat.id, err_msg).send().await?;
        return Ok(());
    }
    let activity = match api::get_by_participants(members).await {
        Ok(activity) => activity,
        Err(_) => {
            let err_msg =
                "Sorry I can't find any activity with this params.\nPlease try another one"
                    .to_string();
            bot.send_message(msg.chat.id, err_msg).send().await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, activity.get_pretty_msg())
        .send()
        .await?;
    Ok(())
}

async fn price_handler(bot: Bot, msg: Message, price: f32) -> HandlerResult {
    let activity = match api::get_by_price(price).await {
        Ok(activity) => activity,
        Err(_) => {
            let err_msg =
                "Sorry I can't find any activity with this params.\nPlease try another one"
                    .to_string();
            bot.send_message(msg.chat.id, err_msg).send().await?;
            return Ok(());
        }
    };
    bot.send_message(msg.chat.id, activity.get_pretty_msg())
        .send()
        .await?;
    Ok(())
}

async fn activity_type_handler(bot: Bot, msg: Message, activity_type: String) -> HandlerResult {
    let activity_type = utils::capitalze(activity_type);
    let activity_type = match api::ActivityType::from_str(&activity_type) {
        Ok(activity_type) => activity_type,
        Err(_) => {
            let err_msg = format!(
                "Sorry I can't find any activity with this type.\nPlease try one of these: {}",
                api::ActivityType::Diy.get_all()
            );
            bot.send_message(msg.chat.id, err_msg).send().await?;
            return Ok(());
        }
    };
    let activity = api::get_by_type(activity_type).await?;
    bot.send_message(msg.chat.id, activity.get_pretty_msg())
        .send()
        .await?;
    Ok(())
}

// async fn default_group_handler(bot: Bot, msg: Message) -> HandlerResult {
//     // TODO: figure out how to get group members count
//     let members = 4;
//     let activity = api::get_by_participants(members).await?;
//     bot.send_message(msg.chat.id, activity.get_pretty_msg())
//         .send()
//         .await?;
//     Ok(())
// }
