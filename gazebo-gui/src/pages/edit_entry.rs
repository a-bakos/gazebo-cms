// TODO WIP

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

    let input_change_handler = (|handle: UseStateHandle<String>| {
        let value = (*handle).clone();
        let value_changed = Callback::from(move |event: Event| {
            let target = event.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(value_input) = target {
                handle.set(value_input.value());
            }
        });
        let cloned_value = value.clone();
        (value, cloned_value, value_changed)
    });

    let (title, cloned_title, title_changed) =
        input_change_handler(use_state(|| String::default()));

    let (permalink, cloned_permalink, permalink_changed) =
        input_change_handler(use_state(|| String::default()));

    let (excerpt, cloned_excerpt, excerpt_changed) =
        input_change_handler(use_state(|| String::default()));

    let (content, cloned_content, content_changed) =
        input_change_handler(use_state(|| String::default()));

    let (password, cloned_password, password_changed) =
        input_change_handler(use_state(|| String::default()));

    let current_user_ctx = use_context::<crate::context::CurrentUserContext>()
        .expect("Current accounts context missing");
    let account_id = match &current_user_ctx.user {
        Some(user) => user.id.clone(),
        None => AccountID(0),
    };

    let on_form_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let clone_navigator = navigator.clone();
        let account_id = account_id.clone().0;

        let cloned_title = cloned_title.clone();
        let cloned_permalink = cloned_permalink.clone();
        let cloned_excerpt = cloned_excerpt.clone();
        let cloned_content = cloned_content.clone();
        let cloned_password = cloned_password.clone();

        gloo_console::log!("{}", cloned_excerpt.clone());

        let update_entry_data = GB_EntryUpdateRequest {
            author_id: account_id as i32, // get_current_account_id()
            slug: cloned_permalink,
            title: cloned_title,
            content: cloned_content,
            status: "draft".to_string(),
            excerpt: Some(cloned_excerpt),
            password: Some(cloned_password),
        };

        spawn_local(async move {
            let response: GB_EntryUpdateResponse =
                api_entry_update_request(update_entry_data).await.unwrap();
            match response.http_status_code {
                200 => {
                    gloo_console::log!(
                        "Entry successfully added: ",
                        response.http_status_code,
                        response.entry_id.0
                    );
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
                            title={ title }
                            title_changed={ title_changed }
                            permalink={ permalink }
                            permalink_changed={ permalink_changed }
                            excerpt={ excerpt }
                            excerpt_changed={ excerpt_changed }
                            content={ content }
                            content_changed={ content_changed }
                            password={ password }
                            password_changed={ password_changed }
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
