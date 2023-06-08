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
- [x] Install neovim
- [x] Install zoxide
- [x] Install direnv
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
    init       Generate a default configuration file
    install    Install developer tools, possible values are: docker, nix, devbox, homebrew,
                   flox, fish, vscode, ble.sh, atuin, tig, fzf, httpie, kubectl, minikube, tilt,
                   zellij, ripgrep, fd, exa, bat, glow, devenv
```

## 📝 Configuration
Crosup uses a configuration file to determine which tools to install. The default configuation is embedded in the binary, but you can generate a default configuration file (Crosfile.hcl) using the `crosup init` subcommand, you can specify the default format using the `--toml` flag.

Example configuration file in HCL format:

```hcl
# Crosfile.hcl
brew "install" {
  pkg "minikube" {
    preinstall = "sudo apt-get install -y qemu-system libvirt-clients libvirt-daemon-system"
    postinstall = "sudo sed -i 's/#user = \"root\"/user = \"root\"/g' /etc/libvirt/qemu.conf\nsudo sed -i 's/#group = \"root\"/group = \"root\"/g' /etc/libvirt/qemu.conf\nsudo sed -i 's/#dynamic_ownership = 1/dynamic_ownership = 0/g' /etc/libvirt/qemu.conf\nsudo sed -i 's/#remember_owner = 1/remember_owner = 0/g' /etc/libvirt/qemu.conf"
    version_check = "minikube version"
  }

  pkg "tilt" {
    version_check = "tilt version"
  }

  pkg "kubernetes-cli" {
    version_check = "kubectl version --client"
  }

  pkg "bat" {
    version_check = "bat --version"
  }

  pkg "direnv" {
    version_check = "direnv --version"
  }

  pkg "exa" {
    version_check = "exa --version"
  }

  pkg "fd" {
    version_check = "fd --version"
  }

  pkg "fzf" {
    version_check = "fzf --version"
  }

  pkg "fish" {
    version_check = "fish --version"
  }

  pkg "glow" {
    version_check = "glow --version"
  }

  pkg "httpie" {
    version_check = "http --version"
  }

  pkg "tig" {
    version_check = "tig --version"
  }

  pkg "zellij" {
    version_check = "zellij --version"
  }

  pkg "zoxide" {
    version_check = "zoxide --version"
  }

  pkg "ripgrep" {
    version_check = "rg --version"
  }

  pkg "neovim" {
    version_check = "nvim --version"
  }
}

git "install" {
  repo "blesh" {
    url = "https://github.com/akinomyoga/ble.sh.git"
    install = "make -C ble.sh install PREFIX=~/.local"
    preinstall = "sudo apt-get install -y gawk build-essential"
    postinstall = "echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc"
    install_check = "~/.local/share/blesh/ble.sh"
    recursive = true
    depth = 1
    shallow_submodules = true
  }
}

nix "install" {
  pkg "flox" {
    impure = true
    experimental_features = "nix-command flakes"
    accept_flake_config = true
    preinstall = "echo 'extra-trusted-substituters = https://cache.floxdev.com' | sudo tee -a /etc/nix/nix.conf && echo 'extra-trusted-public-keys = flox-store-public-0:8c/B+kjIaQ+BloCmNkRUKwaVPFWkriSAd0JJvuDu4F0=' | sudo tee -a /etc/nix/nix.conf"
    flake = "github:flox/floxpkgs#flox.fromCatalog"
    version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && flox --version"
  }

  pkg "cachix" {
    flake = "github:cachix/cachix"
  }

  pkg "devenv" {
    accept_flake_config = true
    preinstall = "echo \"trusted-users = root $USER\" | sudo tee -a /etc/nix/nix.conf\nsudo pkill nix-daemon\ncachix use devenv"
    flake = "github:cachix/devenv/latest"
    depends_on = [
      "cachix"
    ]
    version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && devenv version"
  }
}

