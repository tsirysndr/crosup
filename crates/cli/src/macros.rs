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
                for tool_name in tools {
                    let (graph, installers) = build_installer_graph(&mut $config, $session.clone());
                    let (tool, nix) = installers
                        .into_iter()
                        .find(|installer| installer.name() == tool_name)
                        .map(|x| (x, false))
                        .unwrap_or((
                            Box::new(HomeManagerInstaller {
                                name: tool_name.to_string(),
                                dependencies: vec!["nix".into()],
                                packages: Some(vec![tool_name.to_string()]),
                                ..Default::default()
                            }) as Box<dyn Installer>,
                            true,
                        ));
                    let mut visited = vec![false; graph.size()];

                    if nix {
                        $config.packages = Some(vec![tool_name.to_string()]);
                        let (graph, _) = build_installer_graph(&mut $config, $session.clone());
                        let mut visited = vec![false; graph.size()];
                        graph.install(tool, &mut visited)?;
                        continue;
                    }
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
