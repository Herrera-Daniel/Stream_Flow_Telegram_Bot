use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value::Null;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::requests::ResponseResult;
use tokio::net::windows::named_pipe::PipeEnd::Client;

#[derive(Deserialize, Debug)]
struct DWR_Flow {
    ResultList: Vec<DWR_Flow_Details>,
}

#[derive(Deserialize, Debug)]
struct DWR_Flow_Details {
    measValue: f64,
}

#[derive(Deserialize, Debug)]
struct USGS_Flow {
    value: TimeSeries,
}

#[derive(Deserialize, Debug)]
struct TimeSeries {
    timeSeries: Vec<Values>,
}

#[derive(Deserialize, Debug)]
struct Values {
    values: Vec<FlowValues>,
}

#[derive(Deserialize, Debug)]
struct FlowValues {
    value: Vec<FlowValue>,
}

#[derive(Deserialize, Debug)]
struct FlowValue {
    value: String,
}

#[derive(Deserialize, Debug)]
struct Station {
    ResultList: Vec<StationDetails>,
}

#[derive(Deserialize, Debug)]
struct StationDetails {
    station_name: String,
    abbrev: String,
}

#[tokio::main]
async fn main() {
    println!("We in here bruh");
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
            match stream.trim().to_lowercase().as_str() {
                "clear creek" => {
                    let res = client.get("https://waterservices.usgs.gov/nwis/iv/?format=json&sites=06719505&parameterCd=00060,00065&siteType=ST&siteStatus=all").send().await?;
                    let flow: USGS_Flow = res.json().await?;
                    bot.send_message(msg.chat.id, format!("Clear Creek: \nGolden: {:} cfs", &flow.value.timeSeries[0].values[0].value[0].value)).await?
                }
                "south platte" => {
                    let res1 = client.get("https://waterservices.usgs.gov/nwis/iv/?format=json&sites=06701900&parameterCd=00060,00065&siteType=ST&siteStatus=all").send().await?;
                    let res2 = client.get("https://dwr.state.co.us/Rest/GET/api/v2/telemetrystations/telemetrystation/?format=jsonforced&abbrev=PLABRUCO%2CPLASPICO%2CPLAHARCO%2CPLAGEOCO%2CPLACHECO%2CPLASPLCO%2CPLASTRCO").send().await?;
                    let deckers_flow: USGS_Flow = res1.json().await?;
                    let south_platte_flows: DWR_Flow = res2.json().await?;
                    bot.send_message(msg.chat.id, format!("South Platte: \nAbove Spinney: {aboveSpinney} cfs \nDream Stream: {dreamStream} cfs \nBelow Eleven Mile: {belowElevenMile} cfs \nBelow Cheeseman: {belowCheeseman} cfs \nDeckers: {deckers} cfs \nNorth Fork Confluence: {northForkCon} cfs \nBelow Strontia: {belowStrontia} cfs",
                                                          aboveSpinney = south_platte_flows.ResultList[3].measValue, dreamStream = south_platte_flows.ResultList[2].measValue, belowElevenMile = south_platte_flows.ResultList[1].measValue, belowCheeseman = south_platte_flows.ResultList[0].measValue, deckers = deckers_flow.value.timeSeries[0].values[0].value[0].value, northForkCon = south_platte_flows.ResultList[4].measValue, belowStrontia = south_platte_flows.ResultList[5].measValue)).await?
                }
                "big thompson" | "big t" => {
                    let res = client.get("https://dwr.state.co.us/Rest/GET/api/v2/telemetrystations/telemetrystation/?format=jsonforced&abbrev=BTBLESCO").send().await?;
                    let flow: DWR_Flow = res.json().await?;
                    bot.send_message(msg.chat.id, format!("Big Thompson: \nBelow Lake Estes: {:} cfs", &flow.ResultList[0].measValue)).await?
                }
                _ => {
                    bot.send_message(msg.chat.id, format!("No data found, use /streams to see supported streams.")).await?
                }
            }
        }
        Command::Streams => bot.send_message(msg.chat.id, "Clear Creek\nSouth Platte\nBig Thompson").await?,
    };

    Ok(())
}
