use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;
use crate::utils::{prelude::*, path::media_url};
use super::state::*;

impl Image {
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
        html!("img", {
            .attribute("src", &format!("{}/images/{}", media_url(), self.kind.as_str()))
            .apply_if(mixin.is_some(), |dom| dom.apply(mixin.unwrap_ext()))
        })
    }
}