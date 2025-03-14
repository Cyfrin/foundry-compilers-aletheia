//! Easy to access utility functions for interacting with foundry-compilers

use crate::error::Result;
use foundry_compilers::{ProjectPathsConfig, solc::SolcLanguage, utils};
use foundry_rs_config::{Config, filter::GlobMatcher};
use std::path::{Path, PathBuf};

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

pub struct ProjectConfigInput {
    /// Must have hardhat.config.ts/.js or foundry.toml or (it's FOUNDRY_
    /// equivalet name)
    pub root: PathBuf,

    /// Project Paths representation (contains sources)
    pub project_paths: ProjectPathsConfig<SolcLanguage>,

    /// Paths (sources) containing these strings will be included
    pub include_containing: Vec<String>,

    /// Paths (sources) containing these strings will be excluded
    pub exclude_containing: Vec<String>,

    /// Paths (sources) starting with these will be excluded
    /// These are absolute paths
    pub exclude_starting: Vec<PathBuf>,

    /// Paths (sources) matching with these will be excluded
    pub skip: Vec<GlobMatcher>,
}

pub struct ProjectConfigInputBuilder {
    root: PathBuf,
    sources: SourcesConfig,
    exclude: ExcludeConfig,
    include: IncludeConfig,
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

        let mut config = {
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
            e.append(&mut config.libs);
            e.push(config.test.clone());
            e.push(config.script.clone());

            // dependencies must also be auto excluded if it's a soldeer project
            let soldeer_lock = self.root.join("soldeer.lock");
            let deps = self.root.join("dependencies");

            if soldeer_lock.exists() && soldeer_lock.is_file() && deps.exists() && deps.is_dir() {
                let deps_norm = utils::canonicalize(deps)?;
                e.push(deps_norm);
            }
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

pub fn load_baseline_config(root: PathBuf) -> Result<Config> {
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
