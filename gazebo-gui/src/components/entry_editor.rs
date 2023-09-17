use crate::components::input::Input;
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
