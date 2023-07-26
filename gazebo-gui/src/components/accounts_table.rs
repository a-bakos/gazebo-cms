use serde::Deserialize;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

fn table_entry_row() -> Html {
    html! {
        <tr>
            <td>{"ID"}</td>
            <td>{"Name"}</td>
            <td>{"Email"}</td>
            <td>{"login"}</td>
            <td>{"role"}</td>
            <td>{"registered"}</td>
            <td>{"last login"}</td>
        </tr>
    }
}

#[function_component(AccountsTable)]
pub fn table_accounts() -> Html {
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
                        table_entry_row()
                    }
                </tbody>
            </table>
        </>
    }
}
