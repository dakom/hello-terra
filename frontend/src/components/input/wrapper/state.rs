use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};

pub struct InputWrapper {
    pub label: Mutable<Option<String>>,
    pub error: Mutable<Option<String>>,
}

impl InputWrapper {
    pub fn new() -> Self {
        Self {
            label: Mutable::new(None),
            error: Mutable::new(None),
        }
    }

    pub fn new_label<A: AsRef<str>>(label: A) -> Self {
        Self {
            label: Mutable::new(Some(label.as_ref().to_string())),
            error: Mutable::new(None),
            //error: Mutable::new(Some("Error here!".to_string())),
        }
    }
}
