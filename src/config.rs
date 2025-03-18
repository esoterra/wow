use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use warg_protocol::{registry::PackageName, VersionReq};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub registry: Option<String>,
    pub tools: HashMap<String, Tool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    pub package: PackageName,
    pub version: Option<String>,
}

impl Tool {
    pub fn version_req(&self) -> Result<VersionReq> {
        let req = match &self.version {
            Some(version) => VersionReq::parse(&format!("={}", version))?,
            None => VersionReq::STAR,
        };
        Ok(req)
    }
}

impl Config {
    pub fn parse_file(path: PathBuf) -> Result<Self> {
        let text = fs::read_to_string(&path).context("Reading config file")?;
        let file_name = path.to_string_lossy();
        Config::parse(&file_name, text.as_str())
    }

    pub fn parse(file_name: &str, text: &str) -> Result<Self> {
        let items =
            knuffel::parse::<Vec<KdlConfigItem>>(file_name, text).context("Parsing config file")?;
        let mut registry = None;
        let mut tools = HashMap::new();
        for item in items {
            match item {
                KdlConfigItem::Registry(item) => {
                    registry = Some(item.url);
                }
                KdlConfigItem::Tool(item) => {
                    let tool = Tool {
                        package: PackageName::new(item.package).context("invalid package name")?,
                        version: item.version,
                    };
                    tools.insert(item.name, tool);
                }
            }
        }
        Ok(Self { registry, tools })
    }
}

#[derive(knuffel::Decode)]
enum KdlConfigItem {
    Registry(KdlRegistry),
    Tool(KdlTool),
}

#[derive(knuffel::Decode)]
struct KdlRegistry {
    #[knuffel(argument)]
    url: String,
}

#[derive(knuffel::Decode)]
struct KdlTool {
    #[knuffel(argument)]
    name: String,
    #[knuffel(property)]
    package: String,
    #[knuffel(property)]
    version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parse() {
        let mut expected_tools = HashMap::new();
        expected_tools.insert(
            "wasm-tools".into(),
            Tool {
                package: PackageName::new("ba:wasm-tools").unwrap(),
                version: Some("0.2.1".into()),
            },
        );

        let file_name = "wow.kdl";
        let text = r#"
        registry "wow.wa.dev"
        tool "wasm-tools" package="ba:wasm-tools" version="0.2.1"
        "#;
        let config = Config::parse(file_name, text).unwrap();

        assert_eq!(config.tools, expected_tools);
    }
}
