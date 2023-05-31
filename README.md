# Crosup

<p>
  <a href="LICENSE" target="./LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://crates.io/crates/crosup" target="_blank">
    <img src="https://img.shields.io/crates/v/crosup.svg" />
  </a>
  
  <a href="https://crates.io/crates/crosup" target="_blank">
    <img src="https://img.shields.io/crates/dr/crosup" />
  </a>

  <a href="https://github.com/tsirysndr/crosup/actions/workflows/release.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/crosup/actions/workflows/release.yml/badge.svg" />
  </a>
</p>

<img src="./preview.png" width="100%" style="margin-top: 20px; margin-bottom: 20px;" />

Crosup is a CLI tool to help you quickly setup your development environment on a new Chromebook (ChromeOS). It is designed to be simple and easy to use.

## 📦 Features
- [x] Install vscode
- [x] Install docker
- [x] Install nix
- [x] Install flox
- [x] Install devbox
- [x] Install homebrew
- [x] Install fish
- [x] Install ble.sh
- [x] Install atuin
- [x] Install tig
- [x] Install fzf
- [x] Install httpie
- [x] Install kubectl
- [x] Install minikube
- [x] Install tilt
- [x] Install zellij
- [x] Install ripgrep
- [x] Install fd
- [x] Install exa
- [x] Install bat
- [x] Install glow
- [x] Install devenv
## 🚚 Installation
```sh
curl -sSL https://raw.githubusercontent.com/tsirysndr/crosup/master/install.sh | bash
```

## 🚀 Usage
```
             ______                __  __
            / ____/________  _____/ / / /___
           / /   / ___/ __ \/ ___/ / / / __ \
          / /___/ /  / /_/ (__  ) /_/ / /_/ /
          \____/_/   \____/____/\____/ .___/
                                    /_/

ChromeOS developer environment setup tool

USAGE:
    crosup [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    install    Install developer tools, possible values are: docker, nix, devbox, homebrew,
                   flox, fish, vscode, ble.sh, atuin, tig, fzf, httpie, kubectl, minikube, tilt,
                   zellij, ripgrep, fd, exa, bat, glow, devenv
```
