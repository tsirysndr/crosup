use indexmap::IndexMap;
use os_release::OsRelease;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GitConfiguration {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub repo: IndexMap<String, Repository>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Repository {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,
    pub url: String,
    pub install: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shallow_submodules: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
}

pub fn default_git_install() -> IndexMap<String, GitConfiguration> {
    let mut repo = IndexMap::new();

    let mut blesh = Repository {
        name: "blesh".into(),
        url: "https://github.com/akinomyoga/ble.sh.git".into(),
        install: "make -C ble.sh install PREFIX=~/.local".into(),
        preinstall: None,
        postinstall: Some("echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc".into()),
        install_check: Some("~/.local/share/blesh/ble.sh".into()),
        recursive: Some(true),
        depth: Some(1),
        shallow_submodules: Some(true),
        depends_on: None,
    };

    if cfg!(target_os = "linux") {
        // determine linux distribution using os-release
        if let Ok(os_release) = OsRelease::new() {
            let os = os_release.id.to_lowercase();
            let os = os.as_str();
            match os {
                "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" | "zorin" => {
                    blesh.preinstall = Some("sudo apt-get install -y gawk build-essential".into());
                }
                _ => {}
            }
        }
    }

    if cfg!(target_os = "macos") {
        blesh.preinstall = Some("brew install gawk bash".into());
        blesh.depends_on = Some(vec!["homebrew".into()]);
    }

    repo.insert("blesh".into(), blesh);
    let mut git = IndexMap::new();
    git.insert("install".into(), GitConfiguration { repo });
    git
}
