use foundry_rs_config::Config;

/// Module created with plain `forge init` and nothing more .
mod foundry_basic {
    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::Path};

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-basic";

    #[test]
    fn loads_config_currently() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 1);
        assert!(raw_config.soldeer.is_none());
    }
}

/*

    The default conf looks like this. Since remappings.txt is generated automatically by default, it appears
    as though, using [`Config::load_with_root`] directly reads from it thereby -
    Renders the `.soldeer` property on the config object to `None` (unless config is explicitly set)

    # Ref: https://github.com/mario-eth/soldeer/blob/main/USAGE.md
    ----------------------------------------------------------------------------------------------------------------

    [soldeer]
    # whether Soldeer manages remappings
    remappings_generate = true

    # whether Soldeer re-generates all remappings when installing, updating or uninstalling deps
    remappings_regenerate = false

    # whether to suffix the remapping with the version requirement string: `name-a.b.c`
    remappings_version = true

    # a prefix to add to the remappings ("@" would give `@name`)
    remappings_prefix = ""

    # where to store the remappings ("txt" for `remappings.txt` or "config" for `foundry.toml`)
    # ignored when `soldeer.toml` is used as config (uses `remappings.txt`)
    remappings_location = "txt"

    # whether to install sub-dependencies or not. If true this will install the dependencies of dependencies recursively.
    recursive_deps = false

    ----------------------------------------------------------------------------------------------------------------

*/

mod foundry_soldeer_basic {

    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::Path};

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn loads_config_currently() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 2);
        assert!(raw_config.soldeer.is_none()); // no explicit config
    }
}

mod soldeer_basic {

    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::Path};

    use super::*;

    const ROOT: &'static str = "test-configs/soldeer-basic";

    #[test]
    fn loads_config_currently() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 1); // reads from remappings.txt
        assert!(raw_config.soldeer.is_none()); // no explicit config
        assert!(raw_config.dependencies.is_none()); // DOES NOT READ soldeer.toml
    }
}

mod foundry_soldeer_dep {

    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::Path};

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep";

    #[test]
    fn loads_config_currently() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 2);
        assert!(raw_config.soldeer.is_none());
        assert!(raw_config.dependencies.is_some()); // Reads from foundry.toml
    }
}

mod foundry_soldeer_dep_noremap {

    use pretty_assertions::assert_eq;
    use std::{ffi::OsStr, path::Path};

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-soldeer-dep-noremap";

    #[test]
    fn loads_config_currently() {
        assert!(Path::new(ROOT).exists());
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 6);
        assert!(raw_config.soldeer.is_some()); // Explicitly ask soldeer not to manage remapings
        assert!(raw_config.dependencies.is_some()); // Reads from foundry.toml
    }
}
