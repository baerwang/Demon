use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

type HtmlFn = fn(Html);

pub static FORM: Lazy<HashMap<&str, HtmlFn>> = Lazy::new(|| {
    let mut map: HashMap<&str, HtmlFn> = HashMap::new();
    map.insert("text", text);
    map.insert("textarea", textarea);
    map.insert("password", password);
    map.insert("email", email);
    map.insert("tel", tel);
    map.insert("date", date);
    map.insert("radio", radio);
    map.insert("checkbox", checkbox);
    map.insert("select_one", select_one);
    map.insert("submit", submit);
    map.insert("button", button);
    map
});

#[derive(Debug, Deserialize)]
pub struct Html {
    pub id: String,
    pub name: String,
    pub el_type: String,
    pub tag_name: String,
    pub class_name: String,
    pub label: String,
    pub readonly: String,
    pub xpath: String,
}

fn text(_: Html) {}

fn textarea(_: Html) {}

fn password(_: Html) {}

fn email(_: Html) {}

fn tel(_: Html) {}

fn date(_: Html) {}

fn radio(_: Html) {}

fn checkbox(_: Html) {}

fn select_one(_: Html) {}

fn submit(_: Html) {}

fn button(_: Html) {}
