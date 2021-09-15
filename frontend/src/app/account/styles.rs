use dominator::{stylesheet, class};
use once_cell::sync::Lazy;

use crate::config::THEME;

pub const PAGE:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("justify-content", "center")
        .style("align-items", "center")
    }
});

pub const META_INFO:Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "fixed")
        .style("bottom", "0")
        .style("left", "0")
    }
});