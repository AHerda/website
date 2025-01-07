use maud::{html, Markup, DOCTYPE};

use super::help::Pages;

pub fn base(body: Markup, page: Pages) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "AHerda's Simple Website" }
                link rel="stylesheet" href="static/css/default.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css";
                script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {};
                script src="static/js/projects_change.js" {};
                script src="static/js/hacked_letters.js" {};
                script src="static/js/rainbow.js" {};
                script src="static/js/matrix.js" {};
            } // head
            body {
                div .background {
                    script src="static/js/background.js" {};
                    canvas .background {}
                }
                header {
                    h1 { @match page {
                        Pages::Home => "Home",
                        Pages::About => "About",
                        Pages::Projects => "Projects",
                        Pages::Visuals => "Visuals",
                        Pages::Contact => "Contact",
                    }}
                    nav {
                        a href="/" class=@if page == Pages::Home { "active" } @else { "inactive" } { "Home" }
                        a href="/about" class=@if page == Pages::About { "active" } @else { "inactive" } { "About" }
                        a href="/projects" class=@if page == Pages::Projects { "active" } @else { "inactive" } { "Projects" }
                        a href="/visuals" class=@if page == Pages::Visuals { "active" } @else { "inactive" } { "Visuals" }
                        a href="/contact" class=@if page == Pages::Contact { "active" } @else { "inactive" } { "Contact" }
                    }
                }
                main {
                    (body)
                }
                footer {
                    p { "Website created by Adrian Herda." }
                    a href="https://github.com/AHerda/website" { "Source Code" }
                }
            } // body
        } // html
    }
}
