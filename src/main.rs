#![warn(rust_2018_idioms)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

use simplelog::*;
use std::str::FromStr;
use std::fs::File;
use std::time::Duration;
use std::error::Error;
use std::env;
use serenity::{
    model::gateway::Ready,
    client::{Client, EventHandler, Context}
};

mod api;
pub struct BotHandler;

impl EventHandler for BotHandler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn construct_logger(path: String, tfilter: LevelFilter, wfilter: LevelFilter, cfg: Config) -> Result<(), Box<dyn Error>>{
    CombinedLogger::init(
        vec![
            TermLogger::new(tfilter, cfg).unwrap(),
            WriteLogger::new(wfilter, cfg, File::create(path)?),
        ]
    )?;
    Ok(())
}

fn init() -> Result<(), Box<dyn Error>>{
    std::fs::create_dir(env::var("PLUGINS_DIR").unwrap_or("plugins/".to_string()))?;
    
    // Lets try and make rust make a static non dying reference
    // thanks lib maker.
    static mut TIMEFMT: Option<String> = None;

    if let Ok(fmt) = env::var("TIME_FMT") {
        unsafe {
            TIMEFMT = Some(fmt);
        }
    };
    let mut cfg = Config::default();

    unsafe {
        cfg.time_format = match &TIMEFMT {
            Some(fmt) => Some(fmt.as_str()),
            None => None
        };
    }
    // End of madness

    construct_logger(
            env::var("LOG_FILE").unwrap_or("bot.log".to_string()),
            LevelFilter::from_str(env::var("LOG_TERM").unwrap_or("warn".to_string()).as_str())?,
            LevelFilter::from_str(env::var("LOG_WRITE").unwrap_or("info".to_string()).as_str())?,
            cfg
        )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init()?;

    match Client::new(env::var("DISCORD_TOKEN")?.as_str(), BotHandler) {
        Ok(mut client) => {
           std::thread::spawn(move || {
                if let Err(why) = client.start() {
                    error!("Client error: {:?}", why);
                    panic!();
                }
            });
            
            if let Err(why) = api::start(
                env::var("BIND").unwrap_or("0.0.0.0:9449".to_string()).as_str(),
                &Duration::from_millis(env::var("TIMEOUT").unwrap_or("150".to_string()).parse::<u64>()?)) {
                    error!("Couldn\'t open API: {:?}", why);
                    panic!();
            }
        },
        Err(e) => {
            error!("Error occured in start up [{:?}]", e);
            panic!();
        }
    
    }
    Ok(())
}
