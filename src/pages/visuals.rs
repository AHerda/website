use actix_web::{get, web};
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/visuals")]
async fn visuals() -> Markup {
    base(html! {
        @let title = "Cool visual effects";
        h1 #hacked data-value=(title) { (title) }
        div .div_visual {
            canvas .visual {}
            script .visual src="static/js/rainbow.js" {};
        }
        .navigation {
            nav {
                button #matrix_button hx-get="/visuals/matrix" hx-target="div.div_visual" hx-swap="innerHTML" {
                    i class="fa-solid fa-arrow-left" title="Matrix Visual" {}
                }
                button #rainbow_button hx-get="/visuals/rainbow" hx-target="div.div_visual" hx-swap="innerHTML" {
                    i class="fa-solid fa-arrow-right" title="Synchronic Sounds" {}
                }
            }
            // p .project-page { (1) " / " (PROJECTS.len()) }
            p #info { "Click the arrows to navigate between projects." }
        }
    }, Pages::Visuals)
}

#[get("/visuals/{visual}")]
async fn visual(visual: web::Path<String>) -> Markup {
    html!(
        cavnas .visual {}
        script .visual src=(format!("static/js/{}.js", visual)) {};
    )
}
