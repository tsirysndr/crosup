#[derive(Clone, Default)]
pub struct InstallArgs {
    pub tools: Option<Vec<String>>,
    pub ask: bool,
    pub remote_is_present: bool,
    pub remote: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub inventory: Option<String>,
    pub from: Option<String>
}

#[derive(Clone, Default)]
pub struct SearchArgs {
    pub package: String,
    pub channel: String,
    pub max_results: u32,
}
