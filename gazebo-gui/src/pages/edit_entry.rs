// Admin post/entry editor screen // move to admin/

use crate::components::button::Button;
use crate::components::{admin_bar::AdminBar, entry_editor::EntryEditor};
use gazebo_core_common::entry::gb_post::GB_Post;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EntryEditorProps {
    pub entry_type: AttrValue,
    pub entry_id: AttrValue,
}

#[function_component(EntryEditExisting)]
pub fn entry_edit_existing(props: &EntryEditorProps) -> Html {
    let entry_type = props.entry_type.clone().to_string();
    let entry_id = props.entry_id.clone().to_string();

    let single_entry_editor_handle = use_state(|| GB_Post::new());
    let single_entry = single_entry_editor_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let response = crate::api::post::api_get_single_post(entry_type, entry_id)
                    .await
                    .unwrap();
                single_entry_editor_handle.set(response);
            });
            || ()
        },
        (),
    );

    html! {
         <main id={crate::consts::CSS_ID_ADMIN_AREA}>
            <AdminBar />

            <form>
                <input
                    type={"hidden"}
                    name={"gb_author_id"}
                    value={"1000"}
                />

                <div class={"flex"}>
                    <nav class="bg-gray-300 w-1/6">
                        <ul class="w-full">
                            <li><button class="w-full bg-blue-400 px-4 block hover:bg-red-200" href="">{"?MenuItem"}</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block hover:bg-red-200" href="">{"?MenuItem"}</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block hover:bg-red-200" href="">{ "3" }</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block hover:bg-red-200" href="">{ "Publish" }</button></li>
                            <li><button class="w-full bg-blue-400 px-4 block hover:bg-red-200" href="">{ "Bin" }</button></li>
                        </ul>
                    </nav>
                    <section class="bg-white w-5/6 p-4">
                        <h2 class="font-bold ">{ single_entry.title.clone() }</h2>
                        <p class="font-bold ">{"Permalink: "}{ single_entry.slug.clone() }</p>
                        <hr />
                        <EntryEditor
                            title={ single_entry.title.clone() }
                            // id={ single_entry.id.clone() }
                            // id_author={ single_entry.id_author }
                            // id_parent={ single_entry.id_parent }
                            // date_publish={ single_entry.date_publish.clone() }
                            date_modified={ single_entry.date_modified.clone() }
                            slug={ single_entry.slug.clone() }
                            // status={ single_entry.status.into_prop_value() }
                            excerpt={ single_entry.excerpt.clone() }
                            content={ single_entry.content.clone() }
                            password={ single_entry.password.clone() }
                        />
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
