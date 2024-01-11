use std::sync::Arc;

use headless_chrome::protocol::cdp::types::Event;
use headless_chrome::protocol::cdp::Network::ResourceType;
use headless_chrome::protocol::cdp::Page::{
    AddScriptToEvaluateOnNewDocument, HandleJavaScriptDialog, SetDownloadBehavior,
    SetDownloadBehaviorBehaviorOption,
};
use headless_chrome::protocol::cdp::Runtime::{AddBinding, Evaluate};
use headless_chrome::{Browser, Tab};
use tokio::sync::mpsc;

use crate::handler::collect::collect;
use crate::handler::form::{Html, FORM};
use crate::handler::form_js::{JS_CODE, TAB_INIT};
use crate::{common, model};

pub fn tasks(
    url: &str,
    tx: mpsc::Sender<String>,
    browser: Browser,
    config: &model::task::TaskConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let random_ug = common::user_agent::random_user_agent();
    let tab = browser.new_tab()?;
    tab.enable_runtime()?;
    tab.enable_fetch(None, Some(true))?;
    tab.authenticate(config.username.clone(), config.password.clone())?;
    tab.set_user_agent(random_ug.as_str(), None, None).unwrap();
    tab.call_method(add_binding("addLink"))?;
    tab.call_method(add_binding("Test"))?;
    tab.call_method(AddScriptToEvaluateOnNewDocument {
        source: TAB_INIT.to_string(),
        world_name: None,
        include_command_line_api: None,
    })?;
    tab.set_extra_http_headers(
        config
            .headers
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect(),
    )
    .unwrap();
    tab.call_method(SetDownloadBehavior {
        behavior: SetDownloadBehaviorBehaviorOption::Deny,
        download_path: None,
    })?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;
    let tab_clone = Arc::clone(&tab);
    event_listener(&tab, tab_clone, tx)?;
    let result = tab.call_method(evaluate())?;
    if let Some(result_value) = result.result.value {
        let list: Vec<Html> =
            serde_json::from_str(&result_value.to_string()).expect("Failed to parse JSON");
        for item in list {
            if let Some(func) = FORM.get(item.el_type.as_str()) {
                func(tab.clone(), item);
            } else {
                log::warn!("not el type: {}", item.el_type);
            }
        }
    }
    collect(&tab);
    _ = tab.close(true);

    Ok(())
}

fn event_listener(
    tab: &Arc<Tab>,
    tab_clone: Arc<Tab>,
    tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    tab.add_event_listener(Arc::new(move |event: &Event| match event {
        Event::PageWindowOpen(_) => _ = tab_clone.close_target().unwrap(),
        Event::PageJavascriptDialogOpening(_) => {
            _ = tab_clone
                .call_method(HandleJavaScriptDialog {
                    accept: false,
                    prompt_text: None,
                })
                .unwrap()
        }
        Event::PageLoadEventFired(_) => log::info!("load event fired"),
        Event::PageDomContentEventFired(_) => log::info!("dom content event fired"),
        Event::NetworkRequestWillBeSent(e) => match e.params.Type {
            Some(ResourceType::Document) | Some(ResourceType::Xhr) => {
                if tx.blocking_send(e.clone().params.request.url).is_err() {
                    log::error!("Failed to send URL through channel");
                }
            }
            _ => (),
        },
        Event::NetworkResponseReceived(e) => match e.params.Type {
            ResourceType::Document | ResourceType::Xhr => {
                if tx.blocking_send(e.clone().params.response.url).is_err() {
                    log::error!("Failed to send URL through channel");
                }
            }
            _ => (),
        },
        Event::RuntimeBindingCalled(_) => log::info!("binding called"),
        _ => (),
    }))?;
    Ok(())
}

fn evaluate() -> Evaluate {
    Evaluate {
        expression: JS_CODE.to_string(),
        return_by_value: Some(true),
        generate_preview: Some(true),
        silent: Some(false),
        await_promise: None,
        include_command_line_api: Some(false),
        user_gesture: Some(false),
        object_group: None,
        context_id: None,
        throw_on_side_effect: None,
        timeout: None,
        disable_breaks: None,
        repl_mode: None,
        allow_unsafe_eval_blocked_by_csp: None,
        unique_context_id: None,
    }
}

fn add_binding(name: &str) -> AddBinding {
    AddBinding {
        name: name.to_string(),
        execution_context_id: None,
        execution_context_name: None,
    }
}
