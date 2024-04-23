use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use knuffel;
use warg_protocol::VersionReq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub registry: String,
    pub tools: HashMap<String, Tool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    pub package: String,
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
                        package: item.package,
                        version: item.version,
                    };
                    tools.insert(item.name, tool);
                }
            }
        }
        let Some(registry) = registry else {
            bail!("Registry not defined")
        };
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
                package: "ba:wasm-tools".into(),
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
