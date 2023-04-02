use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::utils::MessageBuilder;

pub fn run(options: &[CommandDataOption]) -> String {
    if options.len() != 2 {
        return "Got incorrect number of options".to_string();
    }

    let user = &options.get(0);

    println!("Before Data:");
    println!("{:?}", user);
    // println!("{:?}", message);

    // return format!("{:?} {:?}", message, user);

    MessageBuilder::new()
        .push("testing")
        // .mention(user)
        .build()
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
