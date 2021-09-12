use dominator::DomBuilder;
use web_sys::HtmlInputElement;
use super::styles;
use crate::utils::prelude::*;

pub struct InputMixin {
}

impl InputMixin {
    pub fn text<'a>(placeholder:Option<&'a str>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + 'a {
        move |dom| {
            dom
                .class(&*styles::TEXT)
                .class(&*styles::INPUT)
                .attr("type", "text")
                .apply_if(placeholder.is_some(), |dom| {
                    dom.property("placeholder", placeholder.unwrap_ext())
                })

        }
    }
    pub fn password<'a>(placeholder:Option<&'a str>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + 'a {
        move |dom| {
            dom
                .class(&*styles::TEXT)
                .class(&*styles::INPUT)
                .attr("type", "password")
                .apply_if(placeholder.is_some(), |dom| {
                    dom.property("placeholder", placeholder.unwrap_ext())
                })

        }
    }
}
