use serde::Serialize;
use typst_syntax::LinkedNode;

#[derive(Serialize)]
pub struct ParseError {
    pub message: String,
    pub range: [usize; 2],
}

#[derive(Serialize)]
struct CstNode {
    kind: String,
    range: [usize; 2],
    text: Option<String>,
    children: Vec<CstNode>,
}

#[derive(Serialize)]
struct CstParseResult {
    root: CstNode,
    errors: Vec<ParseError>,
}

fn node_to_cst(node: &LinkedNode) -> CstNode {
    let text = node.get().text();
    CstNode {
        kind: format!("{:?}", node.get().kind()),
        range: [node.offset(), node.offset() + node.get().len()],
        text: if text.is_empty() {
            None
        } else {
            Some(text.to_string())
        },
        children: node.children().map(|c| node_to_cst(&c)).collect(),
    }
}

pub fn collect_errors(node: &LinkedNode) -> Vec<ParseError> {
    if !node.get().erroneous() {
        return vec![];
    }
    if node.get().kind() == typst_syntax::SyntaxKind::Error {
        return node
            .get()
            .errors()
            .into_iter()
            .map(|e| ParseError {
                message: e.message.to_string(),
                range: [node.offset(), node.offset() + node.get().len()],
            })
            .collect();
    }
    node.children().flat_map(|c| collect_errors(&c)).collect()
}

pub fn make_cst_result(
    root: typst_syntax::SyntaxNode,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let linked = LinkedNode::new(&root);
    let errors = collect_errors(&linked);
    let out = CstParseResult {
        root: node_to_cst(&linked),
        errors,
    };
    serde_wasm_bindgen::to_value(&out).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
