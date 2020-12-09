use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
        Args,
    },
};

use crate::commands::{
    user_commands as uc,
    admin_commands as ac,
    config_commands as cc
};

/*
---------------USER COMMANDS---------------
*/

//Get user command list
#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(uc::help_op()); m}).await?;
    Ok(())
}

//Get some information about the bot
#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(uc::about_op()); m}).await?;
    Ok(())
}

//Get some information about the bot
#[command]
async fn user_info(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let embed = uc::user_info_op(&ctx, &msg, args).await;
    if let Err(e) = embed {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }
    let embed = embed.unwrap();
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(embed); m}).await?;
    Ok(())
}

//Get some information about the server
#[command]
async fn server_info(ctx: &Context, msg: &Message) -> CommandResult {
    let embed = uc::server_info_op(&ctx, &msg).await;
    if let Err(e) = embed {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }
    let embed = embed.unwrap();
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(embed); m}).await.unwrap();
    Ok(())
}

//Add a role
#[command]
async fn add_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = uc::user_role_op(&ctx, &msg, args, false).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

//Remove a role
#[command]
async fn remove_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = uc::user_role_op(&ctx, &msg, args, true).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
async fn roles_list(ctx: &Context, msg: &Message) -> CommandResult {
    let roles = uc::roles_list_op(&ctx, &msg).await;
    if let Err(e) = roles {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(());
    }
    let roles = roles.unwrap();

    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(roles); m}).await?;

    Ok(())
}

#[command]
async fn my_warns(ctx: &Context, msg: &Message) -> CommandResult {
    let warns = uc::my_warns_op(&msg).await;
    if let Err(e) = warns {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(());
    }
    let warns = warns.unwrap();

    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(warns); m}).await?;

    Ok(())
}

//Ping the bot
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, uc::ping_op()).await?;
    Ok(())
}

/*
---------------ADMIN COMMANDS---------------
*/

//Get admin command list
#[command]
#[required_permissions(ADMINISTRATOR)]
async fn admin_help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(ac::admin_help_op()); m}).await?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn register_user(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::register_user_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn deregister_user(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::deregister_user_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn mass_register(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = ac::mass_register_op(&ctx, &msg).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn give_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::role_op(&ctx, &msg, args, true).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn take_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::role_op(&ctx, &msg, args, false).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn purge(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::purge_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn user_list(ctx: &Context, msg: &Message) -> CommandResult {
    let output = ac::user_list_op(&msg).await;
    if let Err(e) = output {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }
    let output = output.unwrap();

    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(output); m}).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn warn(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::warn_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn get_warns(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::get_warns_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn forgive(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::forgive_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn forgive_all(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::forgive_all_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::kick_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = ac::ban_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

/*
---------------ADMIN COMMANDS---------------
*/

//Get admin command list
#[command]
#[required_permissions(ADMINISTRATOR)]
async fn config_help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(cc::config_help_op()); m}).await?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn initialize(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = cc::initialize_op(&ctx, &msg).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn uninitialize(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = cc::uninitialize_op(&ctx, &msg).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn add_start_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = cc::add_start_role_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn remove_start_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = cc::remove_start_role_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn start_roles_list(ctx: &Context, msg: &Message) -> CommandResult {
    let roles = cc::start_roles_list_op(&ctx, &msg).await;
    if let Err(e) = roles {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(());
    }
    let roles = roles.unwrap();

    msg.channel_id.send_message(&ctx.http, |m| {m.set_embed(roles); m}).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn add_allowed_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = cc::add_allowed_role_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn remove_allowed_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = cc::remove_allowed_role_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn set_logging_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = cc::set_logging_channel_op(&ctx, &msg, args).await {
        msg.channel_id.say(&ctx.http, e).await?;
        return Ok(())
    }

    msg.channel_id.say(&ctx.http, format!("Command confirmed")).await?;

    Ok(())
}