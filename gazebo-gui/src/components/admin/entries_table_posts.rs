use crate::{
    api::post::{update_entry_single_param, EntryUpdateProps, EntryUpdateType},
    app::MainNavigationRoute,
    components::button::Button,
};

use gazebo_core_common::entry::{
    entry_type::EntryType,
    gb_entry::{GB_EntryCommon, GB_EntryDateVariant},
    gb_post::GB_Post,
    status::{ContentStatus, EntryStatus},
};

use yew::{html::IntoPropValue, platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostTableRowProps {
    pub row_data: GB_Post,
}

#[function_component(PostTableRow)]
pub fn table_entry_row(props: &PostTableRowProps) -> Html {
    let (status_label, status_label_class) = match props.row_data.get_status() {
        // todo - will be added to common lib
        EntryStatus::Post(content_status) => match content_status {
            ContentStatus::Draft => ("draft".to_string(), "bg-[#eda52d]"),
            ContentStatus::Publish => ("publish".to_string(), "bg-[#607a4b]"),
            ContentStatus::Private => ("private".to_string(), "bg-[#cf743d]"),
            ContentStatus::Trash => ("trash".to_string(), "bg-[#b72224]"),
            _ => ("unknown".to_string(), "bg-white-100"),
        },
        _ => ("unknown".to_string(), "bg-white-100"),
    };

    let post_id = props.row_data.get_id();
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
        <tr class="rounded-xl border border-[#f4e4d4] hover:bg-[#f4e4d4]">
            <td class="p-2">
                <Link<MainNavigationRoute>
                    to={ MainNavigationRoute::EntryEditExisting { entry_type: EntryType::Post.to_string(), id: post_id.to_string() } }
                    classes={ "font-bold text-[#47837b]" }>
                    { props.row_data.get_title() }
                </Link<MainNavigationRoute>>
                <span class="block">
                    <Link<MainNavigationRoute>
                        to={ MainNavigationRoute::EntryView { id: post_id.to_string() } }
                        classes={ "underline select-none mr-1" }>
                        { "View" }
                    </Link<MainNavigationRoute>>

                    <Link<MainNavigationRoute>
                        to={ MainNavigationRoute::EntryEditExisting { entry_type: EntryType::Post.to_string(), id: post_id.to_string() } }
                        classes={ "underline select-none mr-1" }>
                        { "Edit" }
                    </Link<MainNavigationRoute>>

                    <button class="underline select-none mr-1">{ "?clone" }</button>

                    <form

                        class={"select-none inline"}
                        onsubmit={on_form_submit_bin}>

                        <Button
                            button_type="submit"
                            label="Bin it!"
                        />
                    </form>

                </span>
            </td>
            <td>{ "cat 1, cat 2" }</td>
            <td>{ props.row_data.get_author_id() }</td>
            <td>
                <span class={ format!("{} px-2 rounded-md text-white uppercase text-xs font-bold tracking-wide select-none ", status_label_class) }>
                    { status_label.clone() }
                </span>
                {
                    if status_label == "draft" {
                        html! {
                            <form
                                class={"block"}
                                onsubmit={ on_form_submit_publish }>
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
                <p>{ props.row_data.get_date( GB_EntryDateVariant::Publish ) }</p>
                <p>{ "?by admin" }</p>
            </td>
            <td>
                <p>{ props.row_data.get_date( GB_EntryDateVariant::Modified ) }</p>
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
            <table class={"text-left w-full border border-[#f4e4d4]"}>
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
                            <PostTableRow row_data={ entry_row.clone() } />
                        } )
                    }
                </tbody>
            </table>
        </>
    }
}
