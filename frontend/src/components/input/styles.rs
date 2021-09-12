use once_cell::sync::Lazy;
use dominator::class;
use crate::config::THEME;

pub static TEXT:Lazy<String> = Lazy::new(|| {
    class!{
        .style("color", THEME.input_text_color) 
        .style("font-size", "14rem")
    }
});

pub static INPUT:Lazy<String> = Lazy::new(|| {
    class!{
        .style("outline", "none")
        .style("border", "none")
        .style("display", "block")
    }
});

