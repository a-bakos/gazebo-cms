// Admin post/entry editor screen // move to admin/

use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::{admin_bar::AdminBar, entry_editor::EntryEditor};

use gazebo_core_common::entry::gb_post::GB_Post;

use crate::api::post::api_new_entry_insert_request;
use crate::app::MainNavigationRoute;
use gazebo_core_common::account::gb_account::AccountID;
use gazebo_core_common::entry::entry_id::EntryID;
use gazebo_core_common::entry::entry_type::EntryType;
use gazebo_core_common::entry::gb_entry::{GB_EntryInsertRequest, GB_EntryInsertResponse};
use gazebo_core_common::entry::status::ContentStatus::Draft;
use gazebo_core_common::entry::status::EntryStatus;
use yew::html::IntoPropValue;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(EntryEdit)]
pub fn entry_edit() -> Html {
    let navigator = use_navigator();
    let on_form_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let clone_navigator = navigator.clone();

        // Console logging for now
        gloo_console::log!("Saving new post: ");

        let insert_entry_data = GB_EntryInsertRequest {
            author_id: 1001,
            slug: "".to_string(),
            title: "THE TITLE".to_string(),
            content: "THE CONTENTE".to_string(),
            status: "draft".to_string(),
            excerpt: None,
            password: None,
        };

        spawn_local(async move {
            let response: GB_EntryInsertResponse = api_new_entry_insert_request(insert_entry_data)
                .await
                .unwrap();
            match response.http_status_code {
                200 => {
                    gloo_console::log!(
                        "Entry successfully added: ",
                        response.http_status_code,
                        response.entry_id.0 // TODO this value is sticky, fix it
                    );

                    if let Some(nav) = clone_navigator {
                        // Redirect to post edit page using the returned ID
                        nav.push(&MainNavigationRoute::EntryEditExisting {
                            entry_type: EntryType::Post.to_string(),
                            id: response.entry_id.0.to_string(),
                        })
                    }
                }
                _ => {
                    gloo_console::log!("Error during saving entry!");
                }
            }
        });
    });

    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <form onsubmit={ on_form_submit }>
                <div class={"flex"}>
                    <nav class="bg-gray-300">
                        <ul class="w-max">
                            <li><button class="w-full bg-blue-400 px-4 block" href="">{"?MenuItem"}</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block" href="">{"?MenuItem"}</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block" href="">{ "3" }</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block" href="">{ "Publish" }</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block" href="">{ "Bin" }</button></li>
                        </ul>
                    </nav>
                    <section class="bg-white w-full p-4">
                        <h2 class="font-bold ">{"POST TITLE"}</h2>
                        <p class="font-bold ">{"PERMALINK"}</p>
                        <hr />
                        <EntryEditor />
                        <Button
                            button_type={"submit"}
                            label="Save Changes"
                        />
                    </section>
                </div>
            </form>
        </main>
    }
}
