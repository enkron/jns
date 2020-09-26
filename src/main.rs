use std::env;

use futures::StreamExt;
use telegram_bot::*;

async fn tst_msg(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply("hello manny")).await?;
    Ok(())
}

async fn tst(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/jnk" => tst_msg(api, message).await?,
            _ => (),
        },
        _ => (),
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    // Create a new Api instance
    let api = Api::new(token);

    // Create a stream which produces updates from the Telegram server.
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            tst(api.clone(), message).await?;
        }
    }

    Ok(())
}
