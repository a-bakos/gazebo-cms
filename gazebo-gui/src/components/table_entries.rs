use serde::Deserialize;
use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

// TODO - rename this to Post and create a separate file for the Page type

#[derive(Clone, PartialEq, Deserialize)]
struct EntryTableRow {
    title: String,
    status: String,
    author: String,
    category: Option<Vec<String>>,
    date: String,
    id: u32,
}

fn table_entry_row(row_data: &EntryTableRow) -> Html {
    html! {
        <tr>
            <td>
                <a
                    title={ row_data.title.clone() }
                    href="#">
                    { row_data.title.clone() }
                </a>
            </td>
            <td>{row_data.status.clone()}</td>
            <td>{row_data.author.clone()}</td>
            <td>{"Category TBC"}</td>
            <td>{row_data.date.clone()}</td>
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
                for title in response.iter() {
                    gloo_console::log!("response: ", title);

                    let entry_row = EntryTableRow {
                        title: title.to_string(),
                        status: "status".to_string(),
                        author: "author".to_string(),
                        category: Some(vec!["cat".to_string()]),
                        date: "date".to_string(),
                        id: 1000,
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
