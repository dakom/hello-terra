use once_cell::sync::Lazy;
use dominator::{class, pseudo};
use crate::config::THEME;

pub static CURSOR: Lazy<String> = Lazy::new(|| class! {
    .style("cursor", "pointer")
});

pub static COLOR: Lazy<String> = Lazy::new(|| class! {
    .style("display", "inline-block")
    .style("padding", "10rem")
    .style("border-radius", "10rem")
});

pub static COLOR_TEXT: Lazy<String> = Lazy::new(|| class! {
    .style("display", "flex")
    .style("pointer-events", "none")
    .style("user-select", "none")
    .style("align-items", "center")
    .style("justify-content", "center")
    .style("font-size", "16rem")
});

pub static COLOR_BLUE: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", THEME.button_color_blue.bg_regular) 
    .style("color", THEME.button_color_blue.text_regular) 
    .pseudo!(":hover", {
        .style("background-color", THEME.button_color_blue.bg_hover) 
        .style("color", THEME.button_color_blue.text_hover) 
    })
});

pub static COLOR_RED: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", THEME.button_color_red.bg_regular) 
    .style("color", THEME.button_color_red.text_regular) 
    .pseudo!(":hover", {
        .style("background-color", THEME.button_color_red.bg_hover) 
        .style("color", THEME.button_color_red.text_hover) 
    })
});