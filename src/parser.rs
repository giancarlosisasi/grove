use oxc_allocator::Allocator;
use oxc_ast::ast::{Argument, CallExpression, Expression};
use oxc_ast::ast::{ExportNamedDeclaration, ImportDeclaration, ImportExpression};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_span::SourceType;

struct ImportFinder {
    imports: Vec<String>,
}

impl<'a> Visit<'a> for ImportFinder {
    // static imports
    fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
        self.imports.push(it.source.value.to_string());
    }

    // for re-exports
    fn visit_export_named_declaration(&mut self, it: &ExportNamedDeclaration<'a>) {
        if let Some(source) = &it.source {
            self.imports.push(source.value.to_string());
        }
    }

    // dynamic imports import("./foo")
    fn visit_import_expression(&mut self, it: &ImportExpression<'a>) {
        if let Expression::StringLiteral(lit) = &it.source {
            self.imports.push(lit.value.to_string());
        }
    }

    // commonjs require("./page") type import
    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        if let Expression::Identifier(ident) = &it.callee {
            if ident.name == "require" {
                if let Some(lit) = it.arguments.first() {
                    if let Argument::StringLiteral(i) = lit {
                        self.imports.push(i.value.to_string());
                    }
                }
            }
        }

        // ensure the visitor keeps walking into children of the call expression
        walk_call_expression(self, it);
    }
}

pub fn parse(source_code: &str, _filename: &str) -> Vec<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::tsx();

    let parsed = oxc_parser::Parser::new(&allocator, source_code, source_type).parse();

    let mut finder = ImportFinder {
        imports: Vec::new(),
    };

    finder.visit_program(&parsed.program);
    finder.imports

    // for stmt in &parsed.program.body {
    //     if let Statement::ImportDeclaration(import) = stmt {
    //         imports.push(import.source.value.to_string());
    //     }

    //     if let Statement::ExportNamedDeclaration(export) = stmt {
    //         if let Some(source) = &export.source {
    //             imports.push(source.value.to_string());
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_static_import() {
        let source_code = r#"import { foo } from "./bar";"#;
        let imports = parse(source_code, "test.tsx");

        assert_eq!(imports[0], "./bar");
    }

    #[test]
    fn test_parse_named_export() {
        let source_code = r#"export { helper } from "./helpers";"#;
        let imports = parse(source_code, "test.tsx");

        assert_eq!(imports[0], "./helpers");
    }

    #[test]
    fn test_parse_dynamic_import() {
        let source_code = r#"const page = import("./page")"#;
        let imports = parse(source_code, "test.tsx");

        assert_eq!(imports[0], "./page")
    }

    #[test]
    fn test_parse_commonjs_require() {
        let source_code = r#"const utils = require("./utils");"#;
        let imports = parse(source_code, "test.tsx");

        assert_eq!(imports[0], "./utils")
    }
}
