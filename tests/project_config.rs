//! Test the dependencies and challenge assumptions.
//! Allows us to know how we can use these in library

use pretty_assertions::assert_eq;

#[allow(unused_imports)]
mod common {
    use std::collections::HashMap;

    use super::{assert_eq, *};
    use foundry_compilers::artifacts::Sources;

    pub fn get_group(root: &str) -> HashMap<semver::Version, Sources> {
        Default::default()
    }
}

#[allow(unused_imports)]
mod foundry_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-basic";
}

#[allow(unused_imports)]
mod foundry_soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-basic";
}

#[allow(unused_imports)]
mod soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/soldeer-basic";
}

#[allow(unused_imports)]
mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep";
}

#[allow(unused_imports)]
mod foundry_soldeer_dep_noremap {

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep-noremap";
}

#[allow(unused_imports)]
mod hardhat_basic {

    use foundry_compilers::{
        Graph, Project, ProjectBuilder, ProjectCompileOutput,
        project::{self, ProjectCompiler},
        resolver::parse::SolData,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/hardhat-basic";
}

#[allow(unused_imports)]
mod foundry_fix_version {

    use foundry_compilers::{
        Graph, Project, ProjectBuilder, ProjectCompileOutput,
        project::{self, ProjectCompiler},
        resolver::parse::SolData,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };
    use semver::Version;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-fix-version";
}
