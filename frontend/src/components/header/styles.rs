use dominator::{stylesheet, class};
use once_cell::sync::Lazy;

use crate::config::THEME;

pub const NAV:Lazy<String> = Lazy::new(|| {
    class! {
        .style("background-color", THEME.nav_bg_color)
        .style("display", "flex")
        .style("align-items", "center")
        .style("justify-content", "space-between")
    }
});

pub const LOGO_HEIGHT:&'static str = "80rem";
pub const TOP_RIGHT:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("align-items", "center")
        .style("margin-right", "14rem")
    }
});