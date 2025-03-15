# Foundry Compilers Aletheia

## Overview

To simplify installation, add the following to `Cargo.toml`

```toml
[dependencies]
foundry-compilers-aletheia = { git = "https://github.com/Cyfrin/foundry-compilers-aletheia", branch = "main", package = "foundry-compilers-aletheia" }

```

Extract ASTs and other Evm Info

```rust
use std::path::Path;
use foundry_compilers_aletheia::{
    DerivedAstEvmInfo, ProjectConfigInputBuilder, Result, derive_ast_and_evm_info,
};

pub fn ast_info(root: &str) -> Result<DerivedAstEvmInfo> {
    let config = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
    derive_ast_and_evm_info(&config)
}
```

## Goal

To be the fastest AST generator for [Aderyn](https://github.com/cyfrin/aderyn)

## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

<a href="https://github.com/cyfrin/foundry-compilers-aletheia/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/foundry-compilers-aletheia" />
</a>

## Attribution

#### [`foundry-compilers`](https://github.com/foundry-rs/compilers) 
#### [`foundry-config-rs`](https://github.com/foundry-rs/foundry) 

