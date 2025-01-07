use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/contact")]
async fn contact() -> Markup {
    base(html! {
        @let title = "How to contact me";
        h1 #hacked data-value=(title) { (title) }
        .contact {
            button #mail {
                i ."fa-solid fa-envelope" {}
            }
            label for="mail" { "Mail" }
            button #github {
                i ."fa-brands fa-github" {}
            }
            label for="github" { "GitHub" }
            button #cv {
                i ."fa-solid fa-file" {}
            }
            label for="cv" { "CV" }
        }
        h2 { "Also check out" }
        .contact {
            @for name in vec!["facebook", "instagram", "snapchat"] {
                .contact_button {
                    button #(name) {
                        i class=(format!("fa-brands fa-{}", name)) {}
                    }
                    label for=(name) { (uppercase_first(name)) }
                }
            }
        }
    }, Pages::Contact)
}

fn uppercase_first(s: &str) -> String {
    s[..1].to_uppercase() + &s[1..]
}
