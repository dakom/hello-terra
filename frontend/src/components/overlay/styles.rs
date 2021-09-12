use once_cell::sync::Lazy;
use dominator::class;

pub static OVERLAY_BG:Lazy<String> = Lazy::new(|| {
    class!{
        .style("position", "fixed")
        .style("width", "100vw")
        .style("height", "100vh")
        .style("background-color", "rgba(255, 255, 255, 1.0)")
    }
});

pub static LOADER_CONTENT:Lazy<String> = Lazy::new(|| {
    class!{
        .style("display", "flex")
        .style("align-items", "center")
        .style("justify-content", "center")
        .style("margin-top", "calc(33vh)")
        .style("background-color", "white")
    }
});
pub static LOADER_TEXT:Lazy<String> = Lazy::new(|| {
    class!{
        .style("text-align", "center")
    }
});
