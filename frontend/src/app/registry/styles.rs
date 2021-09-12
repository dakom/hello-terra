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

pub const CHOICES:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "row")
        .style("align-items", "center")
        .style("margin-top", "10rem")
        .style("gap", "10rem")
    }
});