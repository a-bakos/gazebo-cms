use crate::api::account::api_auth_me;
use gazebo_core_common::account::{auth::AuthResponsePayload, gb_account::GB_CurrentAccount};

use gazebo_core_common::account::gb_account::AccountID;
use gloo_storage::{SessionStorage, Storage};
use std::rc::Rc;
use yew::{
    context::ContextProvider, platform::spawn_local, prelude::*, Reducible, UseReducerHandle,
};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<GB_CurrentAccount>,
}

pub enum UserAction {
    LoginSuccess,
    #[allow(dead_code)]
    LoginFailure,
}

pub const GB_TOKEN_KEY: &str = "gb_token";

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            UserAction::LoginSuccess => {
                let login_response = action.login_response.expect("Missing login response");
                // Set session token on successful login
                let _ = SessionStorage::set(GB_TOKEN_KEY, login_response.token.clone());
                gloo_console::log!("session storage set!");
                Self {
                    // CurrentUser
                    user: Some(GB_CurrentAccount {
                        id: login_response.account_details.id,
                        username: login_response.account_details.login_name,
                        role: login_response.account_details.role,
                    }),
                }
                .into()
            }
            UserAction::LoginFailure => Self { user: None }.into(),
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: UserAction,
    pub login_response: Option<AuthResponsePayload>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);
    if user.user.is_none() {
        // current user not found (eg. page reload)
        // Look for token in session storage
        if let Ok(token) = SessionStorage::get::<String>(GB_TOKEN_KEY) {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_auth_me(&token).await {
                    Ok(me_response) => {
                        // TODO refactor: currently, every successful auth me request end with re-setting the token in storage
                        // think about how we can avoid sending the token back from the backend after the initial login has succeeded
                        cloned_user.dispatch(CurrentUserDispatchActions {
                            action_type: UserAction::LoginSuccess,
                            login_response: Some(me_response),
                        });
                    }
                    Err(_) => {
                        SessionStorage::clear();
                    }
                }
            });
        };
    };
    //else {
    //}

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}
