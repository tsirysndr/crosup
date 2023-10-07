use indexmap::IndexMap;
use os_release::OsRelease;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BrewConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<Vec<String>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub pkg: Option<IndexMap<String, Package>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Package {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preinstall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cask: Option<bool>,
}

pub fn default_brew_install() -> IndexMap<String, BrewConfiguration> {
    let mut brew = IndexMap::new();
    let mut pkg = IndexMap::new();
    let mut minikube = super::brew::Package {
        name: "minikube".into(),
        preinstall: None,
        postinstall: None,
        ..Default::default()
    };

    if cfg!(target_os = "linux") {
        // determine linux distribution using os-release
        if let Ok(os_release) = OsRelease::new() {
            let os = os_release.id.to_lowercase();
            let os = os.as_str();
            match os {
                "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" | "zorin" => {
                    minikube.preinstall = Some(
                        "sudo apt-get install -y qemu-system libvirt-clients libvirt-daemon-system"
                            .into(),
                    );
                    minikube.postinstall = Some(
                        r#"sudo sed -i 's/#user = "root"/user = "root"/g' /etc/libvirt/qemu.conf
            sudo sed -i 's/#group = "root"/group = "root"/g' /etc/libvirt/qemu.conf
            sudo sed -i 's/#dynamic_ownership = 1/dynamic_ownership = 0/g' /etc/libvirt/qemu.conf
            sudo sed -i 's/#remember_owner = 1/remember_owner = 0/g' /etc/libvirt/qemu.conf"#
                            .into(),
                    );
                }
                _ => {}
            }
        }
    }

    pkg.insert("minikube".into(), minikube);

    pkg.insert(
        "tilt".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "kubernetes-cli".into(),
        super::brew::Package {
            version_check: Some("kubectl".into()),
            ..Default::default()
        },
    );

    pkg.insert(
        "bat".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "direnv".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "exa".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "fd".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "fzf".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "fish".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "glow".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "httpie".into(),
        super::brew::Package {
            version_check: Some("http".into()),
            ..Default::default()
        },
    );

    pkg.insert(
        "tig".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "zellij".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "zoxide".into(),
        super::brew::Package {
            ..Default::default()
        },
    );

    pkg.insert(
        "ripgrep".into(),
        super::brew::Package {
            version_check: Some("rg".into()),
            ..Default::default()
        },
    );

    pkg.insert(
        "neovim".into(),
        super::brew::Package {
            version_check: Some("nvim".into()),
            ..Default::default()
        },
    );

    if cfg!(target_os = "macos") {
        pkg.insert(
            "docker".into(),
            super::brew::Package {
                cask: Some(true),
                ..Default::default()
            },
        );
        pkg.insert(
            "visual-studio-code".into(),
            super::brew::Package {
                cask: Some(true),
                version_check: Some("code".into()),
                ..Default::default()
            },
        );
    }

    brew.insert(
        "install".into(),
        BrewConfiguration {
            pkg: Some(pkg),
            ..Default::default()
        },
    );
    brew
}
