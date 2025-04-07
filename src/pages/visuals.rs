use actix_web::{get, web};
use maud::{html, Markup};

use super::base::base;
use crate::{
    components::{
        button::{Button, Tag},
        icon::Icon,
    },
    helpers::{display_html::DisplayHtml, flex_enum::FlexEnum, htmx, pages_enum::Pages},
};

#[get("/visuals")]
async fn visuals() -> Markup {
    base(
        html! {
            @let title = "Cool visual effects";
            @let starting_visual = "rainbow";
            h1 #hacked data-value=(title) { (title) }
            div .div_visual {
                canvas .visual {}
                script .visual { (starting_visual) "();" };
            }
            .navigation {
                nav {
                    (
                        Button::new(
                            vec![Box::new(Icon::new(
                                "fa-rainbow".to_string(),
                                "fa-solid".to_string(),
                                Some("Synchronic Sounds".to_string())
                            ))],
                            Some("rainbow_button".to_string()),
                            vec![],
                            FlexEnum::Column,
                        )
                            .add_tag(Tag::HtmxTag(htmx::HtmxTag::new(
                                htmx::HtmxRequest::Get("/visuals/rainbow".to_string()),
                                "click".to_string(),
                                htmx::HtmxSwap::OuterHtml,
                                "script.visual".to_string(),
                                None
                            )))
                            .display_html()
                    )
                    (
                        Button::new(
                            vec![Box::new(Icon::new(
                                "fa-cloud-rain".to_string(),
                                "fa-solid".to_string(),
                                Some("Matrix Visual".to_string())
                            ))],
                            Some("matrix_button".to_string()),
                            vec![],
                            FlexEnum::Column,
                        )
                            .add_tag(Tag::HtmxTag(htmx::HtmxTag::new(
                                htmx::HtmxRequest::Get("/visuals/matrix".to_string()),
                                "click".to_string(),
                                htmx::HtmxSwap::OuterHtml,
                                "script.visual".to_string(),
                                None
                            )))
                            .display_html()
                    )
                }
            }
        },
        Pages::Visuals,
    )
}

#[get("/visuals/{visual}")]
async fn visual(visual: web::Path<String>) -> Markup {
    html!(
        script .visual { (visual) "();" };
    )
}
