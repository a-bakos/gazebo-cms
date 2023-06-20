use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
        <h1>{ "Hello, Gazebo CMS!" }</h1>
        <MainNavigation />
        </>
    }
}

#[function_component(MainNavigation)]
pub fn main_navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li>{ "Menu 1" }</li>
                <li>{ "Menu 2" }</li>
                <li>{ "Menu 3" }</li>
            </ul>
        </nav>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Header />
        <main>
            <img class="logo" src="assets/gazebo-logo.jpg" alt="Gazebo logo" />
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
        </>
    }
}
