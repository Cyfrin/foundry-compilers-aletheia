# Foundry Compilers Aletheia

## Overview

Expose AST generation interface for common project strucutures.

```rust
    use foundry_compilers::artifacts::SolcInput;
    use foundry_compilers_aletheia::{ProjectConfigInputBuilder, Result};

    pub fn get_compiler_input(root: &str) -> Result<HashMap<semver::Version, SolcInput>> {
        let config_input = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
        let compiler_input = config_input.solc_input_for_ast_generation()?;
        Ok(compiler_input)
    }
```

## Attribution

### Uses [`foundry-compilers`](https://github.com/foundry-rs/compilers) 
### Vendored [`foundry-config-rs`](https://github.com/foundry-rs/foundry) 

