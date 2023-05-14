use tokio::time::{sleep, Duration};
use crate::{
    Context, 
    Error, 
    utils::{
        database::{
            retrieve_database,
            write_database,
            Item,
        },
        rng::{
            calculate_roll,
            get_rarity_name,
            get_class_name, get_rarity_colour,
        },
    }
};

fn general_check(ctx: &Context<'_>) -> bool {
    ctx.channel_id() == 1051547704314052628
}

#[poise::command(prefix_command, user_cooldown = 6, aliases("r"))]
pub async fn roll(ctx: Context<'_>) -> Result<(), Error> {
    if !general_check(&ctx) {
        ctx.send(|cr| {
            cr.reply(true);
            cr.embed(|em| {
                em.title("Dude");
                em.description("```ini\n[Information]\nNot to be a Discord Mod, but don't roll in general. Please```");
                em.color(0xFF0000)
            })
        }).await?;
        return Ok(());
    }

    let db_guard = ctx.data().file_lock.lock().await;
    let database = retrieve_database(db_guard.as_str());
    drop(db_guard);
    let user = match database.get(&ctx.author().id.as_u64()) {
        Some(user) => user,
        None => {
            ctx.send(|cr| {
                cr.reply(true);
                cr.embed(|em| {
                    em.title("Account not found!");
                    let description_text = format!("```ini\n\
                        [Information]\n\
                        You do not appear to have an RNG Simulator account. \
                        In order to create an account, you must run the \
                        `{}start` command to begin your journey into virginity.\n\
                        ```\
                    ", ctx.prefix());
                    em.description(description_text)
                })
            }).await?;
            return Ok(());
        }
    };

    let new_item = Item {
        class: calculate_roll(user.cluck_level, user.experience),
        rarity: calculate_roll(user.rluck_level, user.experience),
        quality: fastrand::f64(),
    };
    let gained_xp = fastrand::u32(30..=50);
    let lvlup_text = if user.level(gained_xp as f64) > user.level(0.0) {
        "LEVEL UP!"
    } else {
        ""
    };

    let handle = ctx.send(|cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title(format!("Rolling {}'s Item", ctx.author().name));
            em.description("\
                ```ini\n\
                [Rarity]; UNKNOWN\n\
                [Quality]; UNKNOWN\n\
                [Class]; UNKNOWN\n\
                [Total Value]; UNKNOWN\n\
                ```\
            ")
        })
    }).await?;

    sleep(Duration::from_secs(2)).await;

    handle.edit(ctx, |cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title(format!("Rolling {}'s Item", ctx.author().name));
            em.description(format!("\
                ```ini\n\
                [Rarity]; {} ({:.2})\n\
                [Quality]; UNKNOWN\n\
                [Class]; UNKNOWN\n\
                [Total Value]; UNKNOWN\n\
                ```",
                get_rarity_name(&new_item.rarity),
                new_item.rarity as f64 / 1_000_000f64
            ))
        })
    }).await?;

    sleep(Duration::from_secs(2)).await;

    handle.edit(ctx, |cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title(format!("Rolling {}'s Item", ctx.author().name));
            em.description(format!("\
                ```ini\n\
                [Rarity]; {} ({:.2})\n\
                [Quality]; {:.2}\n\
                [Class]; UNKNOWN\n\
                [Total Value]; UNKNOWN\n\
                ```",
                get_rarity_name(&new_item.rarity),
                new_item.rarity as f64 / 1_000_000f64,
                new_item.quality,
            ))
        })
    }).await?;

    sleep(Duration::from_secs(2)).await;

    handle.edit(ctx, |cr| {
        cr.reply(true);
        cr.embed(|em| {
            em.title(format!("Rolling {}'s Item", ctx.author().name));
            em.color(get_rarity_colour(get_rarity_name(&new_item.rarity)));
            em.description(format!("\
                ```ini\n\
                [Rarity]; {} ({:.2})\n\
                [Quality]; {:.2}\n\
                [Class]; {} ({:.2})\n\
                [Total Value]; ${:.2}\n\
                ```",
                get_rarity_name(&new_item.rarity),
                new_item.rarity as f64 / 1_000_000f64,
                new_item.quality,
                get_class_name(&new_item.class),
                new_item.class as f64 / 1_000_000f64,
                new_item.value()
            ))
        })
    }).await?;

    let db_guard = ctx.data().file_lock.lock().await;
    let mut database = retrieve_database(db_guard.as_str());
    database.get_mut(ctx.author().id.as_u64()).unwrap().inventory.push(new_item);
    write_database(db_guard.as_str(), database);
    Ok(())
}