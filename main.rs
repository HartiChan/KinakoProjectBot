// coding=utf-8

// Copyright (c) 2020 HartiChan

// 888    d8P  d8b                   888                       .d8888b.                888          
// 888   d8P   Y8P                   888                      d88P  Y88b               888          
// 888  d8P                          888                      888    888               888          
// 888d88K     888 88888b.   8888b.  888  888  .d88b.         888         .d88b.   .d88888  .d88b.  
// 8888888b    888 888 "88b     "88b 888 .88P d88""88b        888        d88""88b d88" 888 d8P  Y8b 
// 888  Y88b   888 888  888 .d888888 888888K  888  888 888888 888    888 888  888 888  888 88888888 
// 888   Y88b  888 888  888 888  888 888 "88b Y88..88P        Y88b  d88P Y88..88P Y88b 888 Y8b.     
// 888    Y88b 888 888  888 "Y888888 888  888  "Y88P"          "Y8888P"   "Y88P"   "Y88888  "Y8888  

use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }
    Ok(())
}