use crate::api::account::LoginResponseAccountDetails;
use gazebo_core_common::account::gb_account::GB_CurrentAccount;
use std::rc::Rc;
use yew::{context::ContextProvider, prelude::*, Reducible, UseReducerHandle};

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

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            UserAction::LoginSuccess => {
                let login_response = action.login_response.expect("Missing login response");
                Self {
                    user: Some(GB_CurrentAccount {
                        id: login_response.id,
                        username: login_response.login_name,
                        email: login_response.email,
                        role: login_response.role,
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
    pub login_response: Option<LoginResponseAccountDetails>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);
    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}
