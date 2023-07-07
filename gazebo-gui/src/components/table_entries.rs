use yew::prelude::*;
use yew::{platform::spawn_local, prelude::*};

struct TableRows {
    rows: Vec<EntryTableRow>,
}

struct EntryTableRow {
    title: String,
    status: String,
    author: String,
    category: Option<Vec<String>>,
    date: String,
    id: u32,
}

fn table_entry_row(row_data: EntryTableRow) -> Html {
    html! {
        <tr>
            <td>{row_data.title.clone()}</td>
            <td>{row_data.status.clone()}</td>
            <td>{row_data.author.clone()}</td>
            <td>{"None"}</td> // category
            <td>{row_data.date.clone()}</td>
        </tr>
    }
}

#[function_component(EntriesTable)]
pub fn table_entries() -> Html {
    let mut table_rows = TableRows {
        rows: vec![
            EntryTableRow {
                title: "Post title 1".to_string(),
                status: "publish".to_string(),
                author: "author1".to_string(),
                category: Some(vec![
                    "category1".to_string(),
                    "category2".to_string(),
                    "category3".to_string(),
                ]),
                date: "2023-06-29 21:27".to_string(),
                id: 1,
            },
            EntryTableRow {
                title: "Post title 2".to_string(),
                status: "publish".to_string(),
                author: "author2".to_string(),
                category: None,
                date: "2023-06-28 21:27".to_string(),
                id: 2,
            },
            EntryTableRow {
                title: "Post title 3".to_string(),
                status: "publish".to_string(),
                author: "author3".to_string(),
                category: Some(vec!["category1".to_string()]),
                date: "2023-06-27 21:27".to_string(),
                id: 3,
            },
        ],
    };

    // TODO
    spawn_local(async move {
        let response = crate::api::post::api_get_all_posts().await.unwrap();

        for title in response.iter() {
            gloo_console::log!("response: ", title);
        }
    });

    html! {
        <>
          // {
          //   for response.iter().map(|title| html! {
          //     <p>{title.clone()}</p>
          //   } )
          // }
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
                        for table_rows.rows.iter().map(|row| html! {
                            <tr>
                                <td>
                                    <a
                                        title={ row.title.clone() }
                                        href="">{ row.title.clone() }</a>
                                </td>
                                <td>{ &row.status }</td>
                                <td>{ &row.author }</td>
                                <td>
                                    {
                                        if let Some(categories) = &row.category {
                                            html! {
                                                <>
                                                    {
                                                        for categories.iter().map(|tag| html! {
                                                            <>
                                                                <span>{ tag }</span>
                                                                <br />
                                                            </>
                                                        } )
                                                    }
                                                </>
                                            }
                                        } else {
                                            html! { <></> }
                                        }
                                    }
                                </td>
                                <td>{ &row.date }</td>
                            </tr>
                        } )
                    }
                </tbody>
            </table>
        </>
    }
}
