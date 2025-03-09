use foundry_rs_config::Config;
use pretty_assertions::assert_eq;
use std::{ffi::OsStr, path::Path};

/// Module created with plain `forge init` and nothing more .
mod foundry_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-basic";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 1);
        assert!(raw_config.soldeer.is_none());
    }
}

mod foundry_soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 2);
    }
}

mod soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src")); // INCORRECT IDENTIFICATION
        assert_eq!(raw_config.remappings.len(), 1); // reads from remappings.txt
    }
}

mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 2);
    }
}

mod foundry_soldeer_dep_noremap {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep-noremap";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 6);
    }
}

mod hardhat_basic {
    use super::{assert_eq, *};

    const ROOT: &'static str = "test-configs/hardhat-basic";

    #[test]
    fn identifies_remappings_correctly() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("contracts"));
        assert_eq!(raw_config.remappings.len(), 2);
    }
}
