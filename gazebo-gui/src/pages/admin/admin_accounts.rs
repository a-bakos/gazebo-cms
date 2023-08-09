// Admin Users screen

use yew::prelude::*;


use crate::components::{
    accounts_table::AccountsTable, admin_bar::AdminBar
};

#[function_component(AdminAccounts)]
pub fn admin_accounts() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <div class={"flex bg-red-100"}>
                <div class={"w-full"}>
                   <h1>{"Accounts"}</h1>
                    <AccountsTable />
                </div>



            </div>

        </main>
    }
}
