use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: AttrValue,
    pub id: AttrValue,
    pub class_list: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            type="button"
            class={props.class_list.clone()}
            id={props.id.clone()}
            value={props.value.clone()}
            onchange={props.onchange.clone()}>
            {props.label.clone()}
        </button>
    }
}
