use serde::Serialize;

use super::expr::AstExpr;
use super::offset::Range;

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum MathTextKind {
    Character { value: char },
    Number { value: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Unit {
    Pt,
    Mm,
    Cm,
    In,
    Rad,
    Deg,
    Em,
    Fr,
    Percent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UnOp {
    Pos,
    Neg,
    Not,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
    Assign,
    In,
    NotIn,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ArrayItem {
    Pos {
        expr: AstExpr,
    },
    Spread {
        expr: AstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DictItem {
    Named {
        name: String,
        expr: AstExpr,
    },
    Keyed {
        key: AstExpr,
        expr: AstExpr,
    },
    Spread {
        expr: AstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Arg {
    Pos {
        expr: AstExpr,
    },
    Named {
        name: String,
        expr: AstExpr,
    },
    Spread {
        expr: AstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Param {
    Pos {
        pattern: Pattern,
    },
    Named {
        name: String,
        expr: AstExpr,
    },
    Spread {
        sink_ident: Option<String>,
        sink_expr: Option<AstExpr>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Pattern {
    Normal {
        expr: Box<AstExpr>,
    },
    Placeholder {
        range: Range,
    },
    Parenthesized {
        expr: Box<AstExpr>,
    },
    Destructuring {
        range: Range,
        items: Vec<DestructuringItem>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DestructuringItem {
    Pattern { pattern: Pattern },
    Named { name: String, pattern: Pattern },
    Spread { sink_ident: Option<String> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum LetBindingKind {
    Normal { pattern: Pattern },
    Closure { name: String },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Imports {
    Wildcard,
    Items { items: Vec<ImportItem> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ImportItem {
    Simple {
        path: Vec<String>,
        name: String,
    },
    Renamed {
        path: Vec<String>,
        original_name: String,
        new_name: String,
    },
}
