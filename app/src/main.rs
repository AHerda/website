use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::tata_rozpiska::DataListWrapper;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/skills")]
    Skills,
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/")]
    Home,
    #[at("/tata_rozpiska")]
    TataRozpiska,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    // yew::start_app::<DataListWrapper>();
    yew::Renderer::<DataListWrapper>::new().render();
}
