use std::collections::HashMap;

use serde::Deserialize;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::requests::ResponseResult;

#[derive(Deserialize, Debug)]
struct Flow {
    ResultList: Vec<FlowDetails>,
}

#[derive(Deserialize, Debug)]
struct FlowDetails {
    value: f64,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Get the flows for a river or stream. (/Streams) To get streams.")]
    Flow(String),
    #[command(description = "Get the currently supported Streams.")]
    Streams,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Flow(stream) => {
            let client = reqwest::Client::new();
            let res = client.get("https://dwr.state.co.us/Rest/GET/api/v2/surfacewater/surfacewatertsday/?format=json&abbrev=CLEGOLCO&min-measDate=-2days").send().await?;
            let flow: Flow = res.json().await?;
            bot.send_message(msg.chat.id, format!("Clear Creek at Golden: {} cfs {stream}", flow.ResultList[0].value.to_string())).await?
        }
        Command::Streams => bot.send_message(msg.chat.id, "These are the streams").await?,
    };

    Ok(())
}
