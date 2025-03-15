use crate::Result;
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use foundry_compilers::{
    Graph, ProjectBuilder, ProjectPathsConfig,
    artifacts::{
        Settings, SolcInput, Sources, StandardJsonCompilerInput, output_selection::OutputSelection,
    },
    resolver::parse::SolData,
    solc::{Solc, SolcCompiler, SolcLanguage},
};
use foundry_rs_config::filter::GlobMatcher;
use serde::Deserialize;

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

///// Output type `solc` produces
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct SolcCompilerOutput {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<foundry_compilers::artifacts::Error>,
    #[serde(default)]
    pub sources: BTreeMap<PathBuf, AstContent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct AstContent {
    pub id: u32,
    #[serde(deserialize_with = "raw_map_string::deserialize")]
    pub ast: String,
}

mod raw_map_string {
    use serde::{Deserialize, Deserializer};
    use serde_json::{self, Value};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;
        serde_json::to_string(&value).map_err(serde::de::Error::custom)
    }
}

impl ProjectConfigInput {
    pub fn solc_input_for_ast_generation(&self) -> Result<HashMap<semver::Version, SolcInput>> {
        let create_standard_json_for_ast = |sources: Sources| -> SolcInput {
            SolcInput::new(
                SolcLanguage::Solidity,
                sources,
                Settings::new(OutputSelection::ast_output_selection()),
            )
        };

        let sources = self.project_paths.read_sources()?;

        match &self.solc_compiler {
            SolcCompilerInput::AutoDetect => {
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
                        .map(|(v, s, _)| (v, create_standard_json_for_ast(s))),
                ))
            }

            SolcCompilerInput::Specific(solc) => {
                let versioned_sources = HashMap::from_iter(vec![(
                    solc.version.clone(),
                    create_standard_json_for_ast(sources),
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

    pub fn make_asts(&self) -> Result<()> {
        let solc_input_map = self.solc_input_for_ast_generation()?;
        for (version, solc_input) in solc_input_map {
            let solc = Solc::find_or_install(&version)?;

            let output = solc.compile_output(&solc_input)?;
            let str_output = std::str::from_utf8(&output)?;

            let compiler_output: SolcCompilerOutput = serde_json::from_str(str_output)?;

            println!("Version: {:#?}\nCompiled Output: {:#?}\n\n", version, compiler_output);
        }
        Ok(())
    }
}
