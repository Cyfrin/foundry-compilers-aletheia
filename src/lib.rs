//! Foundry Compilers Aletheia
//!
//! Easy to access utility functions for interacting with foundry-compilers

use foundry_compilers::ProjectPathsConfig;
use foundry_rs_config::Config;

pub fn compile_ast() {}

pub fn get_config() {
    println!("{:#?}", Config::load());
    let _ = ProjectPathsConfig::builder();
}
