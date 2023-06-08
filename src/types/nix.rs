use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NixConfiguration {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub pkg: IndexMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Package {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impure: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_features: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_flake_config: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preinstall: Option<String>,
    pub flake: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,
}

pub fn default_nix_install() -> IndexMap<String, NixConfiguration> {
    let mut pkg = IndexMap::new();
    pkg.insert("flox".into(), 
        Package {
            name: "flox".into(),
            impure: Some(true),
            experimental_features: Some("nix-command flakes".into()),
            accept_flake_config: Some(true),
            flake: "github:flox/floxpkgs#flox.fromCatalog".into(),
            preinstall: Some("echo 'extra-trusted-substituters = https://cache.floxdev.com' | sudo tee -a /etc/nix/nix.conf && echo 'extra-trusted-public-keys = flox-store-public-0:8c/B+kjIaQ+BloCmNkRUKwaVPFWkriSAd0JJvuDu4F0=' | sudo tee -a /etc/nix/nix.conf".into()),
            version_check: Some(". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && flox --version".into()),
            ..Default::default()
        }
    );

    pkg.insert(
        "cachix".into(),
        Package {
            name: "cachix".into(),
            flake: "github:cachix/cachix".into(),
            ..Default::default()
        },
    );

    pkg.insert(
        "devenv".into(),
        Package {
            name: "devenv".into(),
            accept_flake_config: Some(true),
            flake: "github:cachix/devenv/latest".into(),
            preinstall: Some(
                r#"echo "trusted-users = root $USER" | sudo tee -a /etc/nix/nix.conf
sudo pkill nix-daemon
cachix use devenv"#
                    .into(),
            ),
            depends_on: Some(vec!["cachix".into()]),
            version_check: Some(
                ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && devenv version"
                    .into(),
            ),
            ..Default::default()
        },
    );

    let mut nix = IndexMap::new();
    nix.insert("install".into(), NixConfiguration { pkg });
    nix
}
