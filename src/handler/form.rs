use std::collections::HashMap;
use std::sync::Arc;

use headless_chrome::Tab;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common::form::{
    random_date, random_email, random_password, random_phone, random_pin_yin, smart_text,
};

type HtmlFn = fn(Arc<Tab>, Html);

static FROM: Lazy<HashMap<&str, HtmlFn>> = Lazy::new(|| {
    let mut map: HashMap<&str, HtmlFn> = HashMap::new();
    map.insert("text", text);
    map.insert("textarea", textarea);
    map.insert("password", password);
    map.insert("email", email);
    map.insert("tel", tel);
    map.insert("date", date);
    map.insert("radio", general);
    map.insert("checkbox", general);
    map.insert("select-one", select_one);
    map.insert("submit", general);
    map.insert("button", general);
    map
});

pub fn filter(tab: Arc<Tab>, h: Html) {
    if let Some(func) = FROM.get(h.el_type.as_str()) {
        _ = match tab.find_element_by_xpath(h.xpath.as_str()) {
            Ok(v) => {
                _ = v.click();
                func(tab.clone(), h);
            }
            Err(err) => log::warn!("{}", err),
        };
    } else {
        log::warn!("not el type: {}", h.el_type);
    }
}

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

fn general(_: Arc<Tab>, _: Html) {}

fn text(tab: Arc<Tab>, h: Html) {
    if !h.readonly.is_empty() {
        _ = tab.press_key("ArrowDown").unwrap().press_key("Enter");
        return;
    }
    _ = tab
        .send_character(smart_text(h.label.as_str()).as_str())
        .unwrap();
}

fn textarea(tab: Arc<Tab>, _: Html) {
    _ = tab
        .send_character(format!("{} {}", random_password(), random_pin_yin()).as_str())
        .unwrap();
}

fn password(tab: Arc<Tab>, _: Html) {
    _ = tab.send_character(random_password().as_str()).unwrap();
}

fn email(tab: Arc<Tab>, _: Html) {
    _ = tab.send_character(random_email().as_str())
}

fn tel(tab: Arc<Tab>, _: Html) {
    _ = tab.send_character(random_phone().as_str())
}

fn date(tab: Arc<Tab>, _: Html) {
    _ = tab.send_character(random_date().as_str())
}

fn select_one(tab: Arc<Tab>, h: Html) {
    // todo
    _ = tab
        .wait_for_xpath(h.xpath.as_str())
        .unwrap()
        .focus()
        .unwrap();
    tab.press_key("ArrowDown").unwrap();
}
