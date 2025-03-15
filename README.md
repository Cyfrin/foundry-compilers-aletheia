# Foundry Compilers Aletheia

## Overview

To simplify installation, add the following to `Cargo.toml`

```toml
[dependencies]
foundry-compilers-aletheia = { git = "https://github.com/Cyfrin/foundry-compilers-aletheia", branch = "main", package = "foundry-compilers-aletheia" }

```

Expose AST generation interface for common project strucutures.

```rust
use foundry_compilers::artifacts::StandardJsonCompilerInput;
use foundry_compilers_aletheia::{ProjectConfigInputBuilder, Result};

pub fn get_compiler_input(root: &str) -> Result<HashMap<semver::Version, StandardJsonCompilerInput>> {
    let config_input = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
    let compiler_input = config_input.standard_json_for_ast_generation()?;
    Ok(compiler_input)
}
```

## Attribution

#### [`foundry-compilers`](https://github.com/foundry-rs/compilers) 
#### [`foundry-config-rs`](https://github.com/foundry-rs/foundry) 

