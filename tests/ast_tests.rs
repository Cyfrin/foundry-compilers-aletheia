mod common {
    use std::path::Path;

    use foundry_compilers_aletheia::{
        DerivedAstEvmInfo, ProjectConfigInputBuilder, Result, derive_ast_and_evm_info,
    };

    pub fn ast_info(root: &str) -> Result<DerivedAstEvmInfo> {
        let config = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
        derive_ast_and_evm_info(&config)
    }
}

mod foundry_basic {
    use pretty_assertions::assert_eq;

    use crate::common::ast_info;

    const ROOT: &str = "test-configs/foundry-basic";

    #[test]
    fn ast_info_function() {
        let ast_info = ast_info(ROOT).unwrap();
        assert_eq!(ast_info.versioned_asts.len(), 1);
        let ast_group = ast_info.versioned_asts.first().expect("no group found");
        let sources = &ast_group.compiler_output.sources;
        assert!(!sources.is_empty());
        for ast_file in sources.values() {
            assert!(ast_file.ast.as_ref().is_some_and(|s| !s.is_empty()));
        }
    }
}
