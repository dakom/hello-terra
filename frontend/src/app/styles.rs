use dominator::{stylesheet, class};
use once_cell::sync::Lazy;
use crate::config::THEME;

pub const MAIN:Lazy<String> = Lazy::new(|| {
    class! {
    }
});

pub const IFRAME_HIDDEN:Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("width", "0")
        .style("height", "0")
        .style("border", "0")
        .style("top", "-100px")
        .style("left", "-100px")

    }
});

pub const IFRAME_VISIBLE:Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "fixed")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100vw")
        .style("height", "100vh")
        .style("border", "0")

    }
});
