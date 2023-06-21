use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: AttrValue,
    pub id: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <>
            <label for={props.id.clone()}>{props.label.clone()}</label>
            <input
                id={props.id.clone()}
                type={props.input_type.clone()}
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </>
    }
}
