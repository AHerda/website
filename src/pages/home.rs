use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/")]
pub async fn home() -> Markup {
    base(
        html! {
            @let title = "Hello, World!";
            img src="static/img/me.png" alt="Photo of me";
            h1 #hacked data-value=(title) { (title) }
            article {
                p { "Welcome to my website." }
                p { "I'm Adrian Herda, a student of Alghoritmic Computer Science at the Wroclaw's University of Science and Technology." }
                p { "Feel free to check out my projects or contact me." }
            }
        },
        Pages::Home,
    )
}
