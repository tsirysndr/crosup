macro_rules! install {
    ($args: ident, $config: ident, $session: expr) => {
        match $args.tool {
            Some(ref tool) => {
                let tool = tool.replace(" ", "");
                let tool = tool.replace("ble.sh", "blesh");
                let tools = match tool.contains(",") {
                    true => tool.split(",").collect(),
                    false => vec![tool.as_str()],
                };
                for tool in tools {
                    let (graph, installers) = build_installer_graph(&mut $config, $session.clone());
                    let tool = installers
                        .into_iter()
                        .find(|installer| installer.name() == tool)
                        .ok_or_else(|| Error::msg(format!("{} is not available yet", tool)))?;
                    let mut visited = vec![false; graph.size()];
                    graph.install(tool, &mut visited)?;
                }
            }
            None => {
                let (graph, _) = build_installer_graph(&mut $config, $session.clone());
                graph.install_all()?;
            }
        }
    };
}

pub(crate) use install;
