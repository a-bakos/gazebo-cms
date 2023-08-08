use crate::api::post::{ContentStatus, EntryStatus, GB_Post};
use serde::Deserialize;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

use crate::app::MainNavigationRoute;
use yew_router::prelude::Link;

fn table_entry_row(row_data: &GB_Post) -> Html {
    let status = match row_data.status.clone() {
        // todo - will be added to common lib
        EntryStatus::Post(content_status) => match content_status {
            ContentStatus::Draft => "draft".to_string(),
            ContentStatus::Publish => "publish".to_string(),
            ContentStatus::Private => "private".to_string(),
            ContentStatus::Trash => "trash".to_string(),
            _ => "unknown".to_string(),
        },
        _ => "unknown".to_string(),
    };

    html! {
        <tr class="bg-white rounded-xl border hover:bg-yellow-100">
            <td>
                <Link<MainNavigationRoute>
                    to={MainNavigationRoute::EntryEdit}
                    classes="font-bold text-blue-600">
                    {row_data.title.clone()}
                </Link<MainNavigationRoute>>
                <span class="block">
                    <button class="underline">{"?view"}</button>
                    <button class="underline">{"?edit"}</button>
                    <button class="underline">{"?clone"}</button>
                    <button class="underline">{"?bin"}</button>
                </span>
            </td>
            <td>{"cat 1, cat 2"}</td>
            <td>{row_data.id_author.clone()}</td>
            <td>
                <span class="bg-pink-100 px-2 rounded-md">{status.clone()}</span>
                {
                    if status == "draft" {
                        html! {
                            <span class="block"><a href="" class="underline">{"publish now"}</a></span>
                        }
                    } else {
                        html! {}
                    }
                }
            </td>
            <td>
                <p>{row_data.date_publish.clone()}</p>
                <p>{"?by admin"}</p>
            </td>
            <td>
                <p>{row_data.date_modified.clone()}</p>
                <p>{"?by editor"}</p>
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
                            table_entry_row(entry_row)
                        } )
                    }
                </tbody>
            </table>
        </>
    }
}
