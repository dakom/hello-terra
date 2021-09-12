use std::{rc::Rc, cell::RefCell};
use dominator_helpers::futures::AsyncLoader;
use web_sys::HtmlInputElement;
use crate::app::App;

pub struct Registry {
    pub wallet_id: String,
    pub app: Rc<App>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
}

impl Registry {
    pub fn new(wallet_id: String, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            wallet_id,
            app,
            file_input: RefCell::new(None),
            loader: AsyncLoader::new()
        })
    }
}