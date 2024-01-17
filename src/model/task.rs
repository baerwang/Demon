use std::collections::HashMap;

pub struct TaskConfig {
    pub headers: HashMap<String, String>,
    pub robots: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}
