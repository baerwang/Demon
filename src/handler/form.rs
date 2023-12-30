use std::collections::HashMap;

use once_cell::sync::Lazy;

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

#[allow(dead_code)]
pub struct Html {
    id: String,
    name: String,
    el_type: String,
    tag_name: String,
    class_name: String,
    label: String,
    readonly: String,
    xpath: String,
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
