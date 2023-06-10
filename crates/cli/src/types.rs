pub struct InstallArgs {
    pub tool: Option<String>,
    pub ask: bool,
    pub remote_is_present: bool,
    pub remote: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub inventory: Option<String>,
}
