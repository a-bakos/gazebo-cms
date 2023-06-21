use yew::prelude::*;

use crate::components::main_navigation::MainNavigation;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
        <h1>{ "Hello, Gazebo CMS!" }</h1>
        <MainNavigation />
        </>
    }
}
