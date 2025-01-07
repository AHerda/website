use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/about")]
pub async fn about() -> Markup {
    base(
        html! {
            @let title = "About me";
            h1 #hacked data-value=(title) { (title) }
            article {
                p {
                    "Hello, I'm Adrian Herda, a student of Alghoritmic Computer Science at the Wroclaw's University of Science and Technology.
                    I just finished my bachelor's degree and I'm currently working on my master's thesis."
                }
                p { "I'm passionate about programming, especially in algorithms and optimizing. I enjoy working with system programming languages like Rust and C++, as they allow me to build efficient and low-level solutions." }
                p {
                    "While I focus on system programming, I’m versatile and enjoy exploring various technologies and problem domains.
                    Whether it's designing algorithms, optimizing systems, or building tools, I’m driven by the challenge of creating robust and elegant solutions."
                }
            }
            p { "This is a simple website created by AHerda." }
        },
        Pages::About,
    )
}
