mod commands;

use std::env;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "hello" => commands::hello::run(&command.data.options),
                _ => "Not a command".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Hello {}!", msg.author))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if reaction.user_id == Some(ctx.cache.current_user_id()) {
            return;
        }

        let msg = reaction
            .message(&ctx.http)
            .await
            .expect("Error getting reaction message.");
        if let Err(why) = msg.react(&ctx.http, reaction.emoji).await {
            println!("Error reacting to message: {:?}", why);
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if reaction.user_id == Some(ctx.cache.current_user_id()) {
            return;
        }

        let msg = reaction
            .message(&ctx.http)
            .await
            .expect("Error getting reaction message.");

        for message_reaction in &msg.reactions {
            if message_reaction.count == 1 && message_reaction.me == true {
                let reaction_type = message_reaction.reaction_type.clone();
                if let Err(why) = msg.delete_reaction_emoji(&ctx.http, reaction_type).await {
                    println!("Error deleting reaction: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_ids_env_var: String =
            env::var("GUILD_IDS").expect("Expected GUILD_IDS in environment");
        let guild_ids = guild_ids_env_var.split(',');

        for id in guild_ids {
            let int_id: u64 = id.parse().unwrap();
            let guild_id = GuildId(int_id);

            let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::hello::register(command))
            })
            .await;

            match commands {
                Ok(_) => return,
                Err(e) => println!("Error when setting application commands: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment :(");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error {:?}", why);
    }
}
