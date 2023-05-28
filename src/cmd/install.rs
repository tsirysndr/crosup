use anyhow::Error;

use crate::graph::build_installer_graph;

pub fn execute_install(tool: Option<String>) -> Result<(), Error> {
    let (graph, installers) = build_installer_graph();
    match tool {
        Some(tool) => {
            let tool = installers
                .into_iter()
                .find(|installer| installer.name() == tool)
                .ok_or_else(|| Error::msg(format!("{} is not available yet", tool)))?;
            let mut visited = vec![false; graph.size()];
            graph.install(tool, &mut visited)?;
        }
        None => {
            graph.install_all()?;
        }
    }

    Ok(())
}
