use foundry_compilers::artifacts::EvmVersion;
use semver::Version;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DerivedAstEvmInfo {
    /// [`VersionedAstOutputs`] grouped by the version of Solidity compiler to use.
    pub versioned_asts: HashMap<Version, Vec<VersionedAstOutputs>>,
    /// EVM version mentioned `foundry.toml` which default to Cancun
    pub evm_version: EvmVersion,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct VersionedAstOutputs {
    pub version: Version,
    pub compiler_output: SolcCompilerOutput,
    pub is_included: HashMap<PathBuf, bool>,
}

/// Output type `solc` produces
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
    pub ast: Option<String>,
}

mod raw_map_string {
    use serde::{Deserialize, Deserializer};
    use serde_json::{self, Value};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;
        if value.is_null() {
            return Ok(None);
        }
        let string = serde_json::to_string(&value).map_err(serde::de::Error::custom)?;
        Ok(Some(string))
    }
}
