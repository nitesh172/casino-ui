use yew::{html, Html};
use yew_router::{prelude::Redirect, Routable};

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Overview,
    #[at("/integrations")]
    Integrations,
    #[at("/participants")]
    Participants,
    #[at("/teams")]
    Teams,
    #[at("/customers")]
    Customers,
    #[at("/games")]
    Games,
    #[at("/tickets")]
    Tickets,
    #[at("/settings")]
    Settings,
    #[at("/*path")]
    NotFound { path: String },
}

pub fn switch_main(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Overview => html! {<h1>{"Overview"}</h1>},
        MainRoute::Integrations => html! {<h1>{"Integrations"}</h1>},
        MainRoute::Participants => html! {<h1>{"Participants"}</h1>},
        MainRoute::Teams => todo!(),
        MainRoute::Customers => todo!(),
        MainRoute::Games => todo!(),
        MainRoute::Tickets => todo!(),
        MainRoute::Settings => todo!(),
        MainRoute::NotFound { path: _ } => {
            html! {<Redirect<MainRoute> to={MainRoute::Overview} /> }
        }
    }
}
