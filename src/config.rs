use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;
use warg_protocol::VersionReq;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Config {
    pub registry: String,
    pub tools: HashMap<String, Tool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
        let text = fs::read_to_string(path).context("Reading config file")?;
        Config::parse(text.as_str())
    }

    pub fn parse(text: &str) -> Result<Self> {
        toml::from_str::<Config>(text).context("Parsing config file")
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
                package: "ba:wasm-tools".into(),
                version: Some("0.2.1".into()),
            },
        );

        let text = r#"
        registry = "wow.wa.dev"

        [tools]
        wasm-tools = { package = "ba:wasm-tools", version = "0.2.1" }
        "#;
        let config = Config::parse(text).unwrap();

        assert_eq!(config.tools, expected_tools);
    }
}
