mod ast;
mod cst;
mod parse_mode;

use serde::Deserialize;
use wasm_bindgen::prelude::*;

use parse_mode::ParseMode;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &str = include_str!("types.ts");

#[wasm_bindgen(start)]
pub fn start() {}

#[derive(Deserialize, Default)]
struct ParseOptions {
    mode: Option<ParseMode>,
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
    cst::make_cst_result(root)
}

#[wasm_bindgen(js_name = "parseAst", skip_typescript)]
pub fn parse_ast(text: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let opts: ParseOptions = if options.is_undefined() || options.is_null() {
        ParseOptions::default()
    } else {
        serde_wasm_bindgen::from_value(options).map_err(|e| JsValue::from_str(&e.to_string()))?
    };

    let mode = opts.mode.unwrap_or_default();
    let root = match &mode {
        ParseMode::Markup => typst_syntax::parse(text),
        ParseMode::Code => typst_syntax::parse_code(text),
        ParseMode::Math => typst_syntax::parse_math(text),
    };

    let result = ast::make_ast_result(&root, &mode).map_err(|e| JsValue::from_str(&e))?;

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
