use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf};

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
