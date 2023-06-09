use std::any::Any;

use anyhow::Error;

pub mod apt;
pub mod brew;
pub mod curl;
pub mod dnf;
pub mod git;
pub mod nix;
pub mod yum;

pub trait Installer {
    fn install(&self) -> Result<(), Error>;
    fn is_installed(&self) -> Result<bool, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn dependencies(&self) -> Vec<String>;
    fn is_default(&self) -> bool;
    fn provider(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
