use serenity::{
    prelude::*,
    builder::CreateEmbed,
    model::prelude::*,
    //model::{
    //    id::RoleId,
    //},
    framework::standard::Args,
};

use mechanus_bot::*;

//All commands admins can use
pub fn admin_help_op() ->  CreateEmbed {
    let mut e = create_embed();

    e.title("Commands list");
    e.field("&&admin_help", "Displays this menu", false);
    e.field("&&register_user <@user>", "Add a user to the system and give starting roles [r]", false);
    e.field("&&deregister_user <@user>", "Remove a user from the system [r]", false);
    e.field("&&mass_register", "Register all current server members [r]", false);
    e.field("&&give_role <@user> <role name>", "Give a user a role", false);
    e.field("&&take_role <@user> <role name", "Remove a role from a user", false);
    e.field("&&purge <number>", "Clear a number of messages from a channel", false);
    e.field("&&get_user_list", "Get a list of current registered users", false);
    e.field("&&warn <@user> <reason>", "Warn a registered user", false);
    e.field("&&get_warns <@user>", "Get a list of warns a registered user has", false);
    e.field("&&forgive <@user> <reason>", "Remove a warn from a user", false);
    e.field("&&forgive_all <@user>", "Remove all warn from a user", false);
    e.field("&&kick <@user> <reason>", "Kick a user from the server", false);
    e.field("&&ban <@user> <reason>", "Ban a user from the server", false);
    e.footer(|f|{ 
        f.text("[r] = requires manual registration to be enabled\n");
        f
    });

    e
}

pub async fn register_user_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id { return Err("Please enter a valid user".to_string()); }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()) }
    let user = user.unwrap();

    if let Err(_) = register(&ctx, &msg, &user){ return Err("Could not register user".to_string()); };

    let filepath = format!("registries/{}.json", msg.guild_id.unwrap());

    let mut server = Server::new();
    if let Err(_) = read_from_json(&filepath, &mut server){ return Err("Could not read registry".to_string()); };

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} registered user {}", msg.author, user.name)).await{
        return Err(e);
    }

    for role in server.start_roles{
        msg.guild_id.unwrap_or_default()
            .member(&ctx.http, &user.id).await.unwrap()
            .add_role(&ctx.http, role).await.unwrap();
    }

    Ok(())
}

pub async fn deregister_user_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user {
        return Err("Please enter a valid user".to_string());
    }
    let user = user.unwrap();

    if let Err(_) = deregister(&ctx, &msg, &user){
        return Err("Could not register user".to_string());
    };

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} deregistered user {}", msg.author, user.name)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn mass_register_op(ctx: &Context, msg: &Message) -> Result<(), String> {
    let guild_id = msg.guild_id;
    if let None = guild_id {
        return Err("Invalid guild".to_string());
    }
    let guild_id = guild_id.unwrap();

    let member_list = guild_id.members(&ctx.http, Some(1000), None).await;
    if let Err(_) = member_list {
        return Err("Could not get member list".to_string())
    }
    let member_list = member_list.unwrap();

    for member in member_list{
        if let Err(_) = register(&ctx, msg, &member.user) {
            return Err("Could not register member".to_string())
        }
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} registered all users", msg.author)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn role_op(ctx: &Context, msg: &Message, mut args: Args, give: bool) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let mut role:Option<RoleId> = None;

    let argu = args.rest().to_string();

    for guild_role in &guild.roles{
        if guild_role.1.name == argu {
            role = Some(*guild_role.0);
        }
    }

    if let None = role {
        return Err("Role does not exist".to_string());
    }

    let role = role.unwrap();
    if give{
        msg.guild_id.unwrap_or_default()
            .member(&ctx.http, &user_id).await.unwrap()
            .add_role(&ctx.http, role).await.unwrap();

            if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} removed role {} from user {}", 
                msg.author, 
                role.to_role_cached(&ctx).await.unwrap().name, 
                user_id.to_user(&ctx.http).await.unwrap().name)
            ).await{
                return Err(e);
            }
    } else {
        msg.guild_id.unwrap_or_default()
            .member(&ctx.http, &user_id).await.unwrap()
            .remove_role(&ctx.http, role).await.unwrap();

            if let Err(e) = log(&ctx, *guild.id.as_u64(), format!("{} gave user {} role {}", 
                msg.author, 
                user_id.to_user(&ctx.http).await.unwrap().name, 
                role.to_role_cached(&ctx).await.unwrap().name)
            ).await{
                return Err(e);
            }
    }

    Ok(())
}

pub async fn purge_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let quant = args.single::<u64>();
    if let Err(_) = quant {
        return Err("Invalid quantity".to_string());
    }
    let quant = quant.unwrap();

    let messages = msg.channel_id.messages(&ctx.http, |r| {
        r.before(msg.id).limit(quant)
    }).await;
    if let Err(_) = messages {
        return Err("Unable to find messages".to_string());
    }
    let messages = messages.unwrap();

    if let Err(_) = msg.channel_id.delete_messages(&ctx.http, messages).await {
        return Err("Unable to purge messages".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} purged {} messages in {}", msg.author, quant, msg.channel(&ctx).await.unwrap().guild().unwrap().name)).await{
        return Err(e);
    }


    Ok(())
}

