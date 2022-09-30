use std::io::Read;

use teloxide::prelude::*;
use teloxide::respond;
use serde::Deserialize;
use serde_json::value::RawValue;

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
    pretty_env_logger::init();
    println!("We in here bruh");

    let bot = Bot::from_env().auto_send();
    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let client = reqwest::Client::new();
        let clear_creek_url = format!("https://dwr.state.co.us/Rest/GET/api/v2/surfacewater/surfacewatertsday/?format=json&abbrev=CLEGOLCO&min-measDate=-2days");
        let res = client.get(&clear_creek_url).send().await?;
        let flow: Flow = res.json().await?;
        bot.send_message(message.chat.id, flow.ResultList[0].value.to_string()).await?;
        respond(())
    }).await;
}
