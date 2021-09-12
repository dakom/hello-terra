use std::rc::Rc;
use crate::app::App;

pub struct Header {
    pub app: Rc<App> 
}

impl Header {
    pub fn new(app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app
        })
    }
}