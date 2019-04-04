#![warn(rust_2018_idioms)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;


use std::time::Duration;
use std::error::Error;
use serenity::{
    model::{
        gateway::Ready,
        id::UserId
    },
    client::{Client, EventHandler, Context}
};

static TOKEN: &'static str = "NDYxMzU5NjUwNDk2MDUzMjU4.XKXXXQ.00BDvCeemUfB-wkY5PyuxFZ4WLc";
static MASTER: u64 = 191793436976873473;

static BIND: &'static str = "0.0.0.0:9449";

lazy_static! {
    static ref TIMEOUT: Duration = Duration::new(1, 5);
}

mod api;
pub struct BotHandler;

impl EventHandler for BotHandler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        match UserId(MASTER).to_user() {
            Ok(user) => { 
                match user.direct_message(|m| m.content("Online")) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Message to master: {:?}", e)
                }
            }, 
            Err(e) => eprintln!("Message to master: {:?}", e)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new(TOKEN, BotHandler).expect("Err creating client");
    
    std::thread::spawn(move || {
        if let Err(why) = client.start() {
            panic!("Client error: {:?}", why);
        }
    });
    
    if let Err(why) = api::start(BIND, &TIMEOUT) {
        panic!("Couldn\'t open API: {:?}", why);
    }
    Ok(())
}
