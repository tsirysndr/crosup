use crate::{
    installers::{
        Installer, _apt::AptInstaller, _brew::BrewInstaller, _curl::CurlInstaller,
        _git::GitInstaller, _nix::NixInstaller,
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
            _ => panic!("Unknown installer: {}", self.name),
        }
    }
}

/*
impl Into<Box<dyn Installer>> for Vertex {
    fn into(self) -> Box<dyn Installer> {
        match self.name.as_str() {
            "docker" => Box::new(DockerInstaller::default()),
            "fish" => Box::new(FishInstaller::default()),
            "nix" => Box::new(NixInstaller::default()),
            "vscode" => Box::new(VSCodeInstaller::default()),
            "ble.sh" => Box::new(BleshInstaller::default()),
            "atuin" => Box::new(AtuinInstaller::default()),
            "homebrew" => Box::new(HomebrewInstaller::default()),
            "tig" => Box::new(TigInstaller::default()),
            "devbox" => Box::new(DevboxInstaller::default()),
            "fzf" => Box::new(FzfInstaller::default()),
            "httpie" => Box::new(HttpieInstaller::default()),
            "kubectl" => Box::new(KubectlInstaller::default()),
            "minikube" => Box::new(MinikubeInstaller::default()),
            "tilt" => Box::new(TiltInstaller::default()),
            "zellij" => Box::new(ZellijInstaller::default()),
            "ripgrep" => Box::new(RipGrepInstaller::default()),
            "fd" => Box::new(FdInstaller::default()),
            "exa" => Box::new(ExaInstaller::default()),
            "bat" => Box::new(BatInstaller::default()),
            "glow" => Box::new(GlowInstaller::default()),
            "devenv" => Box::new(DevenvInstaller::default()),
            "neovim" => Box::new(NeoVimInstaller::default()),
            "zoxide" => Box::new(ZoxideInstaller::default()),
            "direnv" => Box::new(DirEnvInstaller::default()),
            _ => panic!("Unknown installer: {}", self.name),
        }
    }
}
*/

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

