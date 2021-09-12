use cfg_if::cfg_if;
use std::env::VarError;

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

    pub fn terra_local_port(&self) -> u32 {
        match env_var("TERRA_DEV_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 8080,
        }
    }

    //Falls back to Test for release and Local for local
    //can be overridden by env var
    fn terra_chain(&self) -> TerraChain {
        let res = match env_var("TERRA_CHAIN") {
            Ok(chain) => {
                match chain.as_ref() {
                    "main" => Some(TerraChain::Main),
                    "test" => Some(TerraChain::Test),
                    "local" => Some(TerraChain::Local),
                    _ => {
                        log::error!("[{}] is an unsupported chain override! using fallback...", chain);
                        None
                    }
                }
            },
            _ => None
        };

        match res {
            Some(chain) => chain,
            None => {
                match self {
                    Self::Local => TerraChain::Local,
                    Self::Release => TerraChain::Test
                }
            }
        }
    }
    pub fn terra_url(&self) -> String {
        match self.terra_chain() {
            TerraChain::Local => format!("http://localhost:{}", self.terra_local_port()),
            TerraChain::Main => panic!("not supported yet!"),
            TerraChain::Test => "https://bombay-lcd.terra.dev".to_string()
        }
    }

    pub fn terra_chain_id(&self) -> &'static str {
        match self.terra_chain() {
            TerraChain::Local=> "localterra",
            TerraChain::Test => "bombay-10", 
            TerraChain::Main => panic!("not supported yet!") 
        }
    }
}

enum TerraChain {
    Main,
    Test,
    Local
}

//Get env var, for both wasm and terra environments
cfg_if! {
    if #[cfg(feature = "frontend")] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(inline_js = "export function process_env_var(key) { const value = process.env[key]; return value == undefined ? '' : value; }")]
        extern "C" {
            #[wasm_bindgen(catch)]
            fn process_env_var(key:&str) -> Result<String, JsValue>;
        }

        pub fn env_var(key: &str) -> Result<String, VarError> {
            process_env_var(key)
                .map_err(|_| {
                    VarError::NotPresent
                })
                .and_then(|var| if var.is_empty() { Err(VarError::NotPresent) } else { Ok(var) })
        }
    } else {
        pub fn env_var(key: &str) -> Result<String, VarError> {
            std::env::var(key)
        }
    }
}
