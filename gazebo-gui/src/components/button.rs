use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: AttrValue,
    // pub class_list: AttrValue,
    pub button_type: AttrValue,
    pub value: Option<AttrValue>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            // class={props.class_list.clone()}
            type={props.button_type.clone()}
            value={props.value.clone()}>
            {props.label.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormWithButtonProps {
    pub on_submit: Callback<SubmitEvent>,
    pub button_props: ButtonProps,
}

#[function_component(FormWithButton)]
pub fn form_with_button(props: &FormWithButtonProps) -> Html {
    let on_form_submit = props.on_submit.clone();
    html! {
        <form
            class={"inline"}
            onsubmit={on_form_submit}>
            <Button
                button_type={props.button_props.button_type.clone()}
                label={props.button_props.label.clone()}
            />
        </form>
    }
}

// Usage
// <FormWithButton
//     on_submit={on_form_submit_bin}
//     button_props={ButtonProps {
//         button_type: String::from("submit").into_prop_value(),
//         label: String::from("Bin it!").into_prop_value(),
//         value: None,
//     }}
// />
