pub mod apt;
pub mod brew;
pub mod configuration;
pub mod curl;
pub mod dnf;
pub mod git;
pub mod nix;
pub mod yum;

pub const CROSFILE_TOML: &str = "Crosfile.toml";
pub const CROSFILE_HCL: &str = "Crosfile.hcl";
