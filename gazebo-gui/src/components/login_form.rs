use crate::{
    api::account::api_login_request,
    app::MainNavigationRoute,
    components::{input::Input, lost_password::LostPassword},
    context::{
        CurrentUserContext, CurrentUserDispatchActions, UserAction::LoginSuccess, GB_TOKEN_KEY,
    },
};
use gazebo_core_common::account::login::LoginResponseWithStatusCode;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator();
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current accounts context is missing");

    let login_error = String::new();

    let username_handle = use_state(|| String::default());
    let username = (*username_handle).clone();
    let username_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(username_input) = target {
            username_handle.set(username_input.value());
        }
    });

    let password_handle = use_state(|| String::default());
    let password = (*password_handle).clone();
    let password_changed = Callback::from(move |event: Event| {
        let target = event.target_dyn_into::<web_sys::HtmlInputElement>();
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
            "Submitting login form: ",
            cloned_username.clone(),
            cloned_password.clone()
        );

        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        let clone_navigator = navigator.clone();
        let clone_current_user_ctx = current_user_ctx.clone();

        spawn_local(async move {
            // we can match on response instead
            let response: LoginResponseWithStatusCode =
                api_login_request(cloned_username, cloned_password)
                    .await
                    .unwrap();

            // Check status code
            // Todo create type for statuscode
            match response.http_status_code {
                200 => {
                    // IDEA todo: gb_console_log(GB_Log::LoginResponseWithStatusCode, response)
                    gloo_console::log!(
                        "Successful login: ",
                        response.http_status_code,
                        response.account_details.id.0.clone(),
                        response.account_details.login_name.clone(),
                        response.account_details.token.clone()
                    );

                    clone_current_user_ctx.dispatch(CurrentUserDispatchActions {
                        action_type: LoginSuccess,
                        login_response: Some(response.account_details),
                    });

                    // Redirect to Home
                    if let Some(nav) = clone_navigator {
                        nav.push(&MainNavigationRoute::Home)
                    }
                }
                _ => {
                    gloo_console::log!("Login error!");
                }
            }
        });
    });

    html! {
        <>
            if login_error == "401 Unauthorized".to_string() {
                <p>{"LOGIN ERROR!"}</p>
            }

            <form
                class={"bg-white p-4 border-2 rounded-lg w-max content-center mx-auto"}
                onsubmit={on_form_submit}>

                <Input
                    label={"Username or Email"}
                    id={"gb-login-form-accounts"}
                    name={"gb-login-form-accounts"}
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
                <button
                    class={"w-full mt-4 p-4 border-2 rounded-lg py-2 text-2xl block hover:bg-red-300"}
                    type="submit">
                    {"Login"}
                </button>
            </form>
            <LostPassword />
        </>
    }
}
