use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ZigConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ZigConfig<'a> {
    fn default() -> Self {
        ZigConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "↯ ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["zig"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
