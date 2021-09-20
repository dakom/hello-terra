use once_cell::sync::Lazy;
use crate::utils::contract::*;

pub static KNOWN_CONTRACT_CODE_IDS:Lazy<Vec<(ContractCodeIdLookupKey, u64)>> = Lazy::new(|| {
    vec![
        (
            ContractCodeIdLookupKey{
                chain_id: "bombay-10".to_string(),
                file_hash: "7f15e2071140e650ab797f7aa4d27f1090b946a5f338a8121947be8287361bef".to_string()
            },
            11559
        ),

        (
            ContractCodeIdLookupKey{
                chain_id: "bombay-10".to_string(),
                file_hash: "4219e08824838441c8489210b2193f896a545be0d1a1ab7adf8a9b5a7344a888".to_string()
            },
            11560
        ),
    ]
});

pub static KNOWN_CONTRACT_HUB_ADDRS:Lazy<Vec<(ContractHubAddrLookupKey, String)>> = Lazy::new(|| {
    vec![
        (
            ContractHubAddrLookupKey {
                chain_id: "bombay-10".to_string(),
                code_id: 11559
            },
            "terra1nzc7ljhdmjwnpfydqykc9rs2zzv2kxdqlf7tqz".to_string()
        )
    ]
});
pub const CONTRACT_CODE_ID_STORAGE:&'static str = "CONTRACT-CODE-ID";
pub const CONTRACT_HUB_ADDR_STORAGE:&'static str = "CONTRACT-ACCOUNT-HUB-ADDR";


pub const CONTRACT_ACCOUNT_HASH_URI:&'static str = "contracts/contract-account-hash.txt";
pub const CONTRACT_ACCOUNT_WASM_URI:&'static str = "contracts/contract-account.wasm";

pub const CONTRACT_HUB_HASH_URI:&'static str = "contracts/contract-hub-hash.txt";
pub const CONTRACT_HUB_WASM_URI:&'static str = "contracts/contract-hub.wasm";