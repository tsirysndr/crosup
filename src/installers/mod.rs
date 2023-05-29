use anyhow::Error;

pub mod atuin;
pub mod blesh;
pub mod devbox;
pub mod docker;
pub mod fish;
pub mod flox;
pub mod homebrew;
pub mod nix;
pub mod tig;
pub mod vscode;

pub trait Installer {
    fn install(&self) -> Result<(), Error>;
    fn is_installed(&self) -> Result<bool, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn dependencies(&self) -> Vec<String>;
    fn is_default(&self) -> bool;
}
