use foundry_rs_config::Config;

const ROOT: &'static str = "test-configs/foundry-basic";

fn main() {
    let raw_config = Config::load_with_root(ROOT).unwrap(); /*.sanitized()*/
    println!("{:#?}", raw_config.remappings);
    println!("{:#?}", raw_config.get_all_remappings().collect::<Vec<_>>());
    println!("{:#?}", raw_config.project());
    println!("{:#?}", raw_config.solc_compiler());
    println!("{:#?}", raw_config.solc_settings());
    println!("{:#?}", raw_config.libraries_with_remappings());
    println!("{:#?}", raw_config.ephemeral_project().unwrap());
}
