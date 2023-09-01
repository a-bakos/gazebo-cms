use crate::components::input::Input;
use gazebo_core_common::entry::gb_post::GB_Post;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EntryEditorProps {
    pub title: AttrValue,
    pub title_changed: Callback<Event>,
    pub permalink: AttrValue,
    pub permalink_changed: Callback<Event>,
    pub excerpt: Option<AttrValue>,
    pub excerpt_changed: Option<Callback<Event>>,
    pub content: Option<AttrValue>,
    pub content_changed: Option<Callback<Event>>,
    pub password: Option<AttrValue>,
    pub password_changed: Option<Callback<Event>>,
}
/*
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
*/
#[function_component(EntryEditor)]
pub fn entry_editor(props: &EntryEditorProps) -> Html {
    html! {
        <>
            <Input
                label={"Title"}
                id={""}
                name={""}
                input_type={"text"}
                value={ props.title.clone() }
                onchange={ props.title_changed.clone() }
            />

            <Input
                label={"Permalink"}
                id={""}
                name={""}
                input_type={"text"}
                value={ props.permalink.clone() }
                onchange={ props.permalink_changed.clone() }
            />

            <Input
                label={"Excerpt"}
                id={""}
                name={""}
                input_type={"text"}
                value={ props.excerpt.clone().unwrap().clone() }
                onchange={ props.excerpt_changed.clone().unwrap().clone() }
            />

            <Input
                label={"Content"}
                id={""}
                name={""}
                input_type={"text"}
                value={ props.content.clone().unwrap().clone() }
                onchange={ props.content_changed.clone().unwrap().clone() }
            />

            <Input
                label={"Password"}
                id={""}
                name={""}
                input_type={"text"}
                value={ props.password.clone().unwrap().clone() }
                onchange={ props.password_changed.clone().unwrap().clone() }
            />
        </>
    }
}
