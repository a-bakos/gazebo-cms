use yew::prelude::*;

#[function_component(EntryEditor)]
pub fn entry_editor() -> Html {
    // todo api call to get post data by ID
    html! {
        <>
            <p class="font-bold ">{"Title:"}</p>
            <input type={"text"} value={""} placeholder={"TITLE EDITOR"} class={"border-2 block w-full"} />
            <p class="font-bold ">{"Excerpt:"}</p>
            <textarea placeholder={"EXCERPT EDITOR"} class={"border-2 block w-full"}></textarea>
            <p class="font-bold ">{"Content:"}</p>
            <textarea placeholder={"POST CONTENT EDITOR"} class={"border-2 block w-full"}></textarea>
            <p class="font-bold ">{"Password:"}</p>
            <input type={"text"} value={""} placeholder={"PASSWORD"} class={"border-2 block w-full"} />
        </>
    }
}
