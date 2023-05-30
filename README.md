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
- [x] Install VSCode
- [x] Install Docker
- [x] Install Nix
- [x] Install Flox
- [x] Install Devbox
- [x] Install Homebrew
- [x] Install Fish
- [x] Install Ble.sh
- [x] Install Atuin
- [x] Install Tig
  
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
                   flox, fish, vscode, ble.sh, atuin, tig
```
