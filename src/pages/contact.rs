use actix_web::get;
use maud::{html, Markup};

use super::base::base;
use crate::helpers::pages_enum::Pages;

#[get("/contact")]
async fn contact() -> Markup {
    base(
        html! {
            @let title = "How to contact me";
            h1 #hacked data-value=(title) { (title) }
            ."contact row" {
                @for contact_button in vec![
                    ContactButton::new("mail", "solid", Some("envelope"), ContactButtonType::Mail),
                ] {
                    (contact_button.display())
                }
            }
            h1 { "Also check out" }
            ."contact row" {
                @for contact_button in vec![
                    ContactButton::new("facebook", "brands", None, ContactButtonType::SocialMedia("https://facebook.com/adrian.herda.1".to_string())),
                    ContactButton::new("instagram", "brands", None, ContactButtonType::SocialMedia("https://instagram.com/adrian.herda".to_string())),
                    // ContactButton::new("snapchat", "brands", None, ContactButtonType::None),
                    ContactButton::new("github", "brands", None, ContactButtonType::SocialMedia("https://github.com/AHerda".to_string())),
                    ContactButton::new("cv", "solid", Some("file"), ContactButtonType::File("/cv".to_string())),
                ] {
                    (contact_button.display())
                }
            }
        },
        Pages::Contact,
    )
}

enum ContactButtonType {
    Mail,
    SocialMedia(String),
    File(String),
    None
}

struct ContactButton {
    name: String,
    icon_font_class: String,
    icon_name: Option<String>,
    cb_type: ContactButtonType,
}

impl ContactButton {
    fn new(
        name: &str,
        icon_font_class: &str,
        icon_name: Option<&str>,
        cb_type: ContactButtonType,
    ) -> Self {
        Self {
            name: name.to_string(),
            icon_font_class: icon_font_class.to_string(),
            icon_name: match icon_name {
                Some(name) => Some(name.to_string()),
                None => None,
            },
            cb_type,
        }
    }

    fn display(&self) -> Markup {
        html! {
            ."contact_button column" {
                @match &self.cb_type {
                    ContactButtonType::Mail => {
                        a href="mailto:adrianherda@gmail.com" {
                            button #(self.name) {
                                i .(format!("fa-{} fa-{}", self.icon_font_class, self.icon_name.as_ref().unwrap_or(&self.name))) {}
                            }
                        }
                    }
                    ContactButtonType::File(link) => {
                        a href=(format!("{}", link)) {
                            button #(self.name) {
                                i .(format!("fa-{} fa-{}", self.icon_font_class, self.icon_name.as_ref().unwrap_or(&self.name))) {}
                            }
                        }
                    }
                    ContactButtonType::SocialMedia(link) => {
                        a href=(format!("{}", link)) download {
                            button #(self.name) {
                                i .(format!("fa-{} fa-{}", self.icon_font_class, self.icon_name.as_ref().unwrap_or(&self.name))) {}
                            }
                        }
                    },
                    ContactButtonType::None => {
                        button #(self.name) {
                            i .(format!("fa-{} fa-{}", self.icon_font_class, self.icon_name.as_ref().unwrap_or(&self.name))) {}
                        }
                    }
                }
                label for=(self.name) { (Self::uppercase_first(&self.name)) }
            }
        }
    }

    fn uppercase_first(s: &str) -> String {
        s[..1].to_uppercase() + &s[1..]
    }
}
