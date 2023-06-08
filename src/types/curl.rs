use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CurlConfiguration {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub script: IndexMap<String, Script>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Script {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sudo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub env: Option<IndexMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
}

pub fn default_nix_installer() -> Script {
    Script {
        name: "nix".into(),
        url: "https://install.determinate.systems/nix".into(),
        enable_sudo: Some(true),
        version_check: Some(
            ". /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix --version".into(),
        ),
        args: Some("install --no-confirm".into()),
        ..Default::default()
    }
}

pub fn default_brew_installer() -> Script {
    Script {
        name: "homebrew".into(),
        url: "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh".into(),
        postinstall: Some(
            "echo 'eval $(/home/linuxbrew/.linuxbrew/bin/brew shellenv)' >> ~/.bashrc".into(),
        ),
        version_check: Some("brew --version".into()),
        env: Some(
            [("NONINTERACTIVE".into(), "true".into())]
                .iter()
                .cloned()
                .collect(),
        ),
        shell: Some("bash".into()),
        ..Default::default()
    }
}

pub fn default_curl_install() -> IndexMap<String, CurlConfiguration> {
    let mut script = IndexMap::new();
    script.insert(
        "devbox".into(),
        Script {
            name: "devbox".into(),
            url: "https://get.jetpack.io/devbox".into(),
            version_check: Some("devbox version".into()),
            shell: Some("bash".into()),
            depends_on: Some(vec!["nix".into()]),
            env: Some([("FORCE".into(), "1".into())].iter().cloned().collect()),
            ..Default::default()
        },
    );

    script.insert(
        "atuin".into(),
        Script {
            name: "atuin".into(),
            url: "https://raw.githubusercontent.com/ellie/atuin/main/install.sh".into(),
            version_check: Some("atuin --version".into()),
            shell: Some("bash".into()),
            ..Default::default()
        },
    );

    script.insert("nix".into(), default_nix_installer());

    script.insert("homebrew".into(), default_brew_installer());

    let mut curl = IndexMap::new();
    curl.insert("install".into(), CurlConfiguration { script });
    curl
}
