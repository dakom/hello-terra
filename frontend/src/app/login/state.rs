use std::{rc::Rc, cell::RefCell};
use crate::app::App;
use futures_signals::signal::Mutable;
use dominator_helpers::futures::AsyncLoader;
use crate::config::DEBUG;

pub struct Login {
    pub app: Rc<App>,
    pub loader: AsyncLoader,
    pub input_value: RefCell<Option<String>>,
    pub input_error: Mutable<Option<String>>,
}

impl Login {
    pub fn new(app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            loader: AsyncLoader::new(),
            input_value: RefCell::new(DEBUG.wallet_mnemonic.clone()),
            input_error: Mutable::new(None),
        })
    }
}