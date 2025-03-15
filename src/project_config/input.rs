use crate::Result;
use std::{collections::HashMap, path::PathBuf};

use foundry_compilers::{
    Graph, ProjectBuilder, ProjectPathsConfig,
    artifacts::{Settings, SolcInput, Sources, output_selection::OutputSelection},
    resolver::parse::SolData,
    solc::{Solc, SolcCompiler, SolcLanguage},
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
    pub fn make_asts(&self) -> Result<()> {
        let compiler_input = self.solc_input_for_ast_generation()?;

        Ok(())
    }
}

impl ProjectConfigInput {
    fn solc_input_for_ast_generation(&self) -> Result<HashMap<semver::Version, SolcInput>> {
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
                let graph = Graph::<SolData>::resolve_sources(&self.project_paths, sources)?;

                let project = ProjectBuilder::<SolcCompiler>::default()
                    .paths(self.project_paths.clone()) // cheap enough to clone (doesn't contain content)
                    .build(Default::default())?;

                // remove instead of get to avoid cloning
                let versioned_sources = graph
                    .into_sources_by_version(&project)?
                    .sources
                    .remove(&SolcLanguage::Solidity);

                if let Some(versioned_sources) = versioned_sources {
                    for (v, _, _) in &versioned_sources {
                        // ensure solc is installed
                        let _ = Solc::find_or_install(&v)?;
                    }
                    let mut h = HashMap::new();
                    for (v, s, _) in versioned_sources {
                        h.insert(v, create_standard_json_for_ast(s));
                    }
                    return Ok(h);
                }

                // expect no *.sol files
                return Ok(Default::default());
            }
            SolcCompilerInput::Specific(solc) => {
                let versioned_sources = HashMap::from_iter(vec![(
                    solc.version.clone(),
                    create_standard_json_for_ast(sources),
                )]);
                return Ok(versioned_sources);
            }
        }
    }
}
