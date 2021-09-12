use dominator::{stylesheet, class};
use once_cell::sync::Lazy;
use crate::config::THEME;

pub const MAIN:Lazy<String> = Lazy::new(|| {
    class! {
    }
});

