use maud::{html, Markup};

use crate::helpers::display_html::DisplayHtml;

pub struct Icon {
    name: String,
    font_class: String,
    title: Option<String>,
}

impl Icon {
    pub fn new(name: String, font_class: String, title: Option<String>) -> Self {
        Self {
            name,
            font_class,
            title,
        }
    }
}

impl DisplayHtml for Icon {
    fn display_html(&self) -> Markup {
        html! {
            @match &self.title {
                Some(title) => i .(format!("{} {}", self.font_class, self.name)) title=(title) {},
                None => i .(format!("{} {}", self.font_class, self.name)) {},
            }
        }
    }
}
