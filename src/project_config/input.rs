use crate::{Result, SolcCompilerOutput};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use foundry_compilers::{
    Graph, ProjectBuilder, ProjectPathsConfig,
    artifacts::{
        EvmVersion, Settings, SolcInput, Sources, StandardJsonCompilerInput,
        output_selection::OutputSelection,
    },
    resolver::parse::SolData,
    solc::{Solc, SolcCompiler, SolcLanguage},
};
use foundry_rs_config::filter::GlobMatcher;
use semver::Version;

use super::VersionedAstOutputs;

#[derive(Debug)]
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
    pub solc_compiler: SolcCompilerConfigInput,

    /// Evm Version
    pub evm_version: EvmVersion,
}

#[derive(Debug)]
pub enum SolcCompilerConfigInput {
    /// Use graphs to resolve versions and group files
    AutoDetect,

    /// Use specific version
    Specific(Solc),
}

impl ProjectConfigInput {
    /// Generate input material for solc compiler to retrieve ASTs
    pub fn solc_input_for_ast_generation(&self) -> Result<HashMap<semver::Version, SolcInput>> {
        let create_standard_json_for_ast = |sources: Sources, version: &Version| -> SolcInput {
            let mut settings = Settings::new(OutputSelection::ast_output_selection());
            settings.remappings = self.project_paths.remappings.clone();
            settings.sanitize(version, SolcLanguage::Solidity);
            SolcInput::new(SolcLanguage::Solidity, sources, settings)
        };

        // NOTE: Takes too much time in case of soldeer_basic as it goes through dependencies
        let sources = self.project_paths.read_sources()?;

        match &self.solc_compiler {
            SolcCompilerConfigInput::AutoDetect => {
                let project = ProjectBuilder::<SolcCompiler>::default()
                    .paths(self.project_paths.clone()) // cheap enough to clone (doesn't contain content)
                    .build(Default::default())?;

                let solidity_sources = {
                    let graph = Graph::<SolData>::resolve_sources(&self.project_paths, sources)?;
                    graph.into_sources_by_version(&project)?.sources.remove(&SolcLanguage::Solidity)
                };

                let Some(versioned_sources) = solidity_sources else {
                    return Ok(Default::default()); // expect no *.sol files
                };

                // ensure all required solc versions are nstalled
                for (v, _, _) in &versioned_sources {
                    Solc::find_or_install(v)?;
                }

                Ok(HashMap::from_iter(
                    versioned_sources
                        .into_iter()
                        .map(|(v, s, _)| (v.clone(), create_standard_json_for_ast(s, &v))),
                ))
            }

            SolcCompilerConfigInput::Specific(solc) => {
                let versioned_sources = HashMap::from_iter(vec![(
                    solc.version.clone(),
                    create_standard_json_for_ast(sources, &solc.version),
                )]);
                Ok(versioned_sources)
            }
        }
    }

    pub fn standard_json_for_ast_generation(
        &self,
    ) -> Result<HashMap<semver::Version, StandardJsonCompilerInput>> {
        let solc_input = self.solc_input_for_ast_generation()?;
        Ok(solc_input.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    pub fn make_asts(&self) -> Result<Vec<VersionedAstOutputs>> {
        let mut asts: Vec<VersionedAstOutputs> = Default::default();

        for (version, solc_input) in self.solc_input_for_ast_generation()? {
            // Grab the relevant solc compiler
            let solc = Solc::find_or_install(&version)?;

            // Retrieve the ASTs
            let output = solc.compile_output(&solc_input)?;
            let str_output = std::str::from_utf8(&output)?;
            let compiler_output: SolcCompilerOutput = serde_json::from_str(str_output)?;

            // Tag is_included
            let is_included = compiler_output
                .sources
                .keys()
                .map(|k| (k.to_owned(), self.is_included(k)))
                .collect();

            // Versioned Ast
            let outputs =
                VersionedAstOutputs { version: version.clone(), compiler_output, is_included };

            // Store the ASTs
            asts.push(outputs);
        }
        Ok(asts)
    }

    /// Returns if a file should be included
    fn is_included(&self, path: &Path) -> bool {
        // Auto exclude
        for x in &self.exclude_starting {
            if path.starts_with(x) {
                return false;
            }
        }

        // skip (value in foundry.toml)
        if self.skip.iter().any(|skipper| skipper.is_match(path)) {
            return false;
        }

        let path_str = path.as_os_str().to_string_lossy();

        // Exclude containing
        if self.exclude_containing.iter().any(|exclude_string| path_str.contains(exclude_string)) {
            return false;
        }

        // Include containing
        self.include_containing.iter().any(|include_string| path_str.contains(include_string))
    }
}
