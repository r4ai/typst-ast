use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use typst_syntax::{LinkedNode, SyntaxNode};

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &str = include_str!("types.ts");

#[wasm_bindgen(start)]
pub fn start() {}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum ParseMode {
    #[default]
    Markup,
    Code,
    Math,
}

#[derive(Deserialize, Default)]
struct ParseOptions {
    mode: Option<ParseMode>,
}

#[derive(Serialize)]
struct JError {
    message: String,
    range: [usize; 2],
}

#[derive(Serialize)]
struct JNode {
    kind: String,
    range: [usize; 2],
    text: Option<String>,
    children: Vec<JNode>,
}

fn node_to_json(node: &LinkedNode) -> JNode {
    let text = node.get().text();
    JNode {
        kind: format!("{:?}", node.get().kind()),
        range: [node.offset(), node.offset() + node.get().len()],
        text: if text.is_empty() {
            None
        } else {
            Some(text.to_string())
        },
        children: node.children().map(|c| node_to_json(&c)).collect(),
    }
}

fn collect_errors(node: &LinkedNode) -> Vec<JError> {
    if !node.get().erroneous() {
        return vec![];
    }
    if node.get().kind() == typst_syntax::SyntaxKind::Error {
        return node
            .get()
            .errors()
            .into_iter()
            .map(|e| JError {
                message: e.message.to_string(),
                range: [node.offset(), node.offset() + node.get().len()],
            })
            .collect();
    }
    node.children().flat_map(|c| collect_errors(&c)).collect()
}

#[derive(Serialize)]
struct ParseResult {
    root: JNode,
    errors: Vec<JError>,
}

fn make_result(root: SyntaxNode) -> Result<JsValue, JsValue> {
    let linked = LinkedNode::new(&root);
    let errors = collect_errors(&linked);
    let out = ParseResult {
        root: node_to_json(&linked),
        errors,
    };

    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(skip_typescript)]
pub fn parse(text: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let opts: ParseOptions = if options.is_undefined() || options.is_null() {
        ParseOptions::default()
    } else {
        serde_wasm_bindgen::from_value(options).map_err(|e| JsValue::from_str(&e.to_string()))?
    };

    let root = match opts.mode.unwrap_or_default() {
        ParseMode::Markup => typst_syntax::parse(text),
        ParseMode::Code => typst_syntax::parse_code(text),
        ParseMode::Math => typst_syntax::parse_math(text),
    };
    make_result(root)
}
