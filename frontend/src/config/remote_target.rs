use std::fmt;
use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use dotenv::dotenv;
use crate::utils::{prelude::*, env::env_var};

pub static REMOTE_TARGET:Lazy<RemoteTarget> = Lazy::new(|| {
    let _ = dotenv().ok();

    RemoteTarget::new()
}); 

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Release,
}

impl RemoteTarget {
    cfg_if! {
        if #[cfg(feature = "local")] {
            pub fn new() -> Self {
                Self::Local
            }
        } else if #[cfg(feature = "release")] {
            pub fn new() -> Self { 
                Self::Release
            }
        } else {
            pub fn new() -> Self{ 
                panic!("set a feature target!");
            }
        } 
    }


    pub fn media_port(&self) -> u32 {
        match env_var("MEDIA_DEV_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 8080,
        }
    }
    pub fn media_url(&self) -> String {
        match self {
            Self::Local=> format!("http://localhost:{}", self.media_port()),
            Self::Release => "https://media.example.org".to_string(),
        }
    }
}