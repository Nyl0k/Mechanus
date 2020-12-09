use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    model::{
        gateway::{
            Ready,
            Activity,
        },
    },
    prelude::*
};

use std::{
    collections::HashSet,
    fs,
};

mod commands;

use commands::command_list::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready){
        ctx.set_activity(Activity::playing("&&help")).await;
        println!("Mechanus is online");
    }
}


#[group]
#[commands(
    help,
    about,
    user_info,
    server_info,
    add_role,
    remove_role,
    roles_list,
    my_warns,
    ping,
)]
struct User;

#[group]
#[commands(
    admin_help,
    register_user,
    deregister_user,
    mass_register,
    give_role,
    take_role,
    purge,
    user_list,
    warn,
    get_warns,
    forgive,
    forgive_all,
    kick,
    ban,
)]
struct Admin;

#[group]
#[commands(
    config_help,
    initialize, 
    uninitialize,
    add_start_role, 
    remove_start_role,
    start_roles_list,
    add_allowed_role,
    remove_allowed_role,
    set_logging_channel,
)]
struct Config;

#[tokio::main]
async fn main() {
    //Read token from file, file not not included
    let token: String = fs::read_to_string("token.txt")
        .expect("No token file present")
        .parse().unwrap();
    
    let prefix = "&&";
    
    let http = Http::new_with_token(&token);

    //Get some metadata
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(error) => panic!("Could not access application info: {:?}", error)
    };
    
    //Create command framework
    let framework = StandardFramework::new()
        .configure(|c| 
            c.owners(owners)
            .prefix(prefix))
        .group(&USER_GROUP)
        .group(&ADMIN_GROUP)
        .group(&CONFIG_GROUP);
    
    //Create client
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    
    //Start client
    if let Err(error) = client.start().await {
        eprintln!("Client error: {:?}", error);
    }
}