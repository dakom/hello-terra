use std::{rc::Rc, cell::RefCell};
use crate::app::App;
use futures_signals::signal::Mutable;
use dominator_helpers::futures::AsyncLoader;
use crate::config::DEBUG;

pub struct Login {
    pub app: Rc<App>,
}

impl Login {
    pub fn new(app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
        })
    }
}