use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;

use headless_chrome::Tab;

use crate::channel;
use crate::common::filter::matching_filter;
use crate::common::util;

const JS_HREF: &str = r#"
     const HREF_ATTRIBUTES = ["src", "href", "data-url", "data-href", "type", "pluginspage"];
     const CONTENT_TYPE = ["application/x-www-form-urlencoded", "text/plain", "text/html",
     "application/xml", "text/xml", "application/json", "text/javascript", "multipart/form-data", 
     "application/octet-stream", "text/css", "image/x-icon", "image/jpeg", "image/png", "image/gif"];
     let list = [];
     for (const href of HREF_ATTRIBUTES) {
         const result = document.querySelectorAll(`[${href}]`);
         for (const e of result) {
             const value = e.getAttribute(`${href}`);
             if (value) {
                 if (CONTENT_TYPE.some(t => value.startsWith(t))) {
                     continue;
                 }
                 list.push(value);
             }
         }
     }
     list
    "#;

const JS_OBJECT: &str = r#"
    const result = document.querySelectorAll('object[data]');
    let list = [];
    for (const e of result) {
        const value = e.getAttribute('data');
        if (value !== null) {
            list.push(value);
        }
    }
    list
    "#;

pub async fn collect(state: &mut channel::GlobalState, tab: &Arc<Tab>) {
    _ = query_selector_all(state, tab, JS_HREF).await;
    _ = query_selector_all(state, tab, JS_OBJECT).await;
}

async fn query_selector_all(
    state: &mut channel::GlobalState,
    tab: &Arc<Tab>,
    v: &str,
) -> Result<(), Box<dyn Error>> {
    let result = tab.call_method(util::evaluate(v))?;
    if let Some(result_value) = result.result.value {
        let set = serde_json::from_str::<HashSet<String>>(&result_value.to_string())?;
        for s in &set {
            if matching_filter(s) && state.store.insert(s.clone()) {
                state
                    .send_message(parse_url(state.domain.to_string(), s.to_string()).as_str())
                    .await
            }
        }
    }
    Ok(())
}

fn parse_url(root: String, child: String) -> String {
    if child.starts_with("http://") || child.starts_with("https://") {
        child
    } else {
        let tmp = child.replace("../", "");
        if !tmp.starts_with('/') {
            format!("{}/{}", root, tmp)
        } else {
            format!("{}{}", root, tmp)
        }
    }
}
