use indexmap::IndexMap;
use os_release::OsRelease;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AptConfiguration {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub pkg: IndexMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Package {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_repository: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apt_update: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,
}

pub fn default_apt_install() -> IndexMap<String, AptConfiguration> {
    let mut pkg = IndexMap::new();

    // detect linux
    if cfg!(target_os = "linux") {
        // determine linux distribution using os-release
        if let Ok(os_release) = OsRelease::new() {
            let os = os_release.id.to_lowercase();
            let os = os.as_str();

            if os == "debian" {
                pkg.insert(
                    "docker".into(),
                    Package {
                        name: "docker".into(),
                        gpg_key: Some("https://download.docker.com/linux/debian/gpg".into()),
                        gpg_path: Some("/etc/apt/keyrings/docker.gpg".into()),
                        setup_repository: Some(
                          r#"echo "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null"#.into()),
                        apt_update: Some(true),
                        packages: Some(vec![
                          "docker-ce".into(),
                          "docker-ce-cli".into(),
                          "containerd.io".into(),
                          "docker-buildx-plugin".into(),
                          "docker-compose-plugin".into()
                          ]),
                        depends_on: Some(vec!["ca-certificates".into(),"curl".into(), "gnupg".into()]),
                        postinstall: Some("sudo usermod -aG docker $USER && newgrp docker".into()),
                        ..Default::default()
                    },
                );
            }

            if os == "debian" || os == "ubuntu" {
                pkg.insert(
                    "vscode".into(),
                    Package {
                        name: "code".into(),
                        url: Some(
                            "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64".into(),
                        ),
                        version_check: Some("code".into()),
                        ..Default::default()
                    },
                );
            }
        }
    }

    let mut apt = IndexMap::new();
    apt.insert("install".into(), AptConfiguration { pkg });
    apt
}
