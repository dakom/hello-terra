use once_cell::sync::Lazy;
use crate::utils::contract::*;

pub static KNOWN_CONTRACT_IDS:Lazy<Vec<(ContractIdLookupKey, u64)>> = Lazy::new(|| {
    Vec::new()
});

pub static KNOWN_CONTRACT_ADDRS:Lazy<Vec<(ContractAddrLookupKey, String)>> = Lazy::new(|| {
    Vec::new()
});
pub const CONTRACT_ACCOUNT_ID_STORAGE:&'static str = "CONTRACT-ACCOUNT-ID";
pub const CONTRACT_ACCOUNT_ADDR_STORAGE:&'static str = "CONTRACT-ACCOUNT-ADDR";
