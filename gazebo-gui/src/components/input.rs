use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: Option<AttrValue>,
    pub id: Option<AttrValue>,
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
                class={"block border-2 rounded-lg px-3 py-1"}
                id={props.id.clone()}
                type={props.input_type.clone()}
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </>
    }
}
