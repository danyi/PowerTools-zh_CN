mod detect;
pub mod driver;
mod error;
mod general;
mod min_max;
mod traits;

pub mod generic;
pub mod steam_deck;
pub mod unknown;

pub use detect::{auto_detect0, auto_detect_provider, limits_worker::spawn as limits_worker_spawn};
pub use driver::Driver;
pub use general::{SettingVariant, Settings, General};
pub use min_max::MinMax;

pub use error::SettingError;
pub use traits::{OnResume, OnSet, SettingsRange, TGeneral, TGpu, TCpus, TBattery, TCpu};

#[cfg(test)]
mod tests {
    #[test]
    fn system_defaults_test() {
        let settings = super::Settings::system_default("idc".into());
        println!("Loaded system settings: {:?}", settings);
    }
}
