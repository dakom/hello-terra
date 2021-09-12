use cfg_if::cfg_if;
use once_cell::sync::Lazy;
use crate::utils::env::env_var;

cfg_if! {
    if #[cfg(all(feature = "local"))] {
        pub static DEBUG:Lazy<Debug> = Lazy::new(|| Debug::local());
    } else {
        pub static DEBUG:Lazy<Debug> = Lazy::new(|| Debug::default());
    }
}

#[derive(Default)]
pub struct Debug {
    pub wallet_mnemonic: Option<String>,
}

impl Debug {
    pub fn local() -> Self {
        Self {
            wallet_mnemonic: env_var("DEBUG_WALLET_MNEMONIC").ok(),
            //wallet_id: None, 
        }
    }
}