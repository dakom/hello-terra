use std::fmt;
use cfg_if::cfg_if;
use shared::config::RemoteTarget;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use dotenv::dotenv;
use crate::utils::{prelude::*, env::env_var};

pub static REMOTE_TARGET:Lazy<RemoteTarget> = Lazy::new(|| {
    let _ = dotenv().ok();

    RemoteTarget::new()
}); 

pub trait RemoteTargetExt {
    fn media_port(&self) -> u32;
    fn media_url(&self) -> String;
}

impl RemoteTargetExt for RemoteTarget {
    fn media_port(&self) -> u32 {
        match env_var("MEDIA_DEV_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 8080,
        }
    }
    fn media_url(&self) -> String {
        match self {
            Self::Local=> format!("http://localhost:{}", self.media_port()),
            Self::Release => "https://media.example.org".to_string(),
        }
    }
}
