use serenity::{
    prelude::*,
    builder::CreateEmbed,
    utils::Colour,
    model::{
        user::User,
        prelude::*,
    },
    //builder::EditMember,
};
use serde::Serialize;
use serde::Deserialize;
use std::{
    io::BufReader,
    fs,
    fs::File,
    collections::HashMap,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub name: String,
    pub id: u64,
    pub log_channel: ChannelId,
    pub users: Vec<Member>,
    pub start_roles: Vec<RoleId>,
    pub allowed_roles: Vec<RoleId>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            name: "".to_string(),
            id:0,
            log_channel: ChannelId(0),
            users: Vec::new(),
            start_roles: Vec::new(),
            allowed_roles: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub username: String,
    pub id: u64,
    pub warns: Vec<String>,
}

//Get guild ID
pub fn get_id(msg: &Message, holder:&mut u64) -> Result<(), String>{
    match msg.guild_id {
        Some(id) => {
            *holder = *id.as_u64();
            return Ok(());
        },
        None => return Err("Invalid guild".to_string())
    };
}

//Write a struct to a json file
pub fn write_to_json<T>(filepath:&String, content: T) -> Result<(), std::io::Error>
where
    T: Serialize
{
    fs::write(&filepath, serde_json::to_string(&content)?)?;
    Ok(())
}

//Read a struct from a json file
pub fn read_from_json<'a, T>(filepath:&String, receiver: &mut T) -> Result<(), std::io::Error>
where
    T: for<'de> serde::Deserialize<'de>,
{
    *receiver = serde_json::from_value(
        serde_json::from_reader(
            BufReader::new(
                File::open(&filepath)
    ?))?)?;

    Ok(())
}

pub fn register(_ctx: &Context, msg: &Message, user: &User) -> Result<(), std::io::Error> {
    //Get guild id
    let mut guild_id:u64 = 0;

    if let Err(_) = get_id(msg, &mut guild_id) {
        return Ok(())
    }

    //Update server registry

    let filepath = format!("registries/{}.json", guild_id);

    let mut server = Server::new();
    /*
    GuildId(guild_id).edit_member(&ctx.http, user.id, |member|{
        EditMember.roles(vec![RoleId(782378398168121345)])
    });
    */

    read_from_json(&filepath, &mut server)?;

    //Check if user is already registered
    for usr in &server.users{
        if usr.username == user.tag() {
            return Ok(())
        }
    }

    server.users.push(Member { username: user.tag(), id:*user.id.as_u64(), warns:Vec::new() });

    write_to_json(&filepath, server)?;

    Ok(())
}

pub fn deregister(_ctx: &Context, msg: &Message, user: &User) -> Result<(), std::io::Error> {
    //Get guild id
    let mut guild_id:u64 = 0;

    if let Err(_) = get_id(msg, &mut guild_id) {
        return Ok(())
    }

    //Update server registry

    let filepath = format!("registries/{}.json", guild_id);

    let mut server = Server::new();

    read_from_json(&filepath, &mut server)?;

    let member_pos = server.users.iter().position(|x|{ &x.id == user.id.as_u64() });

    if let None = member_pos {
        return Ok(())
    }

    //Yeah using unwrap is bad but we already checked if it was none so like
    server.users.remove(member_pos.unwrap());

    write_to_json(&filepath, server)?;

    Ok(())
}

//Create a basic embed for other functions to use
pub fn create_embed() -> CreateEmbed {
    let mut e = CreateEmbed(HashMap::new());
    e.color(Colour::from_rgb(0, 255, 255));
    e
}

pub async fn log(ctx: &Context, guild_id: u64, message: String) -> Result<(), String> {
    let mut server = Server::new();

    if let Err(_) = read_from_json(&format!("registries/{}.json", guild_id), &mut server){
        return Err("Could not find registry".to_string());
    }

    if let Err(_) = server.log_channel.say(&ctx, message).await {
        return Err("Could not log message".to_string());
    }

    Ok(())
}