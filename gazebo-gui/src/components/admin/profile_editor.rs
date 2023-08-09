use yew::prelude::*;

#[function_component(ProfileEditor)]
pub fn profile_editor() -> Html {
    html! {
        <form>
            <table>
                <tr>
                    <td>{"Name"}</td>
                    <td>
                        <input
                            class={"block"}
                            type={"text"}
                            value={"name value"}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Email"}</td>
                    <td>{"Email input"}</td>
                </tr>
                <tr>
                    <td>{"Role"}</td>
                    <td>{"Role input"}</td>
                </tr>
            </table>
            <button class={"bg-red-200 hover:bg-red-500 text-2xl"}>{"Update profile"}</button>
        </form>
    }
}
