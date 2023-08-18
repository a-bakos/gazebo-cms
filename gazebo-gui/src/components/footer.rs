use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={"w-full border-t text-center"}>
            <small>{"Gazebo CMS | Work In Progress | 2023 | Attila Bakos"}</small>
        </footer>
    }
}
