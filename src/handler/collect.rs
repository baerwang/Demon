use std::collections::HashSet;
use std::sync::Arc;

use headless_chrome::Tab;

const CONTENT_TYPE: [&str; 14] = [
    "application/x-www-form-urlencoded",
    "text/plain",
    "text/html",
    "application/xml",
    "text/xml",
    "application/json",
    "text/javascript",
    "multipart/form-data",
    "application/octet-stream",
    "text/css",
    "image/x-icon",
    "image/jpeg",
    "image/png",
    "image/gif",
];
const HREF_ATTRIBUTES: [&str; 4] = ["src", "href", "data-url", "data-href"];

pub fn collect(tab: &Arc<Tab>) {
    _ = href(tab);
    _ = object(tab);
}

fn href(tab: &Arc<Tab>) -> Result<(), Box<dyn std::error::Error>> {
    let node_id = tab.get_document()?.node_id;
    let mut set: HashSet<String> = HashSet::new();

    for href in HREF_ATTRIBUTES {
        let result = tab.run_query_selector_all_on_node(node_id, format!("[{}]", href).as_str())?;

        for e in result {
            if let Some(attributes) = e.attributes {
                for (index, attribute) in attributes.iter().enumerate().filter(|(i, _)| i % 2 == 0)
                {
                    let name = attribute.as_str();
                    let value = attributes
                        .get(index + 1)
                        .map_or("", |v| v.as_str())
                        .to_string();

                    if name == "type" && CONTENT_TYPE.iter().any(|t| value.starts_with(t)) {
                        continue;
                    }

                    if name == "pluginspage" || name == href {
                        set.insert(value);
                    }
                }
            }
        }
    }

    log::info!("{:?}", set);

    Ok(())
}

fn object(tab: &Arc<Tab>) -> Result<(), Box<dyn std::error::Error>> {
    let node_id = tab.get_document()?.node_id;
    let mut set: HashSet<String> = HashSet::new();

    let result = tab.run_query_selector_all_on_node(node_id, "object[data]")?;
    for e in result {
        if let Some(attributes) = e.attributes {
            for (index, attribute) in attributes.iter().enumerate().filter(|(i, _)| i % 2 == 0) {
                let name = attribute.as_str();
                let value = attributes
                    .get(index + 1)
                    .map_or("", |v| v.as_str())
                    .to_string();
                if name == "data" {
                    set.insert(value);
                }
            }
        }
    }

    log::info!("{:?}", set);

    Ok(())
}
