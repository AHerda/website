use actix_web::get;
use maud::{html, Markup};

use super::{base::base, help::Pages};

#[get("/contact")]
async fn contact() -> Markup {
    base(
        html! {
            @let title = "How to contact me";
            h1 #hacked data-value=(title) { (title) }
            .contact {
                @for contact_button in vec![
                    ContactButton::new("mail", "solid", Some("envelope")),
                ] {
                    (contact_button.display())
                }
            }
            h1 { "Also check out" }
            .contact {
                @for contact_button in vec![
                    ContactButton::new("facebook", "brands", None),
                    ContactButton::new("instagram", "brands", None),
                    ContactButton::new("snapchat", "brands", None),
                    ContactButton::new("github", "brands", None),
                    ContactButton::new("cv", "solid", Some("file")),
                ] {
                    (contact_button.display())
                }
            }
        },
        Pages::Contact,
    )
}

struct ContactButton {
    name: String,
    icon_font_class: String,
    icon_name: Option<String>,
}

impl ContactButton {
    fn new(name: &str, icon_font_class: &str, icon_name: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            icon_font_class: icon_font_class.to_string(),
            icon_name: match icon_name {
                Some(name) => Some(name.to_string()),
                None => None,
            },
        }
    }

    fn display(&self) -> Markup {
        html! {
            .contact_button {
                button #(self.name) {

                    i .(format!("fa-{} fa-{}", self.icon_font_class, self.icon_name.clone().unwrap_or(self.name.clone()))) {}
                }
                label for=(self.name) { (Self::uppercase_first(&self.name)) }
            }
        }
    }

    fn uppercase_first(s: &str) -> String {
        s[..1].to_uppercase() + &s[1..]
    }
}
