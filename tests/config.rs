use foundry_rs_config::Config;
use pretty_assertions::assert_eq;
use std::{ffi::OsStr, path::Path};

mod common {
    use foundry_compilers::{Graph, ProjectBuilder, resolver::parse::SolData, solc::SolcCompiler};

    use super::*;

    pub fn get_raw_config(root: &str) -> Config {
        assert!(Path::new(root).exists());
        Config::load_with_root(root).unwrap() /*.sanitized()*/
    }

    pub fn can_be_grouped(root: &str) {
        let raw_config = common::get_raw_config(root).sanitized();
        let project_paths = raw_config.project_paths();

        let builder = ProjectBuilder::<SolcCompiler>::default();
        let p = builder.paths(project_paths.clone()).build(Default::default()).unwrap();

        let graph = Graph::<SolData>::resolve_sources(
            &project_paths,
            project_paths.read_sources().unwrap(),
        )
        .unwrap();

        assert!(graph.into_sources_by_version(&p).is_ok());
    }

    // NOTE:
    // * [`Config::project_paths::<Solc>()`] has a bug which is that the `sources` field
    // on [`ProjectPaths`] although correctly identified in `Config`, get canonicalized
    // incorrectly when the value is `src`. Otherwise, canonicalization won't happen (as desired)
    // Above observation has been made on non-sanitized config.
    // DO NOT TRUST .sources if .sanitized() IS NOT CALLED
    // TRUST ONLY IF .sanitized() IS CALLED ON `Config`
    // Update: Maybe this bug is only while displaying. Testing seems to work
}

/// Module created with plain `forge init` and nothing more .
mod foundry_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 1);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}

mod foundry_soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}

mod soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 1); // reads from remappings.txt
    }

    #[test]
    //WARN: INCORRECT IDENTIFICATION OF SOURCE
    fn identifies_source_incorrectly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}

mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}

mod foundry_soldeer_dep_noremap {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep-noremap";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 6);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
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

    const ROOT: &'static str = "test-configs/hardhat-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("contracts"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}
