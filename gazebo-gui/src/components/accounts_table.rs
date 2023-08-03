use crate::api::user::{AccountRole, GB_Account};
use serde::Deserialize;
use std::thread::spawn;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

fn table_account_row(row_data: &GB_Account) -> Html {
    let role = match row_data.role.clone() {
        AccountRole::Admin => "admin".to_string(),
        AccountRole::Editor => "editor".to_string(),
        AccountRole::Contributor => "contributor".to_string(),
        _ => "unknown".to_string(),
    };

    html! {
        <tr>
            <td>{row_data.id.clone()}</td>
            <td>{"Name"}</td>
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
                let response = crate::api::user::api_get_all_accounts().await.unwrap();
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
            <table class={"gb-admin-table"}>
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"Name"}</th>
                        <th>{"Email"}</th>
                        <th>{"login"}</th>
                        <th>{"role"}</th>
                        <th>{"registered"}</th>
                        <th>{"last login"}</th>
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
