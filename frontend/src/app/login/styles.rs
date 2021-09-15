use dominator::{stylesheet, class, pseudo};
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
pub const MNEMONIC:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("justify-content", "center")
        .style("align-items", "center")
    }
});

pub const MNEMONIC_FIELDS:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "3rem")
    }
});

pub const CHOICES:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "grid")
        .style("grid-template-columns", "1fr 1fr 1fr")
        .style("justify-content", "center")
        .style("align-items", "center")
        /*
        .style("display", "flex")
        .style("flex-direction", "row")
        .style("justify-content", "center")
        .style("align-items", "center")
        .style("gap", "10rem")
        */
    }
});
pub const BUTTON_MULTI_LINE:Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("justify-content", "center")
        .style("align-items", "center")
        .style("gap", "10rem")
    }
});