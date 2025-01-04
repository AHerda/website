use actix_web::{get, web};
use maud::{html, Markup};

use super::{base::base, help::Pages};

const PROJECTS: [Project; 3] = [
    Project {
        name: "Project 1",
        description: "This is a simple project.",
        link: None,
    },
    Project {
        name: "Project 2",
        description: "This is another simple project.",
        link: None,
    },
    Project {
        name: "Project 3",
        description: "This is a third simple project.",
        link: Some("https://example.com/project3"),
    },
];

#[get("/projects")]
pub async fn projects() -> Markup {
    base(html! {
        h1 { "Projects" }
        @let current_project = 0;
        .projects {
            @for (i, project) in PROJECTS.iter().enumerate() {
                .project id=(if i == current_project { "active" } else { "inactive" }) {
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
                button #left_button hx-get="/projects/{(current_project + PROJECTS.len() - 1) % PROJECTS.len()}" hx-target=".projects" hx-trigger="click" hx-swap="outerHTML" {
                    i class="fa-solid fa-arrow-left" title="previous project" {}
                }
                button #right_button hx-get="/projects/{(current_project + PROJECTS.len() + 1) % PROJECTS.len()}" hx-target=".projects" hx-trigger="click" hx-swap="outerHTML" {
                    i class="fa-solid fa-arrow-right" title="next project" {}
                }
            }
            p.project-page { (current_project + 1) " / " (PROJECTS.len()) }
            p.info { "Click the arrows to navigate between projects." }
        }
    }, Pages::Projects)
}

#[get("/projects/{index}")]
async fn print_projects(index: web::Path<usize>) -> Markup {
    let index = index.into_inner();
    println!("Index: {}", index);
    html! {
        .projects {
            @for (i, project) in PROJECTS.iter().enumerate() {
                .project id=(if i == index { "active" } else { "inactive" }) {
                    h2 { (project.name) }
                    p { (project.description) }
                    @if let Some(link) = project.link {
                        a href=(link) { "Link" }
                    }
                }
            }
        }
    }
}

struct Project<'a> {
    name: &'a str,
    description: &'a str,
    link: Option<&'a str>,
}
