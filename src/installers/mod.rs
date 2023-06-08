use std::any::Any;

use anyhow::Error;

pub mod _apt;
pub mod _brew;
pub mod _curl;
pub mod _git;
pub mod _nix;

pub mod atuin;
pub mod bat;
pub mod blesh;
pub mod devbox;
pub mod devenv;
pub mod direnv;
pub mod docker;
pub mod exa;
pub mod fd;
pub mod fish;
pub mod flox;
pub mod fzf;
pub mod glow;
pub mod homebrew;
pub mod httpie;
pub mod kubectl;
pub mod minikube;
pub mod neovim;
pub mod nix;
pub mod ripgrep;
pub mod tig;
pub mod tilt;
pub mod vscode;
pub mod zellij;
pub mod zoxide;

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
