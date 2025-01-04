use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/")]
pub async fn home() -> Markup {
    base(html! {
        h1 { "Hello, world!" }
    }, Pages::Home)
}
