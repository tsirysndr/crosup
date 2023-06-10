pub mod apk;
pub mod apt;
pub mod brew;
pub mod configuration;
pub mod curl;
pub mod dnf;
pub mod emerge;
pub mod git;
pub mod install;
pub mod inventory;
pub mod nix;
pub mod pacman;
pub mod yum;
pub mod zypper;

pub const CROSFILE_TOML: &str = "Crosfile.toml";
pub const CROSFILE_HCL: &str = "Crosfile.hcl";
pub const INVENTORY_TOML: &str = "Inventory.toml";
pub const INVENTORY_HCL: &str = "Inventory.hcl";
