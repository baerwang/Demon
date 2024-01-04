use std::str::FromStr;

use headless_chrome::protocol::cdp::Network::Request;

pub trait ScanPolicy {
    fn handle(&self, req: Request) -> bool;
}

struct Current {
    host: String,
}

impl ScanPolicy for Current {
    fn handle(&self, req: Request) -> bool {
        let v = url::Url::from_str(req.url.as_str()).unwrap();
        v.host().unwrap().to_string() == self.host
    }
}

struct Whole {}

impl ScanPolicy for Whole {
    fn handle(&self, _req: Request) -> bool {
        todo!()
    }
}

struct ScanSubDomain {}

impl ScanPolicy for ScanSubDomain {
    fn handle(&self, _req: Request) -> bool {
        todo!()
    }
}

struct NotScanSubDomain {}

impl ScanPolicy for NotScanSubDomain {
    fn handle(&self, _req: Request) -> bool {
        todo!()
    }
}

struct ScanDirectory {
    root: String,
}

impl ScanPolicy for ScanDirectory {
    fn handle(&self, req: Request) -> bool {
        if let Ok(v) = url::Url::from_str(&req.url) {
            if v.path().is_empty() {
                return false;
            }
            let combined = format!("{}/{}", v.host_str().unwrap_or_default(), v.path());
            self.root.contains(&combined)
        } else {
            false
        }
    }
}
