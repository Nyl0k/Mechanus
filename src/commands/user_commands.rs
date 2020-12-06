use serenity::{
    prelude::*,
    builder::CreateEmbed,
    model::prelude::*,
    model::{
        id::RoleId,
    },
    framework::standard::Args,
};

use mechanus_bot::*;

//All commands normal members can use
pub fn help_op() ->  CreateEmbed {
    let mut e = create_embed();

    e.title("Commands list");
    e.field("&&help", "Displays this menu", false);
    e.field("&&about", "Displays some information about the bot", false);
    e.field("&&user_info <@user>", "Displays some information about the user", false);
    e.field("&&server_info", "Displays some information about the server", false);
    e.field("&&add_role <role>", "Give yourself a role", false);
    e.field("&&remove_role <role>", "Remove a role from yourself", false);
    e.field("&&roles_list", "See list of available roles", false);
    e.field("&&my_warns", "See what warns you have, if any", false);
    e.field("&&ping", "Send ping", false);

    e
}

pub fn about_op() -> CreateEmbed {
    let mut e = create_embed();
    e.title("About Mechanus");
    e.field("Overview", "Mechanus is a highly configurable administration bot, designed to help with all matters of running a server. User and server metadata are stored in a json file specific to the server, be sure to run &&initialize to create this file as few commands function without it. When a user joins, you must &&register_user them to enter them into the system, or enable automatic registration. See &&config_help for more information.", false);
    e.field("For users", "Mechanus provides several utilities to aid you in server life, including adding and removing roles from yourself and retrieving information about other users. Type &&help for more information", false);
    e.field("For admins", "Mechanus provide a whole host of functions for server administration, including logging, message purging, giving roles to new users upon arrival, and warning. Type &&admin_help for more info", false);
    e.field("Configuration", "Mechanus can be configured in many ways, including whether to use manual registration, what roles are banned from public use, what channel messages should be logged in, and more. Type &&config_help for more info", false);
    e.footer(|f|{ f.text("Contact @Starfall#7832 for more information"); f });

    e
}

pub async fn user_info_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<CreateEmbed, String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Could not parse to user".to_string());
    }
    let user_id = user_id.unwrap();

    let guid = msg.guild_id;
    if let None = guid{
        return Err("Could not find guild".to_string());
    }
    let guid = guid.unwrap();

    let join_date = guid.member(&ctx, user_id).await;
    if let Err(_) = join_date{
        return Err("Could not find user".to_string());
    }
    let join_date = join_date.unwrap().joined_at;

    let user = user_id.to_user(&ctx.http).await.unwrap();

    let mut nick = user.nick_in(&ctx.http, guid).await;

    if let None = nick {
        nick = Some("None".to_string());
    }

    let mut e = create_embed();
    e.title(&user.name);
    e.thumbnail(&user.face());
    e.field("Tag", &user.tag(), false);
    e.field("Nickname", nick.unwrap(), false);
    e.field("Bot?", &user.bot, false);
    e.field("Joined", format!("{}",join_date.unwrap()), false);
    e.field("Created Account", format!("{}", &user.created_at()), false);

    Ok(e)
}

pub async fn server_info_op(ctx: &Context, msg: &Message) -> Result<CreateEmbed, String> {
    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let member_count = guild.members(&ctx.http, Some(1000), None).await;
    if let Err(_) = member_count { return Err("Error obtaining member count".to_string()); }
    let member_count = member_count.unwrap().len();

    let owner = UserId(*guild.owner_id.as_u64()).to_user(&ctx.http).await;
    if let Err(_) = owner { return Err("Error obtaining owner".to_string()); }
    let owner = owner.unwrap().name;

    let mut e = create_embed();
    e.title(&guild.name);
    e.thumbnail(guild.icon_url().unwrap_or_default());
    e.field("Member count", member_count, false);
    e.field("Owner", owner, false);

    Ok(e)
}

//Add or remove a role
pub async fn user_role_op(ctx: &Context, msg: &Message, args: Args, remove: bool) -> Result<(), String> {
    //Get guild object to perform ops on, return if guild doesn't exist
    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    //Figure out what role was inputted, return if it doesn't exist
    let mut role: Option<RoleId> = None;

    for guild_role in &guild.roles{
        if guild_role.1.name == *args.rest().to_string() {
            role = Some(*guild_role.0);
        }
    }

    if let None = role { return Err("Could not find role".to_string()); }

    let role = role.unwrap();

    //Get server registry
    let mut server = Server::new();
    if let Err(_) = read_from_json(&format!("registries/{}.json", guild.id.as_u64()), &mut server) {
        return Err("Could not find registry".to_string()); 
    }

    //Return if the role is banned
    if server.banned_roles.contains(&role){ return Err("This role is inaccessible with this command".to_string()); }

    //Get member object we can perform opertations on
    let mut member = guild.member(&ctx.http, &msg.author.id).await.unwrap();
    
    //Add or remove role
    if remove {
        if let Err(_) = member.remove_role(&ctx.http, &role).await {
            return Err("Could not give you this role".to_string());
        };
        if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from user {}", msg.author, role.to_role_cached(&ctx).await.unwrap().name, member.user.name)).await{
            return Err(e);
        }
    } else {
        if let Err(_) = member.add_role(&ctx.http, &role).await {
            return Err("Could not remove this role from you".to_string());
        }
        if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} gave user {} role {}", msg.author, member.user.name, role.to_role_cached(&ctx).await.unwrap().name)).await{
            return Err(e);
        }
    }

    Ok(())
}

pub async fn roles_list_op(ctx: &Context, msg: &Message) -> Result<CreateEmbed, String> {
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
        if !server.banned_roles.contains(&role.0) && role.1.name != "@everyone"{
            response.push_str(&format!("{}\n", role.1.name)[..]);
        }
    }

    let mut e = create_embed();
    e.title("Available roles");
    e.description(response);

    Ok(e)
}

pub async fn my_warns_op(msg: &Message) -> Result<CreateEmbed, String> {
    let guid = msg.guild_id;
    if let None = guid{ return Err("Invalid guild".to_string()); }
    let guid = guid.unwrap();
    
    let mut server = Server::new();
    match read_from_json(&format!("registries/{}.json", guid.as_u64()), &mut server) {
        Ok(()) => (),
        Err(_) => return Err("Invalid guild".to_string())
    }

    for usr in server.users {
        if usr.username == msg.author.tag() {
            let mut response = String::new();
            for warn in usr.warns {
                response.push_str(&format!("{}\n", warn)[..]);
            }

            let mut e = create_embed();
            e.title("Warns:");
            if response.len()>0{
                e.description(response);
            } else {
                e.description("You have no warns. Congrats.");
            }
            return Ok(e);
        }
    }
    Err("Could not find you in the registry".to_string())
}

//Simple test to make sure the bot is online
pub fn ping_op() -> String {
    "Pong!".to_string()
}