pub async fn user_list_op(msg: &Message) -> Result<String, String> {
    let mut server = Server::new();

    let guild_id = msg.guild_id;
    if let None = guild_id {
        return Err("Invalid guild".to_string());
    }
    let guild_id = guild_id.unwrap();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild_id), &mut server){
        return Err("Could not return registry".to_string());
    }

    Ok(format!("User list:\n ```json\n{:#?}\n```", server.users))
}

pub async fn warn_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let mut server = Server::new();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild.id), &mut server) {
        return Err("Could not read from registry".to_string());
    }

    let reason = args.rest().to_string();

    for usr in &mut server.users {
        if usr.username == user.tag() {
            usr.warns.push(reason.clone());
        }
    }

    if let Err(_) = write_to_json(&format!("registries/{}.json", guild.id), server){
        return Err("Could not update registry".to_string());
    }

    if let Err(_) = user.dm(&ctx, |m|{
        m.content(format!("You have been warned on {} for reason {:?}. Please behave better in the future", guild.name, &reason))
    }).await {
        return Err("Could not notify user of warn".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} warned user {} for {}", msg.author, user.name, reason)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn get_warns_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let mut server = Server::new();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild.id), &mut server) {
        return Err("Could not read from registry".to_string());
    }

    for usr in server.users {
        if usr.username == user.tag() {
            if let Err(_) = msg.channel_id.say(&ctx.http, &format!("```{:#?}```", usr.warns)).await{
                ();
            }
        }
    }

    Ok(())
}

pub async fn forgive_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let mut server = Server::new();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild.id), &mut server) {
        return Err("Could not read from registry".to_string());
    }

    let reason = args.rest().to_string();

    for usr in &mut server.users {
        if usr.username == user.tag() {
            usr.warns.remove(usr.warns.iter().position(|x|{ x == &reason }).unwrap());
        }
    }

    if let Err(_) = write_to_json(&format!("registries/{}.json", guild.id), server){
        return Err("Could not update registry".to_string());
    }

    if let Err(_) = user.dm(&ctx, |m|{
        m.content(format!("You have been forgiven on {} for warn {:?}. Congratulations", guild.name, &reason))
    }).await {
        return Err("Could not notify user of forgiveness".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} forgave user {} for {}", msg.author, user.name, reason)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn forgive_all_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let mut server = Server::new();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild.id), &mut server) {
        return Err("Could not read from registry".to_string());
    }


    for usr in &mut server.users {
        if usr.username == user.tag() {
            usr.warns = Vec::new()
        }
    }

    if let Err(_) = write_to_json(&format!("registries/{}.json", guild.id), server){
        return Err("Could not update registry".to_string());
    }

    if let Err(_) = user.dm(&ctx, |m|{
        m.content(format!("You have been forgiven on {} for all warns. Congratulations", guild.name))
    }).await {
        return Err("Could not notify user of forgiveness".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} forgave user {} for everything", msg.author, user.name)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn kick_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let reason = args.rest();

    if let Err(_) = user.dm(&ctx, |m|{
        m.content(format!("You have been kicked from {} with reason {:?}. Please behave better in the future", guild.name, reason))
    }).await {
        return Err("Could not notify user of kick".to_string());
    }

    if let Err(_) = msg.guild_id.unwrap().kick_with_reason(&ctx.http, &user, &reason).await{
        return Err("Could not kick user".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} kicked user {} for {}", msg.author, user.name, reason)).await{
        return Err(e);
    }

    Ok(())
}

pub async fn ban_op(ctx: &Context, msg: &Message, mut args: Args) -> Result<(), String> {
    let user_id = args.single::<UserId>();
    if let Err(_) = user_id {
        return Err("Please enter a valid user".to_string());
    }
    let user_id = user_id.unwrap();

    let user = user_id.to_user(&ctx.http).await;
    if let Err(_) = user { return Err("Please enter a valid user".to_string()); }
    let user = user.unwrap();

    let guild = msg.guild(&ctx).await;
    if let None = guild{ return Err("Could not find guild".to_string()); }
    let guild = guild.unwrap();

    let del_count = args.single::<u8>();
    if let Err(_) = del_count {
        return Err("Please provide a delete count".to_string());
    }
    let del_count = del_count.unwrap();

    let reason = &args.rest();

    if let Err(_) = user.dm(&ctx, |m|{
        m.content(format!("You have been banned from {} with reason {:?}. Please behave better in the future", guild.name, reason))
    }).await {
        return Err("Could not notify user of kick".to_string());
    }


    if let Err(_) = msg.guild_id.unwrap().ban_with_reason(&ctx.http, &user, del_count, args.rest()).await{
        return Err("Could not ban user".to_string());
    }

    if let Err(e) = log(&ctx, *msg.guild_id.unwrap().as_u64(), format!("{} banned user {} for {}", msg.author, user.name, reason)).await{
        return Err(e);
    }

    Ok(())
}