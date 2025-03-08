use std::path::PathBuf;

use foundry_rs_config::Config;

const ROOT: &'static str = "test-configs/soldeer-basic";

fn main() {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push(ROOT);
    assert!(p.exists());

    let raw_config = Config::load_with_root(p.as_os_str().to_str().unwrap()).unwrap(); /*.sanitized()*/
    println!("{:#?}", raw_config.remappings);
    println!("{:#?}", raw_config.get_all_remappings().collect::<Vec<_>>());
    println!("{:#?}", raw_config.project());
    println!("{:#?}", raw_config.solc_compiler());
    println!("{:#?}", raw_config.solc_settings());
    println!("{:#?}", raw_config.libraries_with_remappings());
    println!("{:#?}", raw_config.ephemeral_project().unwrap());
    println!("{:#?}", raw_config.soldeer);
    println!("{:#?}", raw_config.dependencies);
}