/*
pub fn _build_installer_graph() -> (InstallerGraph, Vec<Box<dyn Installer>>) {
    let mut graph = InstallerGraph::new();
    graph.add_vertex(Vertex::from(
        Box::new(DockerInstaller::default()) as Box<dyn Installer>
    ));
    let fish = graph.add_vertex(Vertex::from(
        Box::new(FishInstaller::default()) as Box<dyn Installer>
    ));
    let homebrew = graph.add_vertex(Vertex::from(
        Box::new(HomebrewInstaller::default()) as Box<dyn Installer>
    ));
    let nix = graph.add_vertex(Vertex::from(
        Box::new(NixInstaller::default()) as Box<dyn Installer>
    ));
    graph.add_vertex(Vertex::from(
        Box::new(VSCodeInstaller::default()) as Box<dyn Installer>
    ));
    graph.add_vertex(Vertex::from(
        Box::new(BleshInstaller::default()) as Box<dyn Installer>
    ));
    graph.add_vertex(Vertex::from(
        Box::new(AtuinInstaller::default()) as Box<dyn Installer>
    ));
    let tig = graph.add_vertex(Vertex::from(
        Box::new(TigInstaller::default()) as Box<dyn Installer>
    ));
    let devbox = graph.add_vertex(Vertex::from(
        Box::new(DevboxInstaller::default()) as Box<dyn Installer>
    ));
    let fzf = graph.add_vertex(Vertex::from(
        Box::new(FzfInstaller::default()) as Box<dyn Installer>
    ));
    let httpie = graph.add_vertex(Vertex::from(
        Box::new(HttpieInstaller::default()) as Box<dyn Installer>
    ));
    let kubectl = graph.add_vertex(Vertex::from(
        Box::new(KubectlInstaller::default()) as Box<dyn Installer>
    ));
    let minikube = graph.add_vertex(Vertex::from(
        Box::new(MinikubeInstaller::default()) as Box<dyn Installer>
    ));
    let tilt = graph.add_vertex(Vertex::from(
        Box::new(TiltInstaller::default()) as Box<dyn Installer>
    ));
    let zellij = graph.add_vertex(Vertex::from(
        Box::new(ZellijInstaller::default()) as Box<dyn Installer>
    ));
    let ripgrep = graph.add_vertex(Vertex::from(
        Box::new(RipGrepInstaller::default()) as Box<dyn Installer>
    ));
    let fd = graph.add_vertex(Vertex::from(
        Box::new(FdInstaller::default()) as Box<dyn Installer>
    ));
    let exa = graph.add_vertex(Vertex::from(
        Box::new(ExaInstaller::default()) as Box<dyn Installer>
    ));
    let bat = graph.add_vertex(Vertex::from(
        Box::new(BatInstaller::default()) as Box<dyn Installer>
    ));
    let glow = graph.add_vertex(Vertex::from(
        Box::new(GlowInstaller::default()) as Box<dyn Installer>
    ));
    let devenv = graph.add_vertex(Vertex::from(
        Box::new(DevenvInstaller::default()) as Box<dyn Installer>
    ));
    let neovim = graph.add_vertex(Vertex::from(
        Box::new(NeoVimInstaller::default()) as Box<dyn Installer>
    ));
    let zoxide = graph.add_vertex(Vertex::from(
        Box::new(ZoxideInstaller::default()) as Box<dyn Installer>
    ));
    let direnv = graph.add_vertex(Vertex::from(
        Box::new(DirEnvInstaller::default()) as Box<dyn Installer>
    ));

    graph.add_edge(fish, homebrew);
    graph.add_edge(tig, homebrew);
    graph.add_edge(devbox, nix);
    graph.add_edge(fzf, homebrew);
    graph.add_edge(httpie, homebrew);
    graph.add_edge(kubectl, homebrew);
    graph.add_edge(minikube, homebrew);
    graph.add_edge(minikube, kubectl);
    graph.add_edge(tilt, homebrew);
    graph.add_edge(zellij, homebrew);
    graph.add_edge(ripgrep, homebrew);
    graph.add_edge(fd, homebrew);
    graph.add_edge(exa, homebrew);
    graph.add_edge(bat, homebrew);
    graph.add_edge(glow, homebrew);
    graph.add_edge(devenv, nix);
    graph.add_edge(neovim, homebrew);
    graph.add_edge(zoxide, homebrew);
    graph.add_edge(direnv, homebrew);

    let installers = vec![
        Box::new(DockerInstaller::default()) as Box<dyn Installer>,
        Box::new(FishInstaller::default()) as Box<dyn Installer>,
        Box::new(HomebrewInstaller::default()) as Box<dyn Installer>,
        Box::new(NixInstaller::default()) as Box<dyn Installer>,
        Box::new(VSCodeInstaller::default()) as Box<dyn Installer>,
        Box::new(BleshInstaller::default()) as Box<dyn Installer>,
        Box::new(AtuinInstaller::default()) as Box<dyn Installer>,
        Box::new(TigInstaller::default()) as Box<dyn Installer>,
        Box::new(DevboxInstaller::default()) as Box<dyn Installer>,
        Box::new(FzfInstaller::default()) as Box<dyn Installer>,
        Box::new(HttpieInstaller::default()) as Box<dyn Installer>,
        Box::new(KubectlInstaller::default()) as Box<dyn Installer>,
        Box::new(MinikubeInstaller::default()) as Box<dyn Installer>,
        Box::new(TiltInstaller::default()) as Box<dyn Installer>,
        Box::new(ZellijInstaller::default()) as Box<dyn Installer>,
        Box::new(RipGrepInstaller::default()) as Box<dyn Installer>,
        Box::new(FdInstaller::default()) as Box<dyn Installer>,
        Box::new(ExaInstaller::default()) as Box<dyn Installer>,
        Box::new(BatInstaller::default()) as Box<dyn Installer>,
        Box::new(GlowInstaller::default()) as Box<dyn Installer>,
        Box::new(DevenvInstaller::default()) as Box<dyn Installer>,
        Box::new(NeoVimInstaller::default()) as Box<dyn Installer>,
        Box::new(ZoxideInstaller::default()) as Box<dyn Installer>,
        Box::new(DirEnvInstaller::default()) as Box<dyn Installer>,
    ];

    (graph, installers)
}
 */

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
