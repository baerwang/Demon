use headless_chrome::Browser;
use tokio::sync::mpsc::Sender;

use crate::model::task::TaskConfig;

pub struct GlobalState {
    pub domain: String,
    pub browser: Browser,
    pub config: TaskConfig,

    pub sender: Option<Sender<String>>,
}

impl GlobalState {
    pub fn new(tx: Sender<String>, domain: String, browser: Browser, config: TaskConfig) -> Self {
        GlobalState {
            domain,
            browser,
            config,
            sender: Some(tx),
        }
    }

    pub fn send_message(&self, message: &str) {
        if let Some(ref sender) = self.sender {
            if sender.blocking_send(message.to_owned()).is_err() {
                log::error!("Failed to send URL through channel");
            }
        }
    }
}
