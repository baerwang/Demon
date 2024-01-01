use std::collections::HashMap;
use std::sync::Arc;

use headless_chrome::Tab;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common::form::{
    random_date, random_email, random_password, random_phone, random_pin_yin, smart_text,
};

type HtmlFn = fn(Arc<Tab>, Html);

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
    map.insert("select-one", select_one);
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

fn text(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap().click();
    if !h.readonly.is_empty() {
        _ = tab.press_key("ArrowDown").unwrap().press_key("Enter");
        return;
    }
    _ = tab
        .send_character(smart_text(h.label.as_str()).as_str())
        .unwrap();
}

fn textarea(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap();
    _ = tab
        .send_character(format!("{} {}", random_password(), random_pin_yin()).as_str())
        .unwrap();
}

fn password(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap();
    _ = tab.send_character(random_password().as_str()).unwrap();
}

fn email(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap();
    _ = tab.send_character(random_email().as_str())
}

fn tel(tab: Arc<Tab>, h: Html) {
    tab.find_element_by_xpath(h.xpath.as_str()).unwrap();
    _ = tab.send_character(random_phone().as_str())
}

fn date(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap();
    _ = tab.send_character(random_date().as_str())
}

fn radio(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap().click();
}

fn checkbox(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap().click();
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

fn submit(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap().click();
}

fn button(tab: Arc<Tab>, h: Html) {
    _ = tab.find_element_by_xpath(h.xpath.as_str()).unwrap().click();
}
