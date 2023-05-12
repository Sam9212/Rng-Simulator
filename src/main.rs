mod commands;
mod utils;

use std::{fs::File, collections::HashMap};
use dotenv::dotenv;
use commands::{
    ping::ping,
    ping::read_db,
    start::start,
};
use poise::{
    futures_util::lock::Mutex,
    serenity_prelude as serenity,
    FrameworkError::{Setup, Command},
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    file_lock: Mutex<String>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // Handles all errors
    match error {
        Setup { error, .. } => panic!("failed to start bot: {:?}", error),
        Command { error, ctx } => println!("Error with command `{}`: {:?}", ctx.command().name, error),
        _ => {}
    }
}


#[tokio::main]
async fn main() {
    // Using .env for token
    dotenv().ok();

    // Setting up database in case it doesn't exist or is empty
    match File::open("db") {
        Ok(f) => {
            let size = f.metadata()
            .expect("Issue getting database metadata")
            .len();

            if size == 0 {
                utils::database::write_database("db", HashMap::new());
            }
        }
        Err(_) => {
            utils::database::write_database("db", HashMap::new());
        }
    }


    let options = poise::FrameworkOptions {
        commands: vec![
            ping(),
            start(),
            read_db(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("r.".into()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            std::env::var("TOKEN").expect("get token in .env pls"),
        )
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                println!("Bot logged in as `{}`", ready.user.name);
                Ok(Data { file_lock: Mutex::from("db".to_owned()) })
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .run()
        .await
        .unwrap();
}
