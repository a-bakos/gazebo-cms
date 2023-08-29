// Admin post/entry editor screen // move to admin/

use gazebo_core_common::entry::gb_post::GB_Post;
use yew::html::IntoPropValue;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::button::Button;
use crate::components::{admin_bar::AdminBar, entry_editor::EntryEditor};

use crate::components::input::Input;

#[function_component(EntryEdit)]
pub fn entry_edit() -> Html {
    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

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
                </section>
            </div>

        </main>
    }
}
