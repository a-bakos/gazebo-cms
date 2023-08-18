use crate::app::MainNavigationRoute;
use gazebo_core_common::{
    account::{gb_account::GB_Account, role::AccountRole},
    consts::{
        ACCOUNT_ROLE_ADMIN, ACCOUNT_ROLE_CONTRIBUTOR, ACCOUNT_ROLE_EDITOR, ACCOUNT_ROLE_NOT_FOUND,
    },
};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Link;

fn table_account_row(row_data: &GB_Account) -> Html {
    let role = match row_data.role.clone() {
        AccountRole::Admin => ACCOUNT_ROLE_ADMIN.to_string(),
        AccountRole::Editor => ACCOUNT_ROLE_EDITOR.to_string(),
        AccountRole::Contributor => ACCOUNT_ROLE_CONTRIBUTOR.to_string(),
        _ => ACCOUNT_ROLE_NOT_FOUND.to_string(),
    };

    html! {
        <tr>
            <td>{row_data.id.clone()}</td>
            <td>
                {"Name / "}
                <Link<MainNavigationRoute> to={MainNavigationRoute::AdminProfile}>
                    {"PROFILE EDIT"}
                </Link<MainNavigationRoute>>
            </td>
            <td>{row_data.email.clone()}</td>
            <td>{row_data.login_name.clone()}</td>
            <td>{role}</td>
            <td>{row_data.registered.clone()}</td>
            <td>{row_data.last_login.clone()}</td>
        </tr>
    }
}

#[function_component(AccountsTable)]
pub fn table_accounts() -> Html {
    let row_accounts_handle = use_state(|| Vec::<GB_Account>::new());
    let row_accounts = row_accounts_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let mut accounts_rows: Vec<GB_Account> = vec![];
                let response = crate::api::account::api_get_all_accounts().await.unwrap();
                for response_gb_account in response.iter() {
                    gloo_console::log!("response: ", response_gb_account.email.clone());
                    accounts_rows.push(response_gb_account.clone());
                }
                row_accounts_handle.set(accounts_rows);
            });
            || ()
        },
        (),
    );

    html! {
        <>
            <table class={"w-full bg-blue-100"}>
                <thead>
                    <tr class={"text-left"}>
                        <th>{"ID"}</th>
                        <th>{"Name"}</th>
                        <th>{"Email"}</th>
                        <th>{"Login name"}</th>
                        <th>{"Role"}</th>
                        <th>{"Registered"}</th>
                        <th>{"Last login"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for row_accounts.iter().map(|account_row| html! {
                            table_account_row(account_row)
                        } )
                    }
                </tbody>
            </table>
        </>
    }
}
