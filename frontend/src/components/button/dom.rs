use dominator::{html, clone, Dom, events, DomBuilder};
use std::rc::Rc;
use super::{state::*, styles};
use once_cell::sync::Lazy;
use web_sys::{HtmlElement, EventTarget, HtmlImageElement};
use crate::utils::prelude::*;

impl Button {
    pub fn render(&self) -> Dom {
        Self::_render_mixin(self, None::<MixinStub<HtmlElement>>)
    }

    pub fn render_mixin<F>(&self, mixin: F) -> Dom 
    where F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
    {
        Self::_render_mixin(self, Some(mixin))
    }

    fn _render_mixin<F>(&self, mixin: Option<F>) -> Dom 
    where F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
    {
        match &self.style {
            ButtonStyle::Color(color, text) => {
                html!("div", {
                    .class(&*styles::CURSOR)
                    .class(&*styles::COLOR)
                    .class(color.get_class())
                    .child(html!("span", {
                        .class(&*styles::COLOR_TEXT)
                        .text(&text)
                    }))
                    .apply_if(mixin.is_some(), |dom| dom.apply(mixin.unwrap_ext()))
                })
            },
            ButtonStyle::Image(image) => {
                image.render_mixin(|dom| {
                    dom
                        .class(&*styles::CURSOR)
                        .apply_if(mixin.is_some(), |dom| dom.apply(mixin.unwrap_ext()))
                })
            }
        }
    }
}

