use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::input::{Input, InputProps};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(|| String::default());
    let username = (*username_handle).clone();
    let username_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>(); // we need web_sys for this type
        if let Some(username_input) = target {
            username_handle.set(username_input.value());
        }
    });

    let password_handle = use_state(|| String::default());
    let password = (*password_handle).clone();
    let password_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>(); // we need web_sys for this type
        if let Some(password_input) = target {
            password_handle.set(password_input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let on_form_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        // Console logging for now
        gloo_console::log!(
            "Submitting form: ",
            cloned_username.clone(),
            cloned_password.clone()
        );

        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        spawn_local(async move {
            let response = crate::api::user::api_login(cloned_username, cloned_password)
                .await
                .unwrap();
            println!("{}", response);

            gloo_console::log!("Login request response: ", response);
        });
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
