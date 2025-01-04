use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/about")]
pub async fn about() -> Markup {
    base(html! {
        h1 { "About" }
        p { "This is a simple website created by AHerda." }
    }, Pages::About)
}
