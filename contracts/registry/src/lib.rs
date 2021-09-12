pub mod msg;
use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(all(feature = "terra"))] {
        pub mod contract;
        cosmwasm_std::create_entry_points!(contract);
    } else {
    }
}