// Admin all posts/entries page

use yew::prelude::*;

use crate::components::{
    admin_bar::AdminBar, button_add_new_entry::ButtonAddNewEntry, entries_table_posts::EntriesTable,
};

#[function_component(AdminPosts)]
pub fn admin_posts() -> Html {
    html! {
        <main id={crate::consts::CSS_ID_ADMIN_AREA} class={"text-[#3c352d]"}>
            <AdminBar />

            <div class={"flex"}>
                <nav class="bg-[#688a72] text-white">
                    <ul class="w-max">
                        <li><button class="w-full px-4 block" href="">{"?Category"}</button></li>
                        <li><button class="w-full px-4 block" href="">{"?Settings"}</button></li>
                        <li><button class="w-full px-4 block" href="">{ "3" }</button></li>
                        <li><button class="w-full px-4 block" href="">{ "4" }</button></li>
                        <li><button class="w-full px-4 block" href="">{ "5" }</button></li>
                    </ul>
                </nav>
                <section class="bg-[#faf7f2] w-full p-4">
                    <h2 class="font-bold ">{"POST ENTRIES"}</h2>
                    <div class="flex w-full bg-gray-100 border">
                        <ButtonAddNewEntry />
                        <button class="px-4 py-2 mr-px bg-[#607a4b] hover:bg-blue-500">{"Publish"}</button>
                        <button class="px-4 py-2 mr-px bg-[#eda52d] hover:bg-blue-500">{"Draft"}</button>
                        <button class="px-4 py-2 mr-px bg-[#cf743d] hover:bg-blue-500">{"Private"}</button>
                        <button class="px-4 py-2 mr-px bg-[#b72224] hover:bg-blue-500">{"Bin"}</button>
                        <input type="text" class="px-4 py-2 mr-px" />
                        <button class="bg-blue-200 px-4 py-2 mr-px hover:bg-blue-500">{"Search"}</button>
                        <button class="bg-blue-200 px-4 py-2 mr-px hover:bg-blue-500">{"Export"}</button>
                    </div>
                    <EntriesTable />
                </section>
            </div>

        </main>
    }
}
