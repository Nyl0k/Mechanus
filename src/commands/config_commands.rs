use serenity::{
    prelude::*,
    utils,
    builder::CreateEmbed,
    model::{
        prelude::*,
    },
    framework::standard::{
        Args,
    },
};

use std::fs;

use mechanus_bot::*;

//All commands normal members can use
pub fn config_help_op() ->  CreateEmbed {
    let mut e = create_embed();

    e.title("Commands list");
    e.field("&&config_help", "Displays this menu", false);
    e.field("&&initialize", "Creates a registry that holds user and server metadata, run this command first", false);
    e.field("&&uninitialize", "Deletes server registry", false);
    e.field("&&add_start_role <@role>", "Make a role automatically given to registered users", false);
    e.field("&&remove_start_role <@role>", "Remove a banned role", false);
    e.field("&&add_banned_role <@role>", "Make a role inaccessible to non-admins", false);
    e.field("&&remove_banned_role <@role>", "Remove a banned role", false);
    e.field("&&set_logging_channel <#channel>", "Set which channel messages are logged in", false);

    e
}

//Initialize the server, only needs to be run once when the bot is added
pub async fn initialize_op(ctx: &Context, msg: &Message) -> Result<(), String> {
    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();
    
    //Check if server has already been initialized
    let filepath = format!("registries/{}.json", *guild.id.as_u64());

    if let Ok(_) = fs::read_to_string(&filepath){
        return Err("Already initialized".to_string());
    };

    //Create server registry
    let mut server = Server::new();
    server.name = guild.name;
    server.id = *guild.id.as_u64();
    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not write to registry".to_string())
    }

    Ok(())
}


pub async fn uninitialize_op(ctx: &Context, msg: &Message) -> Result<(), String> {
    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    if let Err(_) = fs::remove_file(format!("registries/{}.json", guild.id.as_u64())){
        return Err("Could not find registry".to_string())
    }

    Ok(())
}

pub async fn add_start_role_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let role_name = args.single::<String>();
    if let Err(_) = role_name {
        return Err("Please supply a valid role name".to_string());
    }
    let role_name = role_name.unwrap();

    if let None = utils::parse_role(&role_name) { return Err("Invalid role name".to_string()); };
    let role_id = RoleId(utils::parse_role(&role_name[..]).unwrap());

    if server.start_roles.contains(&role_id){
        return Err("Role already a starting role".to_string());
    }

    server.start_roles.push(role_id);

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} added role {} to start roles", msg.author.name, role_name)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn remove_start_role_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let role_name = args.single::<String>();
    if let Err(_) = role_name {
        return Err("Please supply a valid role name".to_string());
    }
    let role_name = role_name.unwrap();

    if let None = utils::parse_role(&role_name) { return Err("Invalid role name".to_string()); };
    let role_id = RoleId(utils::parse_role(&role_name[..]).unwrap());

    if server.start_roles.contains(&role_id){
        server.start_roles.iter().position(|x|{ x == &role_id });
    } else {
        return Err("Unable to find role in starting roles".to_string());
    }

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from start roles", msg.author.name, role_name)).await{
        return Err(e);
    }


    Ok(())
}


pub async fn add_banned_role_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let role_name = args.single::<String>();
    if let Err(_) = role_name {
        return Err("Please supply a valid role name".to_string());
    }
    let role_name = role_name.unwrap();

    if let None = utils::parse_role(&role_name) { return Err("Invalid role name".to_string()); };
    let role_id = RoleId(utils::parse_role(&role_name[..]).unwrap());

    if server.banned_roles.contains(&role_id){
        return Err("Role already a starting role".to_string());
    }

    server.banned_roles.push(role_id);

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} added role {} to banned roles", msg.author.name, role_name)).await{
        return Err(e);
    }


    Ok(())
}

pub async fn remove_banned_role_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let role_name = args.single::<String>();
    if let Err(_) = role_name {
        return Err("Please supply a valid role name".to_string());
    }
    let role_name = role_name.unwrap();

    if let None = utils::parse_role(&role_name) { return Err("Invalid role name".to_string()); };
    let role_id = RoleId(utils::parse_role(&role_name[..]).unwrap());

    if server.banned_roles.contains(&role_id){
        server.banned_roles.iter().position(|x|{ x == &role_id });
    } else {
        return Err("Unable to find role in starting roles".to_string());
    }

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from banned roles", msg.author.name, role_name)).await{
        return Err(e);
    }


    Ok(())
}

pub async fn set_logging_channel_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let channel = args.single::<ChannelId>();
    if let Err(_) = channel {
        return Err("Invalid channel id".to_string());
    }
    let channel = channel.unwrap();

    server.log_channel = channel;

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} set channel {} as logging channel", msg.author.name, channel.name(&ctx).await.unwrap())).await{
        return Err(e);
    }

    Ok(())
}