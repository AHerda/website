use maud::{html, Markup};

use crate::helpers::{display_html::DisplayHtml, flex_enum::FlexEnum, htmx};

pub struct Button {
    components: Vec<Box<dyn DisplayHtml>>,
    id: Option<String>,
    classes: Vec<String>,
    flex: FlexEnum,
    tag: Tag,
}

impl Button {
    pub fn new(
        components: Vec<Box<dyn DisplayHtml>>,
        id: Option<String>,
        classes: Vec<String>,
        flex: FlexEnum,
    ) -> Self {
        Self {
            components,
            id,
            classes,
            flex,
            tag: Tag::None,
        }
    }

    pub fn add_component(mut self, component: Box<dyn DisplayHtml>) -> Self {
        self.components.push(component);
        self
    }

    pub fn add_tag(mut self, tag: Tag) -> Self {
        self.tag = tag;
        self
    }
}

impl DisplayHtml for Button {
    fn display_html(&self) -> Markup {
        let classes = self.classes.join(" ")
            + if self.classes.is_empty() { "" } else { " " }
            + &self.flex.to_string();
        let basic_button = "basic_button".to_string();
        let id = self.id.as_ref().unwrap_or(&basic_button);
        let body = html! {
            @for component in &self.components {
                (component.display_html())
            }
        };
        html! {
            @match &self.tag {
                Tag::HtmxTag(tag) => {
                    @let tag = tag.get();
                    button .(classes) #(id)
                    hx-get=(tag.0) hx-trigger=(tag.1) hx-swap=(tag.2) hx-target=(tag.3) {
                        (body)
                    }
                }
                Tag::OnClick(js) => {
                    button .(classes) #(id)
                    onclick=(js) {
                        (body)
                    }
                }
                Tag::None => {
                    button .(classes) #(id) {
                        (body)
                    }
                }
            }
        }
    }
}

pub enum Tag {
    HtmxTag(htmx::HtmxTag),
    OnClick(String),
    None,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tag::HtmxTag(_), Tag::HtmxTag(_)) => true,
            (Tag::OnClick(_), Tag::OnClick(_)) => true,
            (Tag::None, Tag::None) => true,
            _ => false,
        }
    }
}
