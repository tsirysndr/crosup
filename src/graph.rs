use crate::installers::{
    self, atuin::AtuinInstaller, blesh::BleshInstaller, devbox::DevboxInstaller,
    docker::DockerInstaller, fish::FishInstaller, flox::FloxInstaller, homebrew::HomebrewInstaller,
    nix::NixInstaller, tig::TigInstaller, vscode::VSCodeInstaller, Installer,
};
use anyhow::Error;

#[derive(Clone)]
pub struct Vertex {
    name: String,
    dependencies: Vec<String>,
    default: bool,
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
            default: installer.is_default(),
        }
    }
}

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
            "flox" => Box::new(FloxInstaller::default()),
            _ => panic!("Unknown installer: {}", self.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    from: usize,
    to: usize,
}

pub struct InstallerGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

pub fn build_installer_graph() -> (InstallerGraph, Vec<Box<dyn Installer>>) {
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
    let flox = graph.add_vertex(Vertex::from(
        Box::new(FloxInstaller::default()) as Box<dyn Installer>
    ));

    graph.add_edge(fish, homebrew);
    graph.add_edge(tig, homebrew);
    graph.add_edge(devbox, nix);
    graph.add_edge(flox, nix);

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
        Box::new(FloxInstaller::default()) as Box<dyn Installer>,
    ];

    (graph, installers)
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

    pub fn install_all(&self) -> Result<(), Error> {
        let mut visited = vec![false; self.vertices.len()];

        for (index, vertex) in self.vertices.iter().enumerate() {
            if !visited[index] && vertex.default {
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