curl "install" {
  script "devbox" {
    url = "https://get.jetpack.io/devbox"
    version_check = "devbox version"
    shell = "bash"
    depends_on = [
      "nix"
    ]
  }

  script "atuin" {
    url = "https://raw.githubusercontent.com/ellie/atuin/main/install.sh"
    version_check = "atuin --version"
    shell = "bash"
  }

  script "nix" {
    url = "https://install.determinate.systems/nix"
    enable_sudo = true
    version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix --version"
    args = "install --no-confirm"
  }

  script "homebrew" {
    url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
    postinstall = "echo 'eval $(/home/linuxbrew/.linuxbrew/bin/brew shellenv)' >> ~/.bashrc"
    version_check = "brew --version"

    env {
      NONINTERACTIVE = "true"
    }

    shell = "bash"
  }
}

apt "install" {
  pkg "docker" {
    gpg_key = "https://download.docker.com/linux/debian/gpg"
    gpg_path = "/etc/apt/keyrings/docker.gpg"
    setup_repository = "echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null"
    apt_update = true
    packages = [
      "docker-ce",
      "docker-ce-cli",
      "containerd.io",
      "docker-buildx-plugin",
      "docker-compose-plugin"
    ]
    depends_on = [
      "ca-certificates",
      "curl",
      "gnupg"
    ]
    postinstall = "sudo usermod -aG docker $USER && newgrp docker"
    version_check = "docker --version"
  }

  pkg "vscode" {
    url = "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
    version_check = "code --version"
  }
}
```

Example of a Crosfile.toml for a Debian-based system:

```toml
# Crosfile.toml
[brew.install.pkg.neovim]
version_check = "nvim --version"

[git.install.repo.blesh]
url = "https://github.com/akinomyoga/ble.sh.git"
install = "make -C ble.sh install PREFIX=~/.local"
preinstall = "sudo apt-get install -y gawk build-essential"
postinstall = "echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc"
install_check = "~/.local/share/blesh/ble.sh"
recursive = true
depth = 1
shallow_submodules = true

[nix.install.pkg.flox]
impure = true
experimental_features = "nix-command flakes"
accept_flake_config = true
preinstall = "echo 'extra-trusted-substituters = https://cache.floxdev.com' | sudo tee -a /etc/nix/nix.conf && echo 'extra-trusted-public-keys = flox-store-public-0:8c/B+kjIaQ+BloCmNkRUKwaVPFWkriSAd0JJvuDu4F0=' | sudo tee -a /etc/nix/nix.conf"
flake = "github:flox/floxpkgs#flox.fromCatalog"
version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && flox --version"

[nix.install.pkg.cachix]
flake = "github:cachix/cachix"

[nix.install.pkg.devenv]
accept_flake_config = true
preinstall = """
echo \"trusted-users = root $USER\" | sudo tee -a /etc/nix/nix.conf
sudo pkill nix-daemon
cachix use devenv"""
flake = "github:cachix/devenv/latest"
depends_on = ["cachix"]
version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && devenv version"

[curl.install.script.devbox]
url = "https://get.jetpack.io/devbox"
version_check = "devbox version"
shell = "bash"
depends_on = ["nix"]

[curl.install.script.atuin]
url = "https://raw.githubusercontent.com/ellie/atuin/main/install.sh"
version_check = "atuin --version"
shell = "bash"

[curl.install.script.nix]
url = "https://install.determinate.systems/nix"
enable_sudo = true
version_check = ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix --version"
args = "install --no-confirm"

[curl.install.script.homebrew]
url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
postinstall = "echo 'eval $(/home/linuxbrew/.linuxbrew/bin/brew shellenv)' >> ~/.bashrc"
version_check = "brew --version"
shell = "bash"

[curl.install.script.homebrew.env]
NONINTERACTIVE = "true"

[apt.install.pkg.docker]
gpg_key = "https://download.docker.com/linux/debian/gpg"
gpg_path = "/etc/apt/keyrings/docker.gpg"
setup_repository = "echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null"
apt_update = true
packages = [
    "docker-ce",
    "docker-ce-cli",
    "containerd.io",
    "docker-buildx-plugin",
    "docker-compose-plugin",
]
depends_on = [
    "ca-certificates",
    "curl",
    "gnupg",
]
postinstall = "sudo usermod -aG docker $USER && newgrp docker"
version_check = "docker --version"

[apt.install.pkg.vscode]
url = "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
version_check = "code --version"

```

## 🤝 Contributing
Contributions, issues and feature requests are welcome!
See [CONTRIBUTING](./CONTRIBUTING.md) for more information.

## 📝 License
[MIT](./LICENSE)
