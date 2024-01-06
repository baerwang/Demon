use std::collections::HashMap;

use crate::handler::duplicate::Duplicate;
use crate::handler::scan_policy::ScanPolicy;

pub struct TaskConfig {
    pub headers: HashMap<String, String>,
    pub robots: bool,
    pub username: Option<String>,
    pub password: Option<String>,
    pub range: Box<dyn ScanPolicy>,
    pub repeat: Box<dyn Duplicate>,
}
