use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::user::User;
use serenity::utils::MessageBuilder;

pub fn run(options: &[CommandDataOption]) -> String {
    if options.len() != 2 {
        return "Got incorrect number of options".to_string();
    }

    let user_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected User object");

    let msg_option = options
        .get(1)
        .expect("Expected message option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let user: &User;
    let msg: String;

    if let CommandDataOptionValue::User(user_obj, _) = user_option {
        user = user_obj;
    } else {
        return "Missing User".to_string();
    }

    if let CommandDataOptionValue::String(msg_string) = msg_option {
        msg = format!("{}\t", msg_string);
    } else {
        return "Missing Message".to_string();
    }

    return MessageBuilder::new().push(msg).mention(user).build();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("hello")
        .description("Greet a user.")
        .create_option(|option| {
            option
                .name("user")
                .description("User to greet")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("message")
                .description("Greeting")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
