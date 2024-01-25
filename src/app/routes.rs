use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::page::Page;
use crate::pages::home::Home;

// Define the Route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    RouteHome,
    #[at("/home")]
    Home,
    #[at("/page")]
    Page,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::RouteHome => html! {
            <Redirect<Route> to={Route::Home}/>
        },
        Route::Home => html! {
            <Home />
        },
        Route::Page => html! {
            <Page />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}