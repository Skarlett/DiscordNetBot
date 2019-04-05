use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::collections::HashMap;

pub type APICommand = dyn Fn(Request) -> Result<Response, Box<dyn Error>> + Send + Sync;
pub type APIMap = HashMap<&'static str, Arc<APICommand>>;

#[derive(Serialize, Deserialize, Clone, Debug)] // Auto Impl
pub enum Status {
    OK,
    BadFormat,
    NoAction,
    ServerErr
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
} 

#[derive(Serialize, Clone)]
pub struct Response {
    pub action: String,
    pub arguments: Vec<String>,
    pub status: Status,
    pub response: Value,
}

impl Response {
    pub fn new(req: &Request, status: Status, response: Value) -> Self {
        
        Self {
            action: req.action.clone(),
            arguments: req.arguments.clone(),
            status: status,
            response: response
        }
    }

    pub fn ok(req: &Request) -> Self {
        Self::new(req, Status::OK, Value::String("OK".to_string()))
    }
    
    pub fn error<T>(status: Status, errmsg: T) -> Self 
    where T: std::fmt::Debug + std::fmt::Display {
        Self {
            action: "NoAction".to_string(),
            arguments: vec![],
            status: status,
            response: Value::String(errmsg.to_string())
        }
    }

    #[allow(dead_code)]
    pub fn status(mut self, s: Status) -> Self {
        self.status = s;
        self
    }

    pub fn body(mut self, s: serde_json::Value) -> Self {
        self.response = s;
        self
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Request {
    pub action: String,
    pub arguments: Vec<String>,
}
