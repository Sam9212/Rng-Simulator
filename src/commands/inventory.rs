use std::fmt::format;

use poise::futures_util::future::ok;

use crate::{Context, Error, utils::database::retrieve_database, utils::database::User};

#[poise::command(prefix_command)]
pub async fn inv(ctx: Context<'_>) -> Result<(), Error> {
    let id = ctx.author().id.as_u64();

    let db_guard = ctx.data().file_lock.lock().await;
    let db: std::collections::HashMap<u64, crate::utils::database::User> = retrieve_database(db_guard.as_str());


    if ! db.contains_key(id){
        ctx.say("you do not have an inventory").await?;
        return Ok(());
    }

    let usr_inventory = &db.get(id).unwrap().inventory;
    ctx.say(format!("items: {:?}", usr_inventory)).await?;
    Ok(())
}