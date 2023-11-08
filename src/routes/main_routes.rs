use yew::{html, Html};
use yew_router::{prelude::Redirect, Routable};

use crate::components::pages::main::{
    customers::Customers, games::Games, integrations::Integrations, overview::Overview,
    participants::Participants, settings::Settings, teams::Teams, tickets::Tickets,
};

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
        MainRoute::Overview => html! {<Overview />},
        MainRoute::Integrations => html! { <Integrations />},
        MainRoute::Participants => html! { <Participants/> },
        MainRoute::Teams => html! { <Teams /> },
        MainRoute::Customers => html! { <Customers/>},
        MainRoute::Games => html! { <Games />},
        MainRoute::Tickets => html! {<Tickets  /> },
        MainRoute::Settings => html! { <Settings />},
        MainRoute::NotFound { path: _ } => {
            html! {<Redirect<MainRoute> to={MainRoute::Overview} /> }
        }
    }
}
