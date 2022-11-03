use roux::Subreddit;
use roux::util::{FeedOption, TimePeriod};

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn corgi(ctx: &Context, msg: &Message) -> CommandResult {
    let subreddit = Subreddit::new("corgi");
    let options = FeedOption::new().period(TimePeriod::Today);
    let top = subreddit.top(25, Some(options)).await;

    for listing in &top.unwrap().data.children {
        let url = listing.data.url.as_ref().unwrap().clone();

        if url.ends_with(".jpg") || url.contains("v.redd.it") {
            let response = MessageBuilder::new()
                .push(url).build();
            msg.channel_id.say(&ctx.http, &response).await?;
        }
    }

    Ok(())
}