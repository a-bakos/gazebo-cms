// TODO this doesn't work

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    pub value: AttrValue,
    pub onchange: Callback<InputData>,
}

#[function_component(TextArea)]
pub fn textarea(props: &TextAreaProps) -> Html {
    html! {
        <textarea
            placeholder={"CONTENT"}
            oninput={ props.onchange.clone() }
            value={ props.value.clone() }
            class={"border-2 block w-full"}></textarea>
    }
}
