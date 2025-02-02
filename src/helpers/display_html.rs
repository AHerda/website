use maud::{html, Markup};

pub trait DisplayHtml {
    fn display_html(&self) -> Markup {
        html! {
            p { "This is a default implementation." }
        }
    }
}
