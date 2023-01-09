use pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/repos")]
    Repos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Repos => html! {<p>{"You are looking at Post"}</p>},
        Route::NotFound => html!(<p>{"404! Page not found"}</p>),
    }
}

#[function_component]
fn App() -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    )
}
fn main() {
    yew::Renderer::<App>::new().render();
}
