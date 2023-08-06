use yew::prelude::*;

#[function_component(ProfileEditor)]
pub fn profile_editor() -> Html {
    html! {
        <form>
            <p>{"name"}</p>
            <button>{"Update profile"}</button>
        </form>
    }
}
