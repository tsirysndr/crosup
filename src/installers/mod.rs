use anyhow::Error;

pub mod atuin;
pub mod blesh;
pub mod docker;
pub mod fish;
pub mod homebrew;
pub mod nix;
pub mod vscode;

pub trait Installer {
    fn install(&self) -> Result<(), Error>;
    fn is_installed(&self) -> Result<bool, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn dependencies(&self) -> Vec<String>;
}
