use foundry_rs_config::Config;
use pretty_assertions::assert_eq;
use std::{ffi::OsStr, path::Path};

mod common {
    use super::*;

    pub fn get_raw_config(root: &str) -> Config {
        assert!(Path::new(root).exists());
        Config::load_with_root(root).unwrap() /*.sanitized()*/
    }
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
}

mod hardhat_basic {
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
}
