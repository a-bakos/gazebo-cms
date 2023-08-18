use crate::api::post::{
    update_entry_single_param, ContentStatus, EntryStatus, EntryUpdateProps, EntryUpdateType,
    GB_Post,
};
use crate::app::MainNavigationRoute;
use crate::{
    api::post::api_delete_entry_by_id,
    components::{
        button::Button,
        button::{ButtonProps, FormWithButton},
        input::Input,
    },
};

use gazebo_core_common::entry::entry_type::EntryType;

use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostTableRowProps {
    pub row_data: GB_Post,
}

#[function_component(PostTableRow)]
pub fn table_entry_row(props: &PostTableRowProps) -> Html {
    let (status_label, status_label_class) = match props.row_data.status.clone() {
        // todo - will be added to common lib
        EntryStatus::Post(content_status) => match content_status {
            ContentStatus::Draft => ("draft".to_string(), "bg-pink-200"),
            ContentStatus::Publish => ("publish".to_string(), "bg-green-200"),
            ContentStatus::Private => ("private".to_string(), "bg-yellow-200"),
            ContentStatus::Trash => ("trash".to_string(), "bg-gray-200"),
            _ => ("unknown".to_string(), "bg-white-100"),
        },
        _ => ("unknown".to_string(), "bg-white-100"),
    };

    let post_id = props.row_data.id.clone();
    let navigator = use_navigator();

    // Button event: Move post to bin // TODO reload after callback completion
    let on_form_submit_bin = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let clone_navigator = navigator.clone();
        spawn_local(async move {
            match update_entry_single_param(
                EntryType::Post,
                post_id.clone(),
                EntryUpdateProps {
                    to_update: "status",
                    value: "trash",
                },
            )
            .await
            {
                Ok(response) => {
                    gloo_console::log!(response);
                    // TODO
                    if let Some(nav) = clone_navigator {
                        nav.push(&MainNavigationRoute::Admin)
                    }
                }
                Err(err) => gloo_console::log!(format!("{:?}", err)),
            }
        });
    });

    // Button event: Publish draft post // TODO reload after callback completion
    let on_form_submit_publish = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spawn_local(async move {
            match update_entry_single_param(
                EntryType::Post,
                post_id,
                EntryUpdateProps {
                    to_update: "status",
                    value: "publish",
                },
            )
            .await
            {
                Ok(response) => gloo_console::log!(response),
                Err(err) => gloo_console::log!(format!("{:?}", err)),
            }
        });
    });

    html! {
        <tr class="bg-white rounded-xl border hover:bg-yellow-100">
            <td>
                <Link<MainNavigationRoute>
                    to={MainNavigationRoute::EntryEditExisting { entry_type: EntryType::Post.to_string(), id: post_id.to_string() }}
                    classes="font-bold text-blue-600">
                    {props.row_data.title.clone()}
                </Link<MainNavigationRoute>>
                <span class="block">
                    <a class="underline mr-1">{ "?view" }</a>
                    <a class="underline mr-1">{ "?edit" }</a>
                    <button class="underline mr-1">{ "?clone" }</button>

                    <form
                        class={"inline"}
                        onsubmit={on_form_submit_bin}>
                        <Button
                            button_type="submit"
                            label="Bin it!"
                        />
                    </form>

                </span>
            </td>
            <td>{"cat 1, cat 2"}</td>
            <td>{props.row_data.id_author.clone()}</td>
            <td>
                <span class={ format!("{} px-2 rounded-md", status_label_class) }>
                    { status_label.clone() }
                </span>
                {
                    if status_label == "draft" {
                        html! {
                            <form
                                class={"block"}
                                onsubmit={on_form_submit_publish}>
                                <Button
                                    button_type="submit"
                                    label="publish now"
                                />
                            </form>
                        }
                    } else {
                        html! {}
                    }
                }
            </td>
            <td>
                <p>{ props.row_data.date_publish.clone() }</p>
                <p>{ "?by admin" }</p>
            </td>
            <td>
                <p>{ props.row_data.date_modified.clone() }</p>
                <p>{ "?by editor" }</p>
            </td>
        </tr>
    }
}

#[function_component(EntriesTable)]
pub fn table_entries() -> Html {
    let row_titles_handle = use_state(|| Vec::<GB_Post>::new());
    let row_titles = row_titles_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let mut entry_rows: Vec<GB_Post> = vec![];
                let response = crate::api::post::api_get_all_posts().await.unwrap();
                for response_gb_post in response.iter() {
                    gloo_console::log!("response: ", response_gb_post.title.clone());
                    entry_rows.push(response_gb_post.clone());
                }
                row_titles_handle.set(entry_rows);
            });
            || ()
        },
        (),
    );

    html! {
        <>
            <table class={"text-left w-full border"}>
                <thead>
                    <tr>
                        <th>{"Title"}</th>
                        <th>{"Category"}</th>
                        <th>{"Author"}</th>
                        <th>{"Status"}</th>
                        <th>{"Created"}</th>
                        <th>{"Modified"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for row_titles.iter().map(|entry_row| html! {
                            // table_entry_row(entry_row)
                            <PostTableRow row_data={entry_row.clone()} />
                        } )
                    }
                </tbody>
            </table>
        </>
    }
}
