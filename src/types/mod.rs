pub mod apk;
pub mod apt;
pub mod brew;
pub mod configuration;
pub mod curl;
pub mod dnf;
pub mod git;
pub mod nix;
pub mod pacman;
pub mod yum;
pub mod zypper;

pub const CROSFILE_TOML: &str = "Crosfile.toml";
pub const CROSFILE_HCL: &str = "Crosfile.hcl";
