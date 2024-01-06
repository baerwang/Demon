use headless_chrome::protocol::cdp::Network::Request;
use regex::Regex;

pub fn factory(r: &str) -> Box<dyn Duplicate> {
    match r {
        "body" => Box::new(ParamsMethodBody {}),
        "params" => Box::new(Params {}),
        _ => Box::new(ParamsMethod {}),
    }
}

pub trait Duplicate {
    fn handle(&self, req: Request) -> String;
}

struct Params {}

impl Duplicate for Params {
    fn handle(&self, req: Request) -> String {
        format!("{}{}", req.url, req.method)
    }
}

struct ParamsMethod {}

impl Duplicate for ParamsMethod {
    fn handle(&self, req: Request) -> String {
        format!(
            "{}{}{}",
            parse_url(req.url.clone()),
            req.method,
            parse_url_params(req.url.clone())
        )
    }
}

struct ParamsMethodBody {}

impl Duplicate for ParamsMethodBody {
    fn handle(&self, req: Request) -> String {
        format!(
            "{}{}{}{}",
            parse_url(req.url.clone()),
            req.method,
            parse_url_params(req.url.clone()),
            parse_body(req.clone())
        )
    }
}

fn parse_url(url: String) -> String {
    let url_parts: Vec<&str> = url.split('?').collect();
    url_parts.first().cloned().unwrap_or_default().to_string()
}

fn parse_url_params(input: String) -> String {
    let split: Vec<&str> = input.split('?').collect();
    if split.len() > 1 {
        let values: Vec<&str> = split[1].split('&').collect();
        let mut array: Vec<&str> = Vec::with_capacity(values.len());

        for value in values {
            let sub_split: Vec<&str> = value.split('=').collect();
            if sub_split.len() > 1 {
                array.push(sub_split[0]);
            }
        }
        array.sort();
        return array.join(",");
    }
    String::new()
}

fn parse_body(req: Request) -> String {
    let post_data = req.post_data.clone().unwrap_or_default();
    if post_data.is_empty() {
        return String::new();
    }

    let conn_type = match req.headers.0 {
        Some(data) => data.get("Content-Type").unwrap().to_string(),
        None => String::new(),
    };

    if conn_type.is_empty() {
        return conn_type;
    }

    match conn_type.as_str() {
        "application/json" => {}
        s if s.contains("application/x-www-form-urlencoded") => {
            return parse_url_params(format!("?{}", post_data));
        }
        s if s.contains("text/html;") || s.contains("text/xml") => {
            if post_data.contains("<xml>") {
                return parse_xml(post_data);
            }
            return parse_url_params(format!("?{}", post_data));
        }
        _ => {}
    }
    String::new()
}

fn parse_xml(post_data: String) -> String {
    let re = Regex::new(r#"[^ ]*?="#).expect("Failed to create regex");
    let values: Vec<&str> = re
        .find_iter(post_data.as_str())
        .map(|mat| mat.as_str())
        .collect();
    let mut keys: Vec<&str> = values.into_iter().collect();
    keys.sort();
    keys.join(",")
}
