use crate::api::post::{ContentStatus, EntryStatus};
use serde::Deserialize;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

// TODO - rename this to Post and create a separate file for the Page type

#[derive(Clone, PartialEq, Deserialize)]
pub struct EntryTableRow {
    //category: Option<Vec<String>>,
    pub id: u32,
    pub id_author: u32,
    pub id_parent: Option<u32>,
    pub date_publish: String,
    pub date_modified: String,
    pub slug: Option<String>,
    pub status: String,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub password: Option<String>,
}

fn table_entry_row(row_data: &EntryTableRow) -> Html {
    html! {
        <tr>
            <td>
                <a
                    title={ row_data.title.clone() }
                    href="#">
                    { row_data.title.clone() } { row_data.id.clone() }
                </a>
            </td>
            <td>{row_data.status.clone()}</td>
            <td>{row_data.id_author.clone()}</td>
            <td>{"Category TBC"}</td>
            <td>{row_data.date_publish.clone()}</td>
        </tr>
    }
}

#[function_component(EntriesTable)]
pub fn table_entries() -> Html {
    let row_titles_handle = use_state(|| Vec::<EntryTableRow>::new());
    let row_titles = row_titles_handle.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let mut entry_rows: Vec<EntryTableRow> = vec![];
                let response = crate::api::post::api_get_all_posts().await.unwrap();
                // todo - now make this work with the full post structure
                for response_gb_post in response.iter() {
                    gloo_console::log!("response: ", response_gb_post.title.clone());

                    let status = match response_gb_post.status.clone() {
                        EntryStatus::Post(content_status) => match content_status {
                            ContentStatus::Draft => "draft".to_string(),
                            ContentStatus::Publish => "publish".to_string(),
                            ContentStatus::Private => "private".to_string(),
                            ContentStatus::Trash => "trash".to_string(),
                            _ => "unknown".to_string(),
                        },
                        _ => "unknown".to_string(),
                    };

                    let entry_row = EntryTableRow {
                        id: response_gb_post.id,
                        id_author: response_gb_post.id_author,
                        id_parent: response_gb_post.id_parent,
                        date_publish: response_gb_post.date_publish.clone(),
                        date_modified: response_gb_post.date_modified.clone(),
                        slug: response_gb_post.slug.clone(),
                        status,
                        title: response_gb_post.title.clone(),
                        excerpt: response_gb_post.excerpt.clone(),
                        content: response_gb_post.content.clone(),
                        password: response_gb_post.password.clone(),
                    };
                    entry_rows.push(entry_row);
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
