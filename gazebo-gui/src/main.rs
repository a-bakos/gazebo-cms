mod api;
mod app;
mod components;

use app::App;

fn main() {
    // To create console logs, you can now use the log::info!(“”); macro
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
