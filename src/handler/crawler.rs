use std::sync::Arc;

use headless_chrome::protocol::cdp::types::Event;
use headless_chrome::protocol::cdp::Network::ResourceType;
use headless_chrome::protocol::cdp::Page::HandleJavaScriptDialog;
use headless_chrome::protocol::cdp::Runtime::Evaluate;
use headless_chrome::{Browser, LaunchOptions};

use crate::handler::form::{Html, FORM};
use crate::handler::form_js::JS_CODE;
use crate::{common, model};

pub fn browse_wikipedia(
    config: model::task::TaskConfig,
    launch_options: LaunchOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::new(launch_options)?;
    let random_ug = common::user_agent::random_user_agent();
    for item in &config.target {
        let tab = browser.new_tab()?;
        let tab_clone = Arc::clone(&tab);
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
            Event::NetworkRequestWillBeSent(e) => match e.params.Type {
                Some(ResourceType::Document) | Some(ResourceType::Xhr) => {
                    log::info!("req url:{}", e.params.request.url)
                }
                _ => (),
            },
            Event::NetworkResponseReceived(e) => match e.params.Type {
                ResourceType::Document | ResourceType::Xhr => {
                    log::info!("rsp url:{}", e.params.response.url)
                }
                _ => (),
            },
            _ => (),
        }))?;
        tab.enable_fetch(None, Some(true))?;
        tab.authenticate(config.username.clone(), config.password.clone())?;
        tab.set_user_agent(random_ug.as_str(), None, None).unwrap();
        tab.navigate_to(item)?;
        tab.set_extra_http_headers(
            config
                .headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        )
        .unwrap();
        let ug = tab
            .evaluate("window.navigator.userAgent", false)?
            .value
            .unwrap();
        assert_eq!(random_ug, ug);
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
        _ = tab.close(true);
    }

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
