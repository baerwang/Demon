use std::str::FromStr;

use regex::Regex;

pub fn factory(r: &str, url: url::Url, sub_domain: Option<Vec<String>>) -> Box<dyn ScanPolicy> {
    let host = url.host_str().unwrap().to_string();
    match r {
        "all" => Box::new(All {
            regex: domain_regexp(host),
        }),
        "sub_domain" => Box::new(SubDomain { sub_domain }),
        "not_sub_domain" => Box::new(NotSubDomain {
            regex: domain_regexp(host),
            sub_domain,
        }),
        "dir" => Box::new(Directory {
            root: url.to_string(),
        }),
        _ => Box::new(Current { host }),
    }
}

pub trait ScanPolicy {
    fn handle(&self, _: &str) -> bool;
}

struct Current {
    host: String,
}

impl ScanPolicy for Current {
    fn handle(&self, u: &str) -> bool {
        let v = url::Url::from_str(u).unwrap();
        v.host().unwrap().to_string() == self.host
    }
}

struct All {
    regex: Regex,
}

impl ScanPolicy for All {
    fn handle(&self, u: &str) -> bool {
        let v = url::Url::from_str(u).unwrap();
        self.regex.is_match(v.host_str().unwrap())
    }
}

struct SubDomain {
    sub_domain: Option<Vec<String>>,
}

impl ScanPolicy for SubDomain {
    fn handle(&self, u: &str) -> bool {
        if let Some(items) = &self.sub_domain {
            let v = url::Url::from_str(u).unwrap();
            let host = v.host_str().unwrap_or_default();
            for domain in items {
                if host == domain {
                    return true;
                }
            }
        }
        false
    }
}

struct NotSubDomain {
    regex: Regex,
    sub_domain: Option<Vec<String>>,
}

impl ScanPolicy for NotSubDomain {
    fn handle(&self, u: &str) -> bool {
        let v = url::Url::from_str(u).unwrap();
        let host = v.host_str().unwrap_or_default();
        if !self.regex.is_match(host) {
            return false;
        }
        if let Some(items) = &self.sub_domain {
            for domain in items {
                if host == domain {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

struct Directory {
    root: String,
}

impl ScanPolicy for Directory {
    fn handle(&self, u: &str) -> bool {
        if let Ok(v) = url::Url::from_str(u) {
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

fn domain_regexp(host: String) -> Regex {
    let split: Vec<&str> = host.split('.').collect();

    let regex_str = match split.len() {
        3 => format!(r#"^.*.{}.{}"#, split[1], split[2]),
        2 => format!(r#"^.*{}.{}"#, split[0], split[1]),
        _ => format!(r#"^.*{}"#, host),
    };

    Regex::new(&regex_str).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::handler::scan_policy::domain_regexp;

    #[test]
    fn domain_regexp_test() {
        let regex = domain_regexp("example.com".to_string());
        assert!(regex.is_match("demon.example.com"));
        assert!(!regex.is_match("demon.example1.com"));
        let regex = domain_regexp("demon.example.com".to_string());
        assert!(!regex.is_match("example.com"));
        assert!(regex.is_match("demon1.example.com"));
    }
}
