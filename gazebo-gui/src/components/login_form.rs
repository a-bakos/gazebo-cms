use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    app::MainNavigationRoute,
    components::{
        input::{Input, InputProps},
        lost_password::LostPassword,
    },
};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator();

    let mut login_error = String::new();

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
        let clone_navigator = navigator.clone();

        spawn_local(async move {
            let response = crate::api::user::api_login(cloned_username, cloned_password)
                .await
                .unwrap();
            println!("{}", response);

            // This currently matches all login requests regardless of correct credentials!
            if response == 1.to_string() {
                if let Some(nav) = clone_navigator {
                    nav.push(&MainNavigationRoute::Admin)
                }
            } else {
                // show login error here
                //login_error = "401 Unauthorized".to_string();
            }

            gloo_console::log!("Login request response: ", response);

            // On success, create a cookie
        });
    });

    html! {
        <div id={"gb-login-form"}>

            if login_error == "401 Unauthorized".to_string() {
                <p>{"LOGIN ERROR!"}</p>
            }

            <form
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
            <LostPassword />
        </div>
    }
}
