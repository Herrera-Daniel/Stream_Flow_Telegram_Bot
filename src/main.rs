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
        let message_text = message.text().unwrap();
        if message_text.eq("!flow") {
            println!("{:}", message_text);
            let clear_creek_url = format!("https://dwr.state.co.us/Rest/GET/api/v2/surfacewater/surfacewatertsday/?format=json&abbrev=CLEGOLCO&min-measDate=-2days");
            let res = client.get(&clear_creek_url).send().await?;
            let flow: Flow = res.json().await?;
            let response = format!("Clear Creek at Golden: {} cfs", flow.ResultList[0].value.to_string());
            bot.send_message(message.chat.id, response).await?;
            respond(())
        } else {
            bot.send_message(message.chat.id, "Only valid commands please.

Ex.
!flow <Stream/River name>").await?;
            respond(())
        }
    }).await;
}
