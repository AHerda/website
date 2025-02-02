use actix_web::{
    get,
    web::{self, Json, JsonBody},
    HttpResponse, Responder,
};
use maud::{html, Markup};

use super::base::base;
use crate::{
    components::{button::{Button, Tag}, icon::Icon},
    data::project_list::PROJECTS,
    helpers::{
        display_html::DisplayHtml,
        flex_enum::FlexEnum,
        pages_enum::Pages,
    },
};

#[get("/projects")]
pub async fn projects() -> Markup {
    let projects = {
        PROJECTS.read().expect("Failed to retreive projects").clone()
    };
    base(
        html! {
            .projects {
                @for (i, proj) in projects.iter().enumerate() {
                    (
                        proj
                            .set_class(if i == 0 { "active" } else { "inactive" })
                            .display_html()
                    )
                }
            }
            .navigation {
                nav {
                    (
                        Button::new(
                            vec![
                                Box::new(Icon::new(
                                    "fa-arrow-left".to_string(),
                                    "fa-solid".to_string(),
                                    Some("previous project".to_string())
                                ))
                            ],
                            Some("left_button".to_string()),
                            Vec::new(),
                            FlexEnum::Row,
                        )
                            .add_tag(
                                Tag::OnClick("handleLeftClick()".to_string())
                            )
                            .display_html()
                    )
                    (
                        Button::new(
                            vec![
                                Box::new(Icon::new(
                                    "fa-arrow-right".to_string(),
                                    "fa-solid".to_string(),
                                    Some("next project".to_string())
                                ))
                            ],
                            Some("right_button".to_string()),
                            Vec::new(),
                            FlexEnum::Row,
                        )
                            .add_tag(
                                Tag::OnClick("handleRightClick()".to_string())
                            )
                            .display_html()
                    )
                }
                p #page_no { "1 / 4" }
                p #info { "Click the arrows to navigate between projects." }
            }
        },
        Pages::Projects,
    )
}

#[get("/api/projects/json")]
async fn projects_json() -> impl Responder {
    let projects_json = PROJECTS.read().expect("Failed to retreive projects").clone();
    // FIXME: Change this to return JSON
    HttpResponse::Ok().content_type("application/json").body(format!("{:#?}", projects_json))
}

// #[get("/api/projects/project")]
// async fn project(project_name: web::Json<String>) -> HttpResponse {
//     println!("{:?}", project_name);

//     HttpResponse::Ok().finish()
// }
