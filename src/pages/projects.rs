use actix_web::{get, web::Json, Responder};
use maud::{html, Markup};
use serde::{Deserialize, Serialize};

use super::{base::base, help::Pages};

const PROJECTS: [Project; 4] = [
    Project {
        name: "Checkers",
        description: "Simple project in java to learn the basics of programming.",
        link: Some("https://github.com/AHerda/warcaby"),
    },
    Project {
        name: "Radio Traffic Analyzer",
        description: "Engineering thesis project. Developed in C++.",
        link: Some("https://github.com/AHerda/sdr-analyzer"),
    },
    Project {
        name: "Website",
        description: "Website written in rust using actix-web and maud.",
        link: Some("https://github.com/AHerda/website"),
    },
    Project {
        name: "Compiler",
        description: "Simple compiler written in C++.",
        link: Some("https://github.com/AHerda/Kompilator"),
    },
];

#[get("/projects")]
pub async fn projects() -> Markup {
    base(html! {
        .projects {
            @for (i, project) in PROJECTS.iter().enumerate() {
                .(format!("project {}", if i == 0 { "active" } else { "inactive" })) {
                    h2 { (project.name) }
                    p { (project.description) }
                    @if let Some(link) = project.link {
                        a href=(link) { "Link" }
                    }
                }
            }
        }
        .navigation {
            nav {
                button #left_button onclick="handleLeftClick()" {
                    i class="fa-solid fa-arrow-left" title="previous project" {}
                }
                button #right_button onclick="handleRightClick()" {
                    i class="fa-solid fa-arrow-right" title="next project" {}
                }
            }
            p .project-page { (1) " / " (PROJECTS.len()) }
            p #info { "Click the arrows to navigate between projects." }
        }
    }, Pages::Projects)
}

#[get("/api/projects-json")]
async fn projects_json() -> impl Responder {
    Json(PROJECTS)
}

#[derive(Deserialize, Serialize)]
struct Project<'a> {
    name: &'a str,
    description: &'a str,
    link: Option<&'a str>,
}
