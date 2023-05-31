use anyhow::Error;

pub mod atuin;
pub mod blesh;
pub mod devbox;
pub mod docker;
pub mod fd;
pub mod fish;
pub mod flox;
pub mod fzf;
pub mod homebrew;
pub mod httpie;
pub mod kubectl;
pub mod minikube;
pub mod nix;
pub mod ripgrep;
pub mod tig;
pub mod tilt;
pub mod vscode;
pub mod zellij;

pub trait Installer {
    fn install(&self) -> Result<(), Error>;
    fn is_installed(&self) -> Result<bool, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn dependencies(&self) -> Vec<String>;
    fn is_default(&self) -> bool;
}
