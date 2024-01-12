use headless_chrome::protocol::cdp::Runtime::Evaluate;

pub fn evaluate(v: &str) -> Evaluate {
    Evaluate {
        expression: v.to_string(),
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
