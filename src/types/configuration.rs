use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{
    apk::ApkConfiguration,
    apt::{default_apt_install, AptConfiguration},
    brew::{default_brew_install, BrewConfiguration},
    curl::{default_curl_install, CurlConfiguration},
    dnf::DnfConfiguration,
    emerge::EmergeConfiguration,
    git::{default_git_install, GitConfiguration},
    nix::{default_nix_install, NixConfiguration},
    pacman::PacmanConfiguration,
    yum::YumConfiguration,
    zypper::ZypperConfiguration,
};

pub enum ConfigFormat {
    TOML,
    HCL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub brew: Option<IndexMap<String, BrewConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub git: Option<IndexMap<String, GitConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub nix: Option<IndexMap<String, NixConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub curl: Option<IndexMap<String, CurlConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub apt: Option<IndexMap<String, AptConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub yum: Option<IndexMap<String, YumConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub dnf: Option<IndexMap<String, DnfConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub zypper: Option<IndexMap<String, ZypperConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub apk: Option<IndexMap<String, ApkConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub pacman: Option<IndexMap<String, PacmanConfiguration>>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub emerge: Option<IndexMap<String, EmergeConfiguration>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            brew: Some(default_brew_install()),
            git: Some(default_git_install()),
            nix: Some(default_nix_install()),
            curl: Some(default_curl_install()),
            apt: Some(default_apt_install()),
            yum: None,
            dnf: None,
            zypper: None,
            apk: None,
            pacman: None,
            emerge: None,
        }
    }
}
