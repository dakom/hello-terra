use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use crate::utils::env::env_var;
use std::sync::atomic::{AtomicBool, Ordering};

cfg_if! {
    if #[cfg(all(feature = "local"))] {
        pub static DEBUG:Lazy<Debug> = Lazy::new(|| Debug::local());
    } else {
        pub static DEBUG:Lazy<Debug> = Lazy::new(|| Debug::default());
    }
}

#[derive(Default)]
pub struct Debug {
    pub auto_bootstrap_and_register: bool,
    _auto_login_manually: bool,
    has_auto_loggedin_once: AtomicBool
}

impl Debug {
    pub fn local() -> Self {
        Self {
            _auto_login_manually: true,
            auto_bootstrap_and_register: true,
            has_auto_loggedin_once: AtomicBool::new(false)
        }
    }

    pub fn auto_login_manually(&self) -> bool {
        if self._auto_login_manually {
            if !self.has_auto_loggedin_once.load(Ordering::SeqCst) {
                self.has_auto_loggedin_once.store(true, Ordering::SeqCst);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}