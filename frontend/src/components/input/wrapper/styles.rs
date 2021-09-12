use once_cell::sync::Lazy;
use dominator::{class, pseudo};
use crate::config::THEME;

pub static WRAPPER:Lazy<String> = Lazy::new(|| {
    class!{
        .style("outline", "none")
        .style("padding", "3px")
        .style("display", "inline-block")
        .style("border", &format!("2px solid {}", THEME.input_border_color))
    }
});
pub static LABEL:Lazy<String> = Lazy::new(|| {
    class!{
        .style("color", "var(--color-6)")
        .style("font-size", "var(--size-2)")
    }
});

pub static ERROR:Lazy<String> = Lazy::new(|| {
    class!{
        .style("color", THEME.error_color)
    }
});
