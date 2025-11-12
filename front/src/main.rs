mod app;
use app::App;

mod terminal;
// use terminal::Terminal;
fn main() {
    yew::Renderer::<App>::new().render();
}