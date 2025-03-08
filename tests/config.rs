use foundry_rs_config::Config;

/// Module created with plain `forge init` and nothing more .
mod foundry_basic {
    use pretty_assertions::assert_eq;
    use std::ffi::OsStr;

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-basic";

    #[test]
    fn loads_config_currently() {
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 1);
    }
}

mod foundry_soldeer_basic {

    use pretty_assertions::assert_eq;
    use std::ffi::OsStr;

    use super::*;

    const ROOT: &'static str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn loads_config_currently() {
        let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
        assert_eq!(raw_config.remappings.len(), 2);
    }
}
