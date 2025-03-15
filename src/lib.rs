//! Easily generate ASTs for Solidity Projects

mod error;
mod project_config;

pub use error::*;
pub use project_config::*;

/// Returns AST and EVM based info based on foundry/hardhat's configuration (or) custom framework.
/// Use [`ProjectConfigInputBuilder::build`] to create the `config` argument for this function.
pub fn derive_ast_and_evm_info(config: &ProjectConfigInput) -> Result<DerivedAstEvmInfo> {
    let asts = config.make_asts()?;
    let evm_version = config.evm_version;
    Ok(DerivedAstEvmInfo { versioned_asts: asts, evm_version })
}
