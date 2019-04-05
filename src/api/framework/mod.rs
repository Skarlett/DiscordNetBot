use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Write, BufReader};
use std::time::Duration;
use threadpool::ThreadPool;
use std::error::Error;
use std::sync::Arc;

pub mod models;
pub mod const_var;

use models::*;

fn get_request(stream: &TcpStream) -> Result<Request, Box<dyn Error>> {
    let package: Request = {
        serde_json::from_reader(BufReader::new(stream))?
    };
    Ok(package)
}
    
fn respond(map: Arc<APIMap>, request: &Request) -> Result<Response, Box<dyn Error>> {
    match map.get(request.action.as_str()) {
        Some(cmd) => {
                (cmd)(request.clone())
            },
        None => {
            Ok(Response::error(Status::NoAction, "Not found")) 
        }
    }
}

fn handle_client(pool: &ThreadPool, api: Arc<APIMap>, mut stream: TcpStream) {
    pool.execute(move || {
        let response = match get_request(&stream) {
            Ok(req) => {
                match req.action.as_str() {
                    "help" => {
                        info!("Request: {:?} -> help | {}", stream.peer_addr(), req.action);
                        help_menu(req, api)
                    },
                    _ => {
                        match respond(api, &req) {
                            Ok(response) => {
                                info!("Request: {:?} -> {}", stream.peer_addr(), req.action);
                                response
                            },
                            Err(e) => {
                                warn!("Error in Response from {:?} in cmd {}, returning {:?}", stream.peer_addr(), req.action, e);
                                Response::error(Status::ServerErr, "An Error has occured.")
                            }
                        }
                    }
                }
            }

            Err(_) => {
                warn!("Failed Request: {:?}", stream.peer_addr());
                Response::error(
                    Status::BadFormat,
                    "Expected format {'action': 'method', 'arguments': ['all strings' ...]}\n"
                )
            }
        };
        stream.write_all(serde_json::to_string(&response).unwrap().as_bytes());
        stream.shutdown(Shutdown::Both);

    });
}

pub fn help_menu(req: Request, api: Arc<APIMap>) -> Response {
    Response::ok(&req).body(
        json!(api.iter().map(|(k, _)| k).collect::<Vec<_>>())
    )
}

pub fn start_api(map: APIMap, address: &str, timeout: &Duration) -> std::io::Result<()> {
    let pool = ThreadPool::new(num_cpus::get());
    let listener = TcpListener::bind(address)?;
    let api = Arc::new(map);

    for stream in listener.incoming() {
        let tmo = timeout.clone();
        match stream {
            Ok(stream) => {
                stream.set_read_timeout(Some(tmo));
                stream.set_write_timeout(Some(tmo));
                handle_client(&pool, api.clone(), stream);
            }
            Err(e) => error!("Accepting Connection Error: {:?}", e)
        }
    }
    Ok(())
}
