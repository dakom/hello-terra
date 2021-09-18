use cfg_if::cfg_if;

pub mod contract;
pub mod utils;
pub mod state;

cfg_if! {
    if #[cfg(all(feature = "entry"))] {
        cosmwasm_std::create_entry_points!(contract);
    } else {
    }
}