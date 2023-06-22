macro_rules! install {
    ($args: ident, $config: ident, $session: expr) => {
        match $args.tools {
            Some(ref tools) => {
                for tool_name in tools {
                    let tool_name = tool_name.replace("ble.sh", "blesh");
                    let mut default_config = Configuration::default();
                    let (default_graph, default_installers) =
                        build_installer_graph(&mut default_config, $session.clone());

                    let mut visited = vec![false; default_graph.size()];
                    if let Some(tool) = default_installers
                        .into_iter()
                        .find(|installer| installer.name() == tool_name)
                    {
                        default_graph.install(tool, &mut visited)?;
                        continue;
                    }

                    let (graph, installers) = build_installer_graph(&mut $config, $session.clone());
                    let tool = installers
                        .into_iter()
                        .find(|installer| installer.name() == tool_name)
                        .unwrap();
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
