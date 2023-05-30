use anyhow::Error;

use crate::graph::build_installer_graph;

pub fn execute_install(tool: Option<String>) -> Result<(), Error> {
    match tool {
        Some(tool) => {
            let tool = tool.replace(" ", "");
            let tools = match tool.contains(",") {
                true => tool.split(",").collect(),
                false => vec![tool.as_str()],
            };
            for tool in tools {
                let (graph, installers) = build_installer_graph();
                let tool = installers
                    .into_iter()
                    .find(|installer| installer.name() == tool)
                    .ok_or_else(|| Error::msg(format!("{} is not available yet", tool)))?;
                let mut visited = vec![false; graph.size()];
                graph.install(tool, &mut visited)?;
            }
        }
        None => {
            let (graph, _) = build_installer_graph();
            graph.install_all()?;
        }
    }

    Ok(())
}
