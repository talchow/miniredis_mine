use yew::prelude::*;
use yew_router::prelude::*;
use crate::terminal::Terminal;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Terminal,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Terminal => html! { <Terminal /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}