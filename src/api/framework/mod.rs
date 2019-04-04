
use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};
use std::time::Duration;
use threadpool::ThreadPool;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;


pub mod models;

pub type APICommand = dyn Fn(models::Request) -> Result<models::Response, Box<dyn Error>> + Send + Sync;
pub type APIMap = HashMap<&'static str, Arc<APICommand>>;


fn get_request(stream: &TcpStream) -> Result<models::Request, Box<dyn Error>> {
    let package: models::Request = {
        let mut rawbuf: String = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut rawbuf)?;
        serde_json::from_str(&rawbuf[..])?
    };
    Ok(package)
}
    
fn respond(map: Arc<APIMap>, request: models::Request) -> Result<models::Response, Box<dyn Error>> {
    match map.get(request.action.as_str()) {
        Some(cmd) => {
                (cmd)(request)
            },
        None => {
            Ok(models::Response::error(models::Status::NoAction, "Not found")) 
        }
    }
}

fn handle_client(pool: &ThreadPool, api: Arc<APIMap>, mut stream: TcpStream) {
    pool.execute(move || {
        let response = match get_request(&stream) {
            Ok(req) => {
                match req.action.as_str() {
                    "help" => help_menu(req, api),
                    _ => {
                        match respond(api, req) {
                            Ok(response) => {
                                response
                            },
                            Err(e) => {
                                models::Response::error(models::Status::NoAction, e)
                            }
                        }
                    }
                }
            }
            Err(_) => {
                models::Response::error(
                    models::Status::BadFormat,
                    "Bad format, Expected Json response\r\n{'action': 'method', 'arguments': ['all strings' ...]} + 0xA "
                )
            }
        };
        stream.write_all(serde_json::to_string(&response).unwrap().as_bytes());
        stream.shutdown(Shutdown::Both);

    });
}

pub fn help_menu(req: models::Request, api: Arc<APIMap>) -> models::Response {
    models::Response::ok(&req).body(
        json!(api.iter().map(|(k, _)| k).collect::<Vec<_>>())
    )
}

pub fn start(map: APIMap, address: &str, timeout: &Duration) -> std::io::Result<()> {
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
            Err(e) => eprintln!("Accepting Connection Error: {:?}", e)
        }
    }
    Ok(())
}
