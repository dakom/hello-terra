use once_cell::sync::Lazy;
use crate::utils::contract::*;

pub static KNOWN_CONTRACT_CODE_IDS:Lazy<Vec<(ContractCodeIdLookupKey, u64)>> = Lazy::new(|| {
    Vec::new()
});

pub static KNOWN_CONTRACT_HUB_ADDRS:Lazy<Vec<(ContractHubAddrLookupKey, String)>> = Lazy::new(|| {
    Vec::new()
});
pub const CONTRACT_CODE_ID_STORAGE:&'static str = "CONTRACT-CODE-ID";
pub const CONTRACT_HUB_ADDR_STORAGE:&'static str = "CONTRACT-ACCOUNT-HUB-ADDR";


pub const CONTRACT_ACCOUNT_HASH_URI:&'static str = "contracts/contract-account-hash.txt";
pub const CONTRACT_ACCOUNT_WASM_URI:&'static str = "contracts/contract-account.wasm";

pub const CONTRACT_HUB_HASH_URI:&'static str = "contracts/contract-hub-hash.txt";
pub const CONTRACT_HUB_WASM_URI:&'static str = "contracts/contract-hub.wasm";