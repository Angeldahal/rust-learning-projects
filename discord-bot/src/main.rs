use std::env;
use serenity::client::{Client, ClientBuilder};
use serenity::prelude::GatewayIntents;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
=> Post in the <#1331261628104183980> channel and other humans will assist you.

? Looking for the Code of Conduct?
=> Here it is: <https://opensource.facebook.com/code-of-conduct> 

? Something wrong?
=> You can flag an admin with @admin

I hope that resolves your issue!
-- Helpbot

";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Message kind: {:?} ", msg.kind);
        println!("Received Message {} : {}", msg.content.len(), msg.content);
        if msg.content.trim() == HELP_COMMAND{
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.trim() == "Angel" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Yes, your lord is here.").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
