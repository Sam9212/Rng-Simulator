use crate::utils::rng::{get_rarity_name, get_class_name};

use crate::{Context, Error, utils::database::retrieve_database, utils::database::get_exp_requirement};

#[poise::command(prefix_command, aliases("inv"))]
pub async fn profile(ctx: Context<'_>) -> Result<(), Error> {
    let id = ctx.author().id.as_u64();

    let db_guard = ctx.data().file_lock.lock().await;
    let db: std::collections::HashMap<u64, crate::utils::database::User> = retrieve_database(db_guard.as_str());



    let usr = match db.get(id){
        Some(user) => user,
        None => {
            ctx.say("user doesn't exist").await?;
            return Ok(());
        }
    };


    let handle = ctx.send(|cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title(format!("Showing **{}'s** profile", ctx.author().name));
            em.field("Wealth", format!("\
            ```ini\n\
            Money: {:.2}\n\
            Inventory: {:.2}\n\
            Networth: {:.2}```",usr.balance, usr.inventory_value(), usr.balance+ usr.inventory_value()) , true);

            em.field("Levels", format!("\
            ```ini\n\
            Level: {}\n\
            XP: {:.2}/{:.2}\n\
            Total XP: {}```", usr.level(0f64), usr.experience, get_exp_requirement(usr.level(0f64)+1), "total xp"), true);
            for i in 0..usr.inventory.len(){
                em.field(format!("Item {}", i+1), format!("\
                ```ini\n\
                [Rarity]; {} ({:.2})\n\
                [Quality]; {:.2}\n\
                [Class]; {} ({:.2})\n\
                [Total Value]; {:.2}```", get_rarity_name(&usr.inventory[i].rarity) ,usr.inventory[i].rarity as f64/1_000_000f64, usr.inventory[i].quality, get_class_name(&usr.inventory[i].class), usr.inventory[i].class as f64/1_000_000f64, usr.inventory[i].value())
                , false);

                if i == 2{
                    break;
                }
            }
            em
        })
    }).await?;

    Ok(())
}