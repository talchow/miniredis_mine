mod app;
use app::App;

mod terminal;
mod types;
fn main() {
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}