//! Test the dependencies and challenge assumptions.
//! Allows us to know how we can use these in library

use pretty_assertions::assert_eq;

#[allow(unused_imports)]
mod common {
    use std::{collections::HashMap, path::Path};

    use super::{assert_eq, *};
    use foundry_compilers::artifacts::SolcInput;
    use foundry_compilers_aletheia::{ProjectConfigInputBuilder, Result};

    pub fn get_compiler_input(root: &str) -> Result<HashMap<semver::Version, SolcInput>> {
        let config_input = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
        let ast_input = config_input.solc_input_for_ast_generation()?;
        Ok(ast_input)
    }
}

#[allow(unused_imports)]
mod foundry_basic {
    use foundry_compilers::solc::Solc;

    use crate::common::get_compiler_input;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_basic {
    use super::{assert_eq, *};

    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod soldeer_basic {
    use super::{assert_eq, *};

    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/soldeer-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep";

    use crate::common::get_compiler_input;

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_dep_noremap {

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep-noremap";
    use crate::common::get_compiler_input;

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}

#[allow(unused_imports)]
mod hardhat_basic {

    use super::{assert_eq, *};
    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/hardhat-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 2);
    }
}

#[allow(unused_imports)]
mod foundry_fix_version {

    use semver::Version;

    use crate::common::get_compiler_input;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-fix-version";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.len(), 1); // 1 version group
        let version = c.keys().next().expect("no version group found");
        assert_eq!(version.major, 0); // as declared in foundry.toml
        assert_eq!(version.minor, 8);
        assert_eq!(version.patch, 15);
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}
