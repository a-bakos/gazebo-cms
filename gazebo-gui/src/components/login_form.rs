use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: AttrValue,
    pub id: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <>
            <label for={props.id.clone()}>{props.label.clone()}</label>
            <input
                id={props.id.clone()}
                type={props.input_type.clone()}
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </>
    }
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(|| String::default());
    let username = (*username_handle).clone();
    let username_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>(); // we need web_sys for this type
        if let Some(input) = target {
            username_handle.set(input.value());
        }
    });

    let password_handle = use_state(|| String::default());
    let password = (*password_handle).clone();
    let password_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>(); // we need web_sys for this type
        if let Some(input) = target {
            password_handle.set(input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let on_form_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        gloo_console::log!("Submitting form");
        gloo_console::log!(cloned_username.clone());
        gloo_console::log!(cloned_password.clone());
    });

    html! {
        <form
            id="gb-login-form"
            onsubmit={on_form_submit}>

            <Input
                label={"Username or Email"}
                id={"gb-login-form-user"}
                name={"gb-login-form-user"}
                input_type={"text"}
                value={username}
                onchange={username_changed}
            />

            <Input
                label={"Password"}
                id={"gb-login-form-pass"}
                name={"gb-login-form-pass"}
                input_type={"password"}
                value={password}
                onchange={password_changed}
            />

            <button type="submit">{"Login"}</button>
        </form>
    }
}
