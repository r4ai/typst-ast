mod convert;
pub mod expr;
mod offset;
pub mod types;

use serde::Serialize;
use typst_syntax::ast;
use typst_syntax::LinkedNode;
use typst_syntax::SyntaxNode;

use crate::cst::{self, ParseError};
use crate::parse_mode::ParseMode;
use convert::{convert_code, convert_markup, convert_math};
use offset::{build_offset_map, OffsetMap};

pub use expr::AstExpr;

#[derive(Serialize)]
pub struct AstParseResult {
    pub root: Vec<AstExpr>,
    pub errors: Vec<ParseError>,
}

pub fn make_ast_result(root: &SyntaxNode, mode: &ParseMode) -> Result<AstParseResult, String> {
    let linked = LinkedNode::new(root);
    let mut offsets = OffsetMap::new();
    build_offset_map(&linked, &mut offsets);

    let exprs = match mode {
        ParseMode::Markup => {
            let markup: ast::Markup = root.cast().ok_or("Failed to cast root to Markup")?;
            convert_markup(markup, &offsets)
        }
        ParseMode::Code => {
            let code: ast::Code = root.cast().ok_or("Failed to cast root to Code")?;
            convert_code(code, &offsets)
        }
        ParseMode::Math => {
            let math: ast::Math = root.cast().ok_or("Failed to cast root to Math")?;
            convert_math(math, &offsets)
        }
    };

    let errors = cst::collect_errors(&linked);

    Ok(AstParseResult {
        root: exprs,
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(text: &str, mode: ParseMode) -> AstParseResult {
        let root = match mode {
            ParseMode::Markup => typst_syntax::parse(text),
            ParseMode::Code => typst_syntax::parse_code(text),
            ParseMode::Math => typst_syntax::parse_math(text),
        };

        make_ast_result(&root, &mode).expect("make_ast_result should not fail")
    }

    #[test]
    fn handles_erroneous_code_without_panicking() {
        for src in ["#show:", "#for x in", "not"] {
            let result = parse_ok(src, ParseMode::Code);
            assert!(
                !result.errors.is_empty(),
                "expected syntax errors for input: {src:?}"
            );
        }
    }

    #[test]
    fn uses_null_range_for_placeholder_nodes() {
        let result = parse_ok("#show:", ParseMode::Code);
        let show_rule = result.root.into_iter().find_map(|expr| match expr {
            AstExpr::ShowRule { transform, .. } => Some(transform),
            _ => None,
        });
        let Some(transform) = show_rule else {
            panic!("expected show rule");
        };

        match *transform {
            AstExpr::None { range } => assert!(range.is_none()),
            _ => panic!("expected placeholder none expression"),
        }
    }

    #[test]
    fn handles_erroneous_math_without_panicking() {
        for src in ["√", "x_"] {
            let result = parse_ok(src, ParseMode::Math);
            assert!(
                !result.errors.is_empty(),
                "expected syntax errors for input: {src:?}"
            );
        }
    }

    #[test]
    fn handles_erroneous_markup_without_panicking() {
        let result = parse_ok("[*", ParseMode::Markup);
        assert!(!result.errors.is_empty());
    }
}
