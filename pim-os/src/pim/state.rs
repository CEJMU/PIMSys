use super::config;
use pim_isa::{BankMode, Kernel, PimConfig};

// TODO return token and return to singlebank when dropped
pub fn set_bank_mode(bank_mode: BankMode) {
    config::write(
        serde_json_core::to_string::<PimConfig, 64>(&PimConfig {
            kernel: None,
            bank_mode: Some(bank_mode),
        })
        .unwrap()
        .as_str(),
    );
}

pub fn set_kernel(kernel: &Kernel) {
    config::write(
        serde_json_core::to_string::<PimConfig, 2048>(&PimConfig {
            kernel: Some(kernel.clone()),
            bank_mode: None,
        })
        .unwrap()
        .as_str(),
    );
}
