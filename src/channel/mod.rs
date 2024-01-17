use std::collections::HashSet;

use headless_chrome::Browser;
use tokio::sync::mpsc::Sender;

use crate::handler::duplicate::Duplicate;
use crate::handler::scan_policy::ScanPolicy;
use crate::model::task::TaskConfig;

pub struct GlobalState {
    pub domain: String,
    pub browser: Browser,
    pub config: TaskConfig,
    pub scan: Box<dyn ScanPolicy>,
    pub repeat: Box<dyn Duplicate>,
    pub store: HashSet<String>,

    pub sender: Option<Sender<String>>,
}

impl GlobalState {
    pub fn new(
        tx: Sender<String>,
        domain: String,
        browser: Browser,
        scan: Box<dyn ScanPolicy>,
        repeat: Box<dyn Duplicate>,
        config: TaskConfig,
    ) -> Self {
        GlobalState {
            domain,
            browser,
            config,
            store: HashSet::new(),
            sender: Some(tx),
            scan,
            repeat,
        }
    }

    pub async fn send_message(&self, message: &str) {
        if let Some(ref sender) = self.sender {
            if sender.send(message.to_owned()).await.is_err() {
                log::error!("Failed to send URL through channel");
            }
        }
    }
}
