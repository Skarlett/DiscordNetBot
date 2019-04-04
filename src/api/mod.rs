mod framework;

use framework::models::{Request, Response, Status, UPTIME};
use framework::{start as _start, models, APIMap};
use std::time::Duration;
use std::sync::Arc;
use std::error::Error;
use serenity::CACHE;
use serenity::model::id::{ChannelId, UserId};


// Syntax sugar
macro_rules! API(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = APIMap::new();
            $(
                m.insert($key, Arc::new($value));
            )+
            m
        }
     };
);


pub fn start(address: &str, timeout: &Duration) -> std::io::Result<()> {
   // let mut state = models::State::new();

    _start(
        API!{
            "test" => |r| Ok(models::Response::ok(&r)),
            "msg" => |r| msg(r),
            "info" => |r| info(r)
        },
        address,
        timeout
    )?;

    Ok(())
}

fn msg(req: Request) -> Result<Response, Box<dyn Error>> {
    // msg, id, medium
    let mut medium = req.arguments[2].clone();
    medium.make_ascii_lowercase();

    let uuid = req.arguments[1].parse::<u64>()?;

    match medium.as_str() {
        "channel" => {
            ChannelId(uuid).say(req.arguments[0].clone())?;
        },
        "user" => { 
            let user = UserId(uuid).to_user()?;
            user.create_dm_channel()?;
            user.direct_message(|m| m.content(req.arguments[0].clone()))?;
        },
        _ => return Ok(Response::error(Status::NoAction, "Unknown message medium specified, use USER or CHANNEL"))
    };
    Ok(Response::ok(&req))
}


fn info(req: Request) -> Result<Response, Box<dyn Error>>{    
    let cons = CACHE.read().guilds.iter().map(|(gid, gobj)| {
        let mut channels_buf: Vec<(u64, String)> = Vec::new();        
        for (cid, cobj) in &gobj.read().channels {
            channels_buf.push((cid.as_u64().clone(), cobj.read().name.clone()))
        }    
        (gobj.read().name.clone(), channels_buf)
    }).collect::<Vec<_>>();

    Ok(Response::ok(&req).body(
        json!({
            "shards": CACHE.read().shard_count,
            "connections": cons, 
            "bot": CACHE.read().user.name,
            "uptime": UPTIME.elapsed()?.as_secs(),
        })
    ))
}
