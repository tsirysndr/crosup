use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::slackpkg;

use super::{apk, apt, brew, dnf, emerge, pacman, yum, zypper};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstallConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,

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
    pub preinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_interactive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub apk: Option<apk::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub apt: Option<apt::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub brew: Option<brew::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub dnf: Option<dnf::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub emerge: Option<emerge::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub pacman: Option<pacman::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub yum: Option<yum::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub zypper: Option<zypper::Package>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub slackpkg: Option<slackpkg::Package>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cask: Option<bool>,
}

impl Into<brew::BrewConfiguration> for InstallConfiguration {
    fn into(self) -> brew::BrewConfiguration {
        let pkg = Some(
            self.pkg
                .into_iter()
                .map(|(name, pkg)| match pkg.brew {
                    Some(brew) => (name.clone(), brew),
                    None => (name.clone(), brew::Package { name, ..pkg.into() }),
                })
                .collect(),
        );
        brew::BrewConfiguration {
            pkgs: self.packages,
            pkg,
        }
    }
}

impl Into<apk::ApkConfiguration> for InstallConfiguration {
    fn into(self) -> apk::ApkConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.apk {
                Some(apk) => (name.clone(), apk),
                None => (name.clone(), apk::Package { name, ..pkg.into() }),
            })
            .collect();
        apk::ApkConfiguration { pkg }
    }
}

impl Into<apt::AptConfiguration> for InstallConfiguration {
    fn into(self) -> apt::AptConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.apt {
                Some(apt) => (name.clone(), apt),
                None => (name.clone(), apt::Package { name, ..pkg.into() }),
            })
            .collect();
        apt::AptConfiguration { pkg }
    }
}

impl Into<dnf::DnfConfiguration> for InstallConfiguration {
    fn into(self) -> dnf::DnfConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.dnf {
                Some(dnf) => (name.clone(), dnf),
                None => (name.clone(), dnf::Package { name, ..pkg.into() }),
            })
            .collect();
        dnf::DnfConfiguration { pkg }
    }
}

impl Into<emerge::EmergeConfiguration> for InstallConfiguration {
    fn into(self) -> emerge::EmergeConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.emerge {
                Some(emerge) => (name.clone(), emerge),
                None => (name.clone(), emerge::Package { name, ..pkg.into() }),
            })
            .collect();
        emerge::EmergeConfiguration { pkg }
    }
}

impl Into<pacman::PacmanConfiguration> for InstallConfiguration {
    fn into(self) -> pacman::PacmanConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.pacman {
                Some(pacman) => (name.clone(), pacman),
                None => (name.clone(), pacman::Package { name, ..pkg.into() }),
            })
            .collect();
        pacman::PacmanConfiguration { pkg }
    }
}

impl Into<yum::YumConfiguration> for InstallConfiguration {
    fn into(self) -> yum::YumConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.yum {
                Some(yum) => (name.clone(), yum),
                None => (name.clone(), yum::Package { name, ..pkg.into() }),
            })
            .collect();
        yum::YumConfiguration { pkg }
    }
}

impl Into<zypper::ZypperConfiguration> for InstallConfiguration {
    fn into(self) -> zypper::ZypperConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.zypper {
                Some(zypper) => (name.clone(), zypper),
                None => (name.clone(), zypper::Package { name, ..pkg.into() }),
            })
            .collect();
        zypper::ZypperConfiguration { pkg }
    }
}

impl Into<slackpkg::SlackpkgConfiguration> for InstallConfiguration {
    fn into(self) -> slackpkg::SlackpkgConfiguration {
        let pkg = self
            .pkg
            .into_iter()
            .map(|(name, pkg)| match pkg.slackpkg {
                Some(slackpkg) => (name.clone(), slackpkg),
                None => (name.clone(), slackpkg::Package { name, ..pkg.into() }),
            })
            .collect();
        slackpkg::SlackpkgConfiguration { pkg }
    }
}

impl Into<apk::Package> for Package {
    fn into(self) -> apk::Package {
        apk::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
            interactive: self.interactive,
        }
    }
}

impl Into<apt::Package> for Package {
    fn into(self) -> apt::Package {
        apt::Package {
            name: self.name,
            url: self.url,
            gpg_key: self.gpg_key,
            gpg_path: self.gpg_path,
            setup_repository: self.setup_repository,
            apt_update: self.apt_update,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
        }
    }
}

impl Into<dnf::Package> for Package {
    fn into(self) -> dnf::Package {
        dnf::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
        }
    }
}

impl Into<emerge::Package> for Package {
    fn into(self) -> emerge::Package {
        emerge::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
            ask: self.ask,
            verbose: self.verbose,
        }
    }
}

impl Into<pacman::Package> for Package {
    fn into(self) -> pacman::Package {
        pacman::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
        }
    }
}

impl Into<yum::Package> for Package {
    fn into(self) -> yum::Package {
        yum::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
        }
    }
}

impl Into<zypper::Package> for Package {
    fn into(self) -> zypper::Package {
        zypper::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
            non_interactive: self.non_interactive,
        }
    }
}

impl Into<brew::Package> for Package {
    fn into(self) -> brew::Package {
        brew::Package {
            name: self.name,
            preinstall: self.preinstall,
            postinstall: self.postinstall,
            version_check: self.version_check,
            cask: self.cask,
        }
    }
}

impl Into<slackpkg::Package> for Package {
    fn into(self) -> slackpkg::Package {
        slackpkg::Package {
            name: self.name,
            packages: self.packages,
            depends_on: self.depends_on,
            postinstall: self.postinstall,
            version_check: self.version_check,
        }
    }
}
