use std::collections::HashMap;

pub struct TaskConfig {
    pub target: Vec<String>,
    pub headers: HashMap<String, String>,
    pub robots: bool,
    pub range: i8,
    pub repeat: i8,
}
