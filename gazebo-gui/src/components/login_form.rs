use yew::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <form>
            <label>{"Username or Email"}</label>
            <input type="text" />
            <label>{"Password"}</label>
            <input type="text" />
            <button type="submit">{"Login"}</button>
        </form>
    }
}
