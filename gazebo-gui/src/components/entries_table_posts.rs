use crate::api::post::{ContentStatus, EntryStatus, GB_Post};
use serde::Deserialize;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

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
        <tr>
            <td>
                <a
                    title={ row_data.title.clone() }
                    href="#">
                    { row_data.title.clone() } { row_data.id.clone() }
                </a>
            </td>
            <td>{status}</td>
            <td>{row_data.id_author.clone()}</td>
            <td>{"Category TBC"}</td>
            <td>{row_data.date_publish.clone()}</td>
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
            <table class={"gb-admin-table"}>
                <thead>
                    <tr>
                        <th>{"Title"}</th>
                        <th>{"Status"}</th>
                        <th>{"Author"}</th>
                        <th>{"Category"}</th>
                        <th>{"Published"}</th>
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
