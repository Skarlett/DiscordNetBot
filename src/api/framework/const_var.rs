// Because I can.
use std::time::SystemTime;
lazy_static! {
    pub static ref UPTIME: SystemTime = SystemTime::now();
}
