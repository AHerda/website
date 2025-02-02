use std::fmt::{self, Display, Formatter};

use maud::{html, Markup, Render};

// pub trait Htmx {
//     fn add_tag(self, tags: HtmxTag) -> Self;
// }

pub struct HtmxTag {
    pub command: HtmxRequest,
    pub trigger: String,
    pub swap: HtmxSwap,
    pub target: String,
    include: Option<String>,
}

impl HtmxTag {
    pub fn new(
        command: HtmxRequest,
        trigger: String,
        swap: HtmxSwap,
        target: String,
        include: Option<String>,
    ) -> Self {
        Self {
            command,
            trigger,
            swap,
            target,
            include,
        }
    }

    pub fn get(&self) -> (String, String, String, String) {
        (
            self.command.url(),
            self.trigger.clone(),
            self.swap.to_string(),
            self.target.clone(),
        )
    }
}

impl Display for HtmxTag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} hx-trigger=\"{}\" {} hx-target=\"{}\"{}",
            self.command,
            self.trigger,
            self.swap,
            self.target,
            match &self.include {
                Some(include) => format!(" hx-include=\"{}\"", include),
                None => "".to_string(),
            }
        )
    }
}

impl Render for HtmxTag {
    fn render(&self) -> Markup {
        html! {(self.to_string())}
    }
}

pub enum HtmxRequest {
    Get(String),
    Post(String),
    Put(String),
    Patch(String),
    Delete(String),
}

impl HtmxRequest {
    pub fn url(&self) -> String {
        match self {
            HtmxRequest::Get(url) => url,
            HtmxRequest::Post(url) => url,
            HtmxRequest::Put(url) => url,
            HtmxRequest::Patch(url) => url,
            HtmxRequest::Delete(url) => url,
        }
        .into()
    }
}

impl Display for HtmxRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HtmxRequest::Get(url) => write!(f, "hx-get=\"{}\"", url),
            HtmxRequest::Post(url) => write!(f, "hx-post=\"{}\"", url),
            HtmxRequest::Put(url) => write!(f, "hx-put=\"{}\"", url),
            HtmxRequest::Patch(url) => write!(f, "hx-patch=\"{}\"", url),
            HtmxRequest::Delete(url) => write!(f, "hx-delete=\"{}\"", url),
        }
    }
}

pub enum HtmxSwap {
    InnerHtml,
    OuterHtml,
    BeforeBegin,
    BeforeEnd,
    AfterBegin,
    AfterEnd,
}

impl Display for HtmxSwap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HtmxSwap::InnerHtml => "innerHTML",
                HtmxSwap::OuterHtml => "outerHTML",
                HtmxSwap::BeforeBegin => "beforebegin",
                HtmxSwap::BeforeEnd => "beforeend",
                HtmxSwap::AfterBegin => "afterbegin",
                HtmxSwap::AfterEnd => "afterend",
            }
        )
    }
}
