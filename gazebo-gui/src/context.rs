use crate::api::account::{api_me, LoginResponseAccountDetails};
use gazebo_core_common::account::gb_account::GB_CurrentAccount;
use gloo_storage::{SessionStorage, Storage};
use std::rc::Rc;
use yew::platform::spawn_local;
use yew::{context::ContextProvider, prelude::*, Reducible, UseReducerHandle};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<GB_CurrentAccount>,
    pub token: Option<String>,
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
                Self {
                    // CurrentUser
                    user: Some(GB_CurrentAccount {
                        id: login_response.id,
                        username: login_response.login_name,
                        email: login_response.email,
                        role: login_response.role,
                    }),
                    token: Some(login_response.token),
                }
                .into()
            }
            UserAction::LoginFailure => Self {
                user: None,
                token: None,
            }
            .into(),
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: UserAction,
    pub login_response: Option<LoginResponseAccountDetails>,
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
                match api_me(&token).await {
                    Ok(me_response) => {
                        gloo_console::log!("SERVER RESPONSE: {}!", me_response.clone());
                        // gloo_console::log!("TOKEN {}!", token.clone());
                        cloned_user.dispatch(CurrentUserDispatchActions {
                            action_type: UserAction::LoginSuccess,
                            login_response: Some(LoginResponseAccountDetails {
                                // todo!
                                id: 0,
                                login_name: "LOIGNNAME".to_string(),
                                email: "".to_string(),
                                role: "".to_string(),
                                token: token.to_string(),
                            }),
                        });
                    }
                    Err(_) => SessionStorage::clear(),
                }
            });
        }
    } else {
    }

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}
