use dominator::{html, Dom};
use super::styles;
use super::state::*;

impl Overlay {
    pub fn render(&self, child: Dom) -> Dom {
        html!("div", {
            .class(&*styles::OVERLAY_BG)
            .child(child)
        })
    }

    pub fn render_loader(&self) -> Dom {
        self.render(
            html!("div", {
                .class(&*styles::LOADER_CONTENT)
                .child(html!("h1", {
                    .class(&*styles::LOADER_TEXT)
                    .text("Loading...")
                }))
            })
        )
    }
}
