use std::path::PathBuf;

use foundry_compilers::{
    ProjectPathsConfig,
    solc::{Solc, SolcLanguage},
};
use foundry_rs_config::filter::GlobMatcher;

pub struct ProjectConfigInput {
    /// Root directory must contain hardhat.config.ts/.js or foundry.toml or (it's FOUNDRY_
    /// equivalet name)
    pub root: PathBuf,

    /// Project Paths representation (contains sources)
    pub project_paths: ProjectPathsConfig<SolcLanguage>,

    /// Paths (sources) containing these strings will be included
    pub include_containing: Vec<String>,

    /// Paths (sources) containing these strings will be excluded
    pub exclude_containing: Vec<String>,

    /// List of Absolute Paths (nested in sources) that will be excluded
    pub exclude_starting: Vec<PathBuf>,

    /// Paths (sources) matching with these will be excluded
    pub skip: Vec<GlobMatcher>,

    /// Solc Version to use for compiling
    pub solc_compiler: SolcCompilerInput,
}

pub enum SolcCompilerInput {
    AutoDetect,
    Specific(Solc),
}

impl ProjectConfigInput {
    pub fn make_asts() {}
}
