use maud::{html, Markup};
use serde::{Deserialize, Serialize};

use crate::helpers::{display_html::DisplayHtml, flex_enum::FlexEnum};

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Project<'a> {
    pub name: &'a str,
    pub class: &'a str,
    pub description: &'a str,
    pub link: Option<&'a str>,
}

impl Project<'_> {
    pub const fn new<'a>(
        name: &'a str,
        description: &'a str,
        link: Option<&'a str>,
    ) -> Project<'a> {
        Project {
            name,
            class: "",
            description,
            link,
        }
    }

    pub fn set_class<'a>(mut self, class: &'a str) -> Self
    where
        'a: 'static,
    {
        self.class = class;
        self
    }
}

impl DisplayHtml for Project<'_> {
    fn display_html(&self) -> Markup {
        let classes = format!("project {} {}", self.class, FlexEnum::Column);
        html! {
            .(classes) {
                h2 { (self.name) }
                p { (self.description) }
                @if let Some(link) = self.link {
                    a href=(link) { "Link" }
                }
            }
        }
    }
}
