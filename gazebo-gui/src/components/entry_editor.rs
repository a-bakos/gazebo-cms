use gazebo_core_common::entry::gb_post::GB_Post;
use yew::prelude::*;

// struct EditorProps {
//     title: String,
//     title_changed: Callback<>,
//     permalink: String,
//     permalink_changed:Callback<>,
//     excerpt: String,
//     excerpt_changed:Callback<>,
//     password: String,
//     password_changed:Callback<>,
// }

#[derive(Properties, PartialEq)]
pub struct EntryEditorProps {
    pub title: Option<AttrValue>,
    // pub id: AttrValue,
    // pub id_author: AttrValue,
    // pub id_parent: Option<AttrValue>,
    // pub date_publish: AttrValue,
    pub date_modified: Option<AttrValue>,
    pub slug: Option<AttrValue>,
    // pub status: AttrValue,
    pub excerpt: Option<AttrValue>,
    pub content: Option<AttrValue>,
    pub password: Option<AttrValue>,
}

#[function_component(EntryEditor)]
pub fn entry_editor(props: &EntryEditorProps) -> Html {
    let title = props.title.clone();
    let content = props.content.clone();

    html! {
        <>
            <p class="font-bold ">{"Title:"}</p>
            <input type={"text"} value={title} placeholder={"TITLE EDITOR"} class={"border-2 block w-full"} />

            <p class="font-bold">
                {"Permalink: "}
                <input type={"text"} value={props.slug.clone()} placeholder={"permalink"} class={"border-2 w-full block"} />
            </p>

            <p class="font-bold ">{"Excerpt:"}</p>
            <textarea placeholder={"EXCERPT EDITOR"} value={ props.excerpt.clone() } class={"border-2 block w-full"}></textarea>
            <p class="font-bold ">{"Content:"}</p>
            <textarea placeholder={"POST CONTENT EDITOR"} value={ props.content.clone() } class={"border-2 block w-full"}></textarea>
            <p class="font-bold ">{"Password:"}</p>
            <input type={"text"} value={props.password.clone()} placeholder={"PASSWORD"} class={"border-2 block w-full"} />
        </>
    }
}
