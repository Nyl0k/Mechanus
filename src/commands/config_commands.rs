use serenity::{
    prelude::*,
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
    e.field("&&add_start_role <role name>", "Make a role automatically given to registered users", false);
    e.field("&&remove_start_role <role name>", "Remove a start role", false);
    e.field("&&start_roles_list", "Get list of start roles", false);
    e.field("&&add_allowed_role <role name>", "Make a role accessible to non-admins", false);
    e.field("&&remove_allowed_role <role name>", "Remove an allowed role", false);
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

pub async fn add_start_role_op(ctx: &Context, msg: &Message, args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let mut role: Option<RoleId> = None;

    for guild_role in &guild.roles{
        if guild_role.1.name == *args.rest().to_string() {
            role = Some(*guild_role.0);
        }
    }

    if let None = role { return Err("Could not find role".to_string()); }

    let role = role.unwrap();

    if server.start_roles.contains(&role){
        return Err("Role already a starting role".to_string());
    }

    server.start_roles.push(role);

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} added role {} to start roles", msg.author.name, role.to_role_cached(&ctx).await.unwrap().name)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn remove_start_role_op(ctx: &Context, msg: &Message, args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let mut role: Option<RoleId> = None;

    for guild_role in &guild.roles{
        if guild_role.1.name == *args.rest().to_string() {
            role = Some(*guild_role.0);
        }
    }

    if let None = role { return Err("Could not find role".to_string()); }

    let role = role.unwrap();

    if server.start_roles.contains(&role){
        server.start_roles.iter().position(|x|{ x == &role });
    } else {
        return Err("Unable to find role in starting roles".to_string());
    }

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from start roles", msg.author.name, role.to_role_cached(&ctx).await.unwrap().name)).await{
        return Err(e);
    }


    Ok(())
}

pub async fn start_roles_list_op(ctx: &Context, msg: &Message) -> Result<CreateEmbed, String> {
    let guid = msg.guild_id;
    if let None = guid{ return Err("Invalid guild".to_string()); }
    let guid = guid.unwrap();
    
    let mut server = Server::new();
    match read_from_json(&format!("registries/{}.json", guid.as_u64()), &mut server) {
        Ok(()) => (),
        Err(_) => return Err("Invalid guild".to_string())
    }

    let guild = Guild::get(&ctx.http, guid).await;
    if let Err(_) = guild{ return Err("Error parsing to guild".to_string()); }
    let guild = guild.unwrap();
    
    let mut response = String::new();
    for role in guild.roles {
        if server.start_roles.contains(&role.0) && role.1.name != "@everyone"{
            response.push_str(&format!("{}\n", role.1.name)[..]);
        }
    }

    let mut e = create_embed();
    e.title("Start roles");
    e.description(response);

    Ok(e)
}


pub async fn add_allowed_role_op(ctx: &Context, msg: &Message, args: Args) -> Result<(), String> {
    let mut server = Server::new();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let mut role: Option<RoleId> = None;

    for guild_role in &guild.roles{
        if guild_role.1.name == *args.rest().to_string() {
            role = Some(*guild_role.0);
        }
    }

    if let None = role { return Err("Could not find role".to_string()); }

    let role = role.unwrap();

    if server.allowed_roles.contains(&role){
        return Err("Role already a starting role".to_string());
    }

    server.allowed_roles.push(role);

    if let Err(_) = write_to_json(&filepath, server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} added role {} to allowed roles", msg.author.name, role.to_role_cached(&ctx).await.unwrap().name)).await{
        return Err(e);
    }


    Ok(())
}

pub async fn remove_allowed_role_op(ctx: &Context, msg: &Message, args: Args) -> Result<(), String> {
    let guild = msg.guild(&ctx).await;
    if let None = guild { return Err("Could not find guild".to_string())}
    let guild = guild.unwrap();

    let mut server = Server::new();

    let filepath = format!("registries/{}.json", guild.id.as_u64());

    if let Err(_) = read_from_json(&filepath, &mut server){
        return Err("Could not find registry".to_string());
    }

    let mut role: Option<RoleId> = None;

    for guild_role in &guild.roles{
        if guild_role.1.name == *args.rest().to_string() {
            role = Some(*guild_role.0);
        }
    }

    if let None = role { return Err("Could not find role".to_string()); }

    let role = role.unwrap();

    if server.allowed_roles.contains(&role){
        server.allowed_roles.iter().position(|x|{ x == &role });
    } else {
        return Err("Unable to find role in starting roles".to_string());
    }

    if let Err(_) = write_to_json(&format!("registries/{}.json", guild.id.as_u64()), server){
        return Err("Could not find registry".to_string());
    }

    if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from allowed roles", msg.author.name, role.to_role_cached(&ctx).await.unwrap().name)).await{
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