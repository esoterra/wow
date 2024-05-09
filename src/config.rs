use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use kdl::KdlDocument;
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
        let text = fs::read_to_string(path).context("Reading config file")?;
        Config::parse(text.as_str())
    }

    pub fn parse(text: &str) -> Result<Self> {
        let mut registry = None;
        let mut tools = HashMap::new();

        let doc: KdlDocument = text.parse().context("Parsing config file")?;

        // get registry, if defined
        if let Some(node) = doc.get("registry") {
            if let Some(entry) = node.get(0) {
                if let kdl::KdlValue::String(url) = entry.value() {
                    registry = Some(url.clone());
                }
            }
        }

        // parse tools
        for node in doc
            .nodes()
            .iter()
            .filter(|node| node.name().value() == "tool")
        {
            // TODO better error messages with source span
            let name = if let Some(entry) = node.get(0) {
                if let kdl::KdlValue::String(s) = entry.value() {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let package = if let Some(entry) = node.get("package") {
                if let kdl::KdlValue::String(s) = entry.value() {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let version = if let Some(entry) = node.get("version") {
                if let kdl::KdlValue::String(s) = entry.value() {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if name.is_none() {
                bail!("missing tool name");
            }
            if package.is_none() {
                bail!("missing tool package name");
            }
            let package = PackageName::new(package.unwrap()).context("Invalid package name")?;

            let tool = Tool { package, version };
            tools.insert(name.unwrap(), tool);
        }

        Ok(Self { registry, tools })
    }
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

        let text = r#"
        registry "wow.wa.dev"
        tool "wasm-tools" package="ba:wasm-tools" version="0.2.1"
        "#;
        let config = Config::parse(text).unwrap();

        assert_eq!(config.tools, expected_tools);
    }
}
