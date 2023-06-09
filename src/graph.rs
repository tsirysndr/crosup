use crate::{
    installers::{
        apt::AptInstaller, brew::BrewInstaller, curl::CurlInstaller, dnf::DnfInstaller,
        git::GitInstaller, nix::NixInstaller, yum::YumInstaller, zypper::ZypperInstaller,
        Installer,
    },
    macros::{add_vertex, add_vertex_with_condition},
    types::{
        configuration::Configuration,
        curl::{default_brew_installer, default_nix_installer},
    },
};
use anyhow::Error;

#[derive(Debug, Clone)]
pub struct Vertex {
    name: String,
    dependencies: Vec<String>,
    provider: String,
    apt: Option<AptInstaller>,
    brew: Option<BrewInstaller>,
    curl: Option<CurlInstaller>,
    git: Option<GitInstaller>,
    nix: Option<NixInstaller>,
    yum: Option<YumInstaller>,
    dnf: Option<DnfInstaller>,
    zypper: Option<ZypperInstaller>,
}

impl From<Box<dyn Installer + 'static>> for Vertex {
    fn from(installer: Box<dyn Installer + 'static>) -> Self {
        Self {
            name: installer.name().to_string(),
            dependencies: installer
                .dependencies()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            provider: installer.provider().to_string(),
            apt: match installer.provider() {
                "apt" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<AptInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            brew: match installer.provider() {
                "brew" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<BrewInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            curl: match installer.provider() {
                "curl" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<CurlInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            git: match installer.provider() {
                "git" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<GitInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            nix: match installer.provider() {
                "nix" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<NixInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            yum: match installer.provider() {
                "yum" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<YumInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            dnf: match installer.provider() {
                "dnf" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<DnfInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
            zypper: match installer.provider() {
                "zypper" => Some(
                    installer
                        .as_any()
                        .downcast_ref::<ZypperInstaller>()
                        .map(|x| x.clone())
                        .unwrap(),
                ),
                _ => None,
            },
        }
    }
}

impl Into<Box<dyn Installer>> for Vertex {
    fn into(self) -> Box<dyn Installer> {
        match self.provider.as_str() {
            "apt" => Box::new(self.apt.unwrap()),
            "brew" => Box::new(self.brew.unwrap()),
            "curl" => Box::new(self.curl.unwrap()),
            "git" => Box::new(self.git.unwrap()),
            "nix" => Box::new(self.nix.unwrap()),
            "yum" => Box::new(self.yum.unwrap()),
            "dnf" => Box::new(self.dnf.unwrap()),
            "zypper" => Box::new(self.zypper.unwrap()),
            _ => panic!("Unknown installer: {}", self.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
pub struct InstallerGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Into<Vec<Box<dyn Installer>>> for InstallerGraph {
    fn into(self) -> Vec<Box<dyn Installer>> {
        self.vertices.into_iter().map(|x| x.into()).collect()
    }
}

pub fn build_installer_graph(config: &Configuration) -> (InstallerGraph, Vec<Box<dyn Installer>>) {
    let mut graph = InstallerGraph::new();

    if config.clone().nix.is_some() {
        if let Some(curl) = config.clone().curl {
            if !curl.into_iter().any(|(_, y)| y.script.contains_key("nix")) {
                let nix = default_nix_installer();
                graph.add_vertex(Vertex::from(Box::new(CurlInstaller {
                    name: nix.name.clone(),
                    ..CurlInstaller::from(nix.clone())
                }) as Box<dyn Installer>));
            }
        }
    }

    if config.clone().brew.is_some() {
        if let Some(curl) = config.clone().curl {
            if !curl.into_iter().any(|(_, y)| y.script.contains_key("brew")) {
                let brew = default_brew_installer();
                graph.add_vertex(Vertex::from(Box::new(CurlInstaller {
                    name: brew.name.clone(),
                    ..CurlInstaller::from(brew.clone())
                }) as Box<dyn Installer>));
            }
        }
    }

    add_vertex!(graph, AptInstaller, config, apt, pkg);
    add_vertex!(graph, CurlInstaller, config, curl, script);
    add_vertex!(graph, GitInstaller, config, git, repo);
    add_vertex!(graph, NixInstaller, config, nix, pkg);
    add_vertex!(graph, YumInstaller, config, yum, pkg);
    add_vertex!(graph, DnfInstaller, config, dnf, pkg);
    add_vertex!(graph, ZypperInstaller, config, zypper, pkg);
    add_vertex_with_condition!(graph, BrewInstaller, config, brew, pkg);

    setup_dependencies(&mut graph);

    (graph.clone(), graph.into())
}

fn setup_dependencies(graph: &mut InstallerGraph) {
    let mut edges = vec![];

    for (i, vertex) in graph.vertices.iter().enumerate() {
        for dependency in vertex.dependencies.iter() {
            if let Some(j) = graph.vertices.iter().position(|x| x.name == *dependency) {
                edges.push(Edge { from: i, to: j });
            }
        }
    }

    graph.edges = edges;
}

impl InstallerGraph {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            edges: vec![],
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }

    pub fn contains(&self, name: &str) -> bool {
        self.vertices.iter().any(|x| x.name == name)
    }

    pub fn install_all(&self) -> Result<(), Error> {
        let mut visited = vec![false; self.vertices.len()];

        for (index, vertex) in self.vertices.iter().enumerate() {
            if !visited[index] {
                self.install(vertex.clone().into(), &mut visited)?;
            }
        }

        Ok(())
    }

    pub fn install(
        &self,
        package: Box<dyn Installer>,
        visited: &mut Vec<bool>,
    ) -> Result<(), Error> {
        let index = self
            .vertices
            .iter()
            .position(|x| x.name == package.name())
            .unwrap();

        if visited[index] {
            return Ok(());
        }

        for edge in self.edges.iter().filter(|x| x.from == index) {
            self.install(self.vertices[edge.to].clone().into(), visited)?;
        }

        package.install()?;

        visited[index] = true;

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }
}
