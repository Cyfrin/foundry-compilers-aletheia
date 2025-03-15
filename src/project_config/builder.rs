use crate::error::Result;
use foundry_compilers::utils;
use foundry_rs_config::Config;
use std::path::{Path, PathBuf};

use super::ProjectConfigInput;

pub struct ProjectConfigInputBuilder {
    root: PathBuf,
    sources: SourcesConfig,
    exclude: ExcludeConfig,
    include: IncludeConfig,
}

pub enum SourcesConfig {
    AutoDetect,
    Specific(PathBuf),
}

pub enum ExcludeConfig {
    None,
    Specific(Vec<String>),
}

pub enum IncludeConfig {
    All,
    Specific(Vec<String>),
}

pub const EMPTY_STRING: &str = "";

impl ProjectConfigInputBuilder {
    pub fn new(root: &Path) -> ProjectConfigInputBuilder {
        ProjectConfigInputBuilder {
            root: root.to_owned(),
            sources: SourcesConfig::AutoDetect,
            exclude: ExcludeConfig::None,
            include: IncludeConfig::All,
        }
    }
    pub fn with_sources(mut self, sources: SourcesConfig) -> ProjectConfigInputBuilder {
        self.sources = sources;
        self
    }
    pub fn with_exclude(mut self, exclude: ExcludeConfig) -> ProjectConfigInputBuilder {
        self.exclude = exclude;
        self
    }
    pub fn with_include(mut self, include: IncludeConfig) -> ProjectConfigInputBuilder {
        self.include = include;
        self
    }
    pub fn build(self) -> Result<ProjectConfigInput> {
        if !self.root.exists() {
            return Err(String::from("Non existent root").into());
        }

        let config = {
            let mut c = load_baseline_config(self.root.clone())?;
            // Sources
            if let SourcesConfig::Specific(src) = self.sources {
                c.src = src;
            }
            c.sanitized()
        };

        // Auto excludes
        let exclude_starting = {
            let mut e = Vec::new();

            // sometimes, lib, test, script may point to a directory inside the src
            // that's when this helps otherwise there's no point since we don't look
            // outside src to begin with.
            let mut unwanted = config
                .libs
                .iter()
                .filter(|lib| lib.starts_with(&config.src))
                .map(|p| p.clone())
                .collect::<Vec<_>>();
            e.append(&mut unwanted);

            if config.test.starts_with(&config.src) {
                e.push(config.test.clone());
            }

            if config.script.starts_with(&config.src) {
                e.push(config.script.clone());
            }

            // dependencies must also be auto excluded if it's a soldeer project
            let soldeer_lock = self.root.join("soldeer.lock");
            let deps = self.root.join("dependencies");

            if soldeer_lock.is_file() && deps.is_dir() {
                let deps_norm = utils::canonicalize(deps)?;
                if deps_norm.starts_with(&config.src) {
                    e.push(deps_norm);
                }
            }

            // For whatever reason, if node_modules is used in custom project, ignore that
            let node_modules = self.root.join("node_modules");
            if node_modules.is_dir() {
                let node_modules_norm = utils::canonicalize(node_modules)?;
                if node_modules_norm.starts_with(&config.src) {
                    e.push(node_modules_norm);
                }
            }

            e.dedup();
            e
        };

        // Excludes
        let exclude_containing = {
            match self.exclude {
                ExcludeConfig::None => Default::default(),
                ExcludeConfig::Specific(x) => x,
            }
        };

        // Includes
        let include_containing = {
            match self.include {
                IncludeConfig::All => vec![EMPTY_STRING.to_string()],
                IncludeConfig::Specific(x) => x,
            }
        };

        Ok(ProjectConfigInput {
            project_paths: config.project_paths(),
            root: self.root,
            include_containing,
            exclude_containing,
            exclude_starting,
            skip: config.skip,
        })
    }
}

fn load_baseline_config(root: PathBuf) -> Result<Config> {
    // Load config with auto detect default values
    let mut config = Config::load_with_root(&root)?;

    // If not HH/Foundry, reset config.src to self.root instead of `src`
    let hh_js = root.join("hardhat.config.js");
    let hh_ts = root.join("hardhat.config.ts");
    let foundry = root.join("foundry.toml");
    if !hh_js.exists() && !hh_ts.exists() && !foundry.exists() {
        // by default, if framework is not detected, it will be 'src'
        // we want it to be the same as root
        config.src = ".".into();
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::EMPTY_STRING;

    #[test]
    fn empty_string_is_always_a_substring() {
        let test_path = std::env::current_dir().unwrap();
        assert!(!test_path.as_os_str().is_empty());
        assert!(test_path.as_os_str().to_string_lossy().contains(EMPTY_STRING));
    }
}
