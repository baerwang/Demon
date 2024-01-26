use std::collections::HashSet;

use crossbeam::channel::Sender;
use headless_chrome::protocol::cdp::Network::{Request, Response};
use headless_chrome::Browser;

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
    pub rx_store: HashSet<String>,

    pub sender: Sender<String>,
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
            scan,
            repeat,
            store: HashSet::new(),
            rx_store: HashSet::new(),
            sender: tx,
        }
    }

    pub fn send_message(&self, message: &str) {
        scope(self.sender.clone(), message.to_string())
    }

    pub fn send_req(&mut self, req: Request) {
        let handle = self.repeat.handle(req.clone());
        if self.rx_store.insert(handle) {
            self.send_message(req.url.as_str())
        }
    }

    pub fn send_rsp(&mut self, rsp: Response) {
        if self.rx_store.insert(rsp.url.clone()) {
            self.send_message(rsp.url.as_str())
        }
    }
}

pub fn scope(tx: Sender<String>, msg: String) {
    _ = crossbeam::scope(|s| {
        s.spawn(|_| {
            if tx.send(msg).is_err() {
                log::error!("Failed to send URL through channel");
            }
        });
    });
}
