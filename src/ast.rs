use std::collections::HashMap;
use std::num::NonZeroUsize;

use serde::Serialize;
use typst_syntax::ast::{self, AstNode};
use typst_syntax::{LinkedNode, SyntaxNode};

type OffsetMap = HashMap<*const SyntaxNode, usize>;
type JRange = Option<[usize; 2]>;

fn build_offset_map(node: &LinkedNode, map: &mut OffsetMap) {
    map.insert(node.get() as *const SyntaxNode, node.offset());
    for child in node.children() {
        build_offset_map(&child, map);
    }
}

fn range_of(node: &SyntaxNode, offsets: &OffsetMap) -> JRange {
    // `typst-syntax` can return detached placeholder nodes for missing children
    // in erroneous input. Those placeholders are not part of the parsed tree,
    // so they are absent from the offset map.
    offsets
        .get(&(node as *const SyntaxNode))
        .copied()
        .map(|offset| [offset, offset + node.len()])
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstExpr {
    Text {
        range: JRange,
        text: String,
    },
    Space {
        range: JRange,
    },
    Linebreak {
        range: JRange,
    },
    Parbreak {
        range: JRange,
    },
    Escape {
        range: JRange,
        character: char,
    },
    Shorthand {
        range: JRange,
        character: char,
    },
    SmartQuote {
        range: JRange,
        double: bool,
    },
    Strong {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    Emph {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    Raw {
        range: JRange,
        lines: Vec<String>,
        lang: Option<String>,
        block: bool,
    },
    Link {
        range: JRange,
        url: String,
    },
    Label {
        range: JRange,
        name: String,
    },
    Ref {
        range: JRange,
        target: String,
        supplement: Option<Vec<JAstExpr>>,
    },
    Heading {
        range: JRange,
        depth: usize,
        body: Vec<JAstExpr>,
    },
    ListItem {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    EnumItem {
        range: JRange,
        number: Option<u64>,
        body: Vec<JAstExpr>,
    },
    TermItem {
        range: JRange,
        term: Vec<JAstExpr>,
        description: Vec<JAstExpr>,
    },
    Equation {
        range: JRange,
        body: Vec<JAstExpr>,
        block: bool,
    },

    // Math
    Math {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    MathText {
        range: JRange,
        text: JAstMathTextKind,
    },
    MathIdent {
        range: JRange,
        name: String,
    },
    MathShorthand {
        range: JRange,
        character: char,
    },
    MathAlignPoint {
        range: JRange,
    },
    MathDelimited {
        range: JRange,
        open: Box<JAstExpr>,
        body: Vec<JAstExpr>,
        close: Box<JAstExpr>,
    },
    MathAttach {
        range: JRange,
        base: Box<JAstExpr>,
        bottom: Option<Box<JAstExpr>>,
        top: Option<Box<JAstExpr>>,
        primes: Option<usize>,
    },
    MathPrimes {
        range: JRange,
        count: usize,
    },
    MathFrac {
        range: JRange,
        num: Box<JAstExpr>,
        denom: Box<JAstExpr>,
    },
    MathRoot {
        range: JRange,
        index: Option<u8>,
        radicand: Box<JAstExpr>,
    },

    // Literals
    Ident {
        range: JRange,
        name: String,
    },
    None {
        range: JRange,
    },
    Auto {
        range: JRange,
    },
    Bool {
        range: JRange,
        value: bool,
    },
    Int {
        range: JRange,
        value: i64,
    },
    Float {
        range: JRange,
        value: f64,
    },
    Numeric {
        range: JRange,
        value: f64,
        unit: JAstUnit,
    },
    Str {
        range: JRange,
        value: String,
    },

    // Code structures
    CodeBlock {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    ContentBlock {
        range: JRange,
        body: Vec<JAstExpr>,
    },
    Parenthesized {
        range: JRange,
        expr: Box<JAstExpr>,
    },
    Array {
        range: JRange,
        items: Vec<JAstArrayItem>,
    },
    Dict {
        range: JRange,
        items: Vec<JAstDictItem>,
    },

    // Operations
    Unary {
        range: JRange,
        op: JAstUnOp,
        expr: Box<JAstExpr>,
    },
    Binary {
        range: JRange,
        op: JAstBinOp,
        lhs: Box<JAstExpr>,
        rhs: Box<JAstExpr>,
    },
    FieldAccess {
        range: JRange,
        target: Box<JAstExpr>,
        field: String,
    },
    FuncCall {
        range: JRange,
        callee: Box<JAstExpr>,
        args: Vec<JAstArg>,
    },
    Closure {
        range: JRange,
        name: Option<String>,
        params: Vec<JAstParam>,
        body: Box<JAstExpr>,
    },

    // Bindings
    LetBinding {
        range: JRange,
        binding_kind: JAstLetBindingKind,
        init: Option<Box<JAstExpr>>,
    },
    DestructAssignment {
        range: JRange,
        pattern: JAstPattern,
        value: Box<JAstExpr>,
    },

    // Rules
    SetRule {
        range: JRange,
        target: Box<JAstExpr>,
        args: Vec<JAstArg>,
        condition: Option<Box<JAstExpr>>,
    },
    ShowRule {
        range: JRange,
        selector: Option<Box<JAstExpr>>,
        transform: Box<JAstExpr>,
    },
    Contextual {
        range: JRange,
        body: Box<JAstExpr>,
    },

    // Control flow
    Conditional {
        range: JRange,
        condition: Box<JAstExpr>,
        if_body: Box<JAstExpr>,
        else_body: Option<Box<JAstExpr>>,
    },
    WhileLoop {
        range: JRange,
        condition: Box<JAstExpr>,
        body: Box<JAstExpr>,
    },
    ForLoop {
        range: JRange,
        pattern: JAstPattern,
        iterable: Box<JAstExpr>,
        body: Box<JAstExpr>,
    },

    // Module
    ModuleImport {
        range: JRange,
        source: Box<JAstExpr>,
        new_name: Option<String>,
        imports: Option<JAstImports>,
    },
    ModuleInclude {
        range: JRange,
        source: Box<JAstExpr>,
    },

    // Jump
    LoopBreak {
        range: JRange,
    },
    LoopContinue {
        range: JRange,
    },
    FuncReturn {
        range: JRange,
        body: Option<Box<JAstExpr>>,
    },
}

// --- Sub-types ---

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstMathTextKind {
    Character { value: char },
    Number { value: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JAstUnit {
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
pub enum JAstUnOp {
    Pos,
    Neg,
    Not,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JAstBinOp {
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
pub enum JAstArrayItem {
    Pos {
        expr: JAstExpr,
    },
    Spread {
        expr: JAstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstDictItem {
    Named {
        name: String,
        expr: JAstExpr,
    },
    Keyed {
        key: JAstExpr,
        expr: JAstExpr,
    },
    Spread {
        expr: JAstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstArg {
    Pos {
        expr: JAstExpr,
    },
    Named {
        name: String,
        expr: JAstExpr,
    },
    Spread {
        expr: JAstExpr,
        sink_ident: Option<String>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstParam {
    Pos {
        pattern: JAstPattern,
    },
    Named {
        name: String,
        expr: JAstExpr,
    },
    Spread {
        sink_ident: Option<String>,
        sink_expr: Option<JAstExpr>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstPattern {
    Normal {
        expr: Box<JAstExpr>,
    },
    Placeholder {
        range: JRange,
    },
    Parenthesized {
        expr: Box<JAstExpr>,
    },
    Destructuring {
        range: JRange,
        items: Vec<JAstDestructuringItem>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstDestructuringItem {
    Pattern { pattern: JAstPattern },
    Named { name: String, pattern: JAstPattern },
    Spread { sink_ident: Option<String> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstLetBindingKind {
    Normal { pattern: JAstPattern },
    Closure { name: String },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstImports {
    Wildcard,
    Items { items: Vec<JAstImportItem> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JAstImportItem {
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

// --- Conversion functions ---

fn convert_markup(markup: ast::Markup, offsets: &OffsetMap) -> Vec<JAstExpr> {
    markup.exprs().map(|e| convert_expr(e, offsets)).collect()
}

fn convert_code(code: ast::Code, offsets: &OffsetMap) -> Vec<JAstExpr> {
    code.exprs().map(|e| convert_expr(e, offsets)).collect()
}

fn convert_math(math: ast::Math, offsets: &OffsetMap) -> Vec<JAstExpr> {
    math.exprs().map(|e| convert_expr(e, offsets)).collect()
}

fn convert_expr(expr: ast::Expr, offsets: &OffsetMap) -> JAstExpr {
    let node = expr.to_untyped();
    let range = range_of(node, offsets);

    match expr {
        ast::Expr::Text(v) => JAstExpr::Text {
            range,
            text: v.get().to_string(),
        },
        ast::Expr::Space(_) => JAstExpr::Space { range },
        ast::Expr::Linebreak(_) => JAstExpr::Linebreak { range },
        ast::Expr::Parbreak(_) => JAstExpr::Parbreak { range },
        ast::Expr::Escape(v) => JAstExpr::Escape {
            range,
            character: v.get(),
        },
        ast::Expr::Shorthand(v) => JAstExpr::Shorthand {
            range,
            character: v.get(),
        },
        ast::Expr::SmartQuote(v) => JAstExpr::SmartQuote {
            range,
            double: v.double(),
        },
        ast::Expr::Strong(v) => JAstExpr::Strong {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Emph(v) => JAstExpr::Emph {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Raw(v) => JAstExpr::Raw {
            range,
            lines: v.lines().map(|l| l.get().to_string()).collect(),
            lang: v.lang().map(|l| l.get().to_string()),
            block: v.block(),
        },
        ast::Expr::Link(v) => JAstExpr::Link {
            range,
            url: v.get().to_string(),
        },
        ast::Expr::Label(v) => JAstExpr::Label {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::Ref(v) => JAstExpr::Ref {
            range,
            target: v.target().to_string(),
            supplement: v.supplement().map(|s| convert_markup(s.body(), offsets)),
        },
        ast::Expr::Heading(v) => JAstExpr::Heading {
            range,
            depth: NonZeroUsize::get(v.depth()),
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::ListItem(v) => JAstExpr::ListItem {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::EnumItem(v) => JAstExpr::EnumItem {
            range,
            number: v.number(),
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::TermItem(v) => JAstExpr::TermItem {
            range,
            term: convert_markup(v.term(), offsets),
            description: convert_markup(v.description(), offsets),
        },
        ast::Expr::Equation(v) => JAstExpr::Equation {
            range,
            body: convert_math(v.body(), offsets),
            block: v.block(),
        },

        // Math
        ast::Expr::MathText(v) => JAstExpr::MathText {
            range,
            text: convert_math_text_kind(v.get()),
        },
        ast::Expr::MathIdent(v) => JAstExpr::MathIdent {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::MathShorthand(v) => JAstExpr::MathShorthand {
            range,
            character: v.get(),
        },
        ast::Expr::MathAlignPoint(_) => JAstExpr::MathAlignPoint { range },
        ast::Expr::MathDelimited(v) => JAstExpr::MathDelimited {
            range,
            open: Box::new(convert_expr(v.open(), offsets)),
            body: convert_math(v.body(), offsets),
            close: Box::new(convert_expr(v.close(), offsets)),
        },
        ast::Expr::MathAttach(v) => JAstExpr::MathAttach {
            range,
            base: Box::new(convert_expr(v.base(), offsets)),
            bottom: v.bottom().map(|e| Box::new(convert_expr(e, offsets))),
            top: v.top().map(|e| Box::new(convert_expr(e, offsets))),
            primes: v.primes().map(|p| p.count()),
        },
        ast::Expr::MathPrimes(v) => JAstExpr::MathPrimes {
            range,
            count: v.count(),
        },
        ast::Expr::MathFrac(v) => JAstExpr::MathFrac {
            range,
            num: Box::new(convert_expr(v.num(), offsets)),
            denom: Box::new(convert_expr(v.denom(), offsets)),
        },
        ast::Expr::MathRoot(v) => JAstExpr::MathRoot {
            range,
            index: v.index(),
            radicand: Box::new(convert_expr(v.radicand(), offsets)),
        },

        // Math (inline)
        ast::Expr::Math(v) => JAstExpr::Math {
            range,
            body: convert_math(v, offsets),
        },

        // Literals
        ast::Expr::Ident(v) => JAstExpr::Ident {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::None(_) => JAstExpr::None { range },
        ast::Expr::Auto(_) => JAstExpr::Auto { range },
        ast::Expr::Bool(v) => JAstExpr::Bool {
            range,
            value: v.get(),
        },
        ast::Expr::Int(v) => JAstExpr::Int {
            range,
            value: v.get(),
        },
        ast::Expr::Float(v) => JAstExpr::Float {
            range,
            value: v.get(),
        },
        ast::Expr::Numeric(v) => {
            let (value, unit) = v.get();
            JAstExpr::Numeric {
                range,
                value,
                unit: convert_unit(unit),
            }
        }
        ast::Expr::Str(v) => JAstExpr::Str {
            range,
            value: v.get().to_string(),
        },

        // Code structures
        ast::Expr::CodeBlock(v) => JAstExpr::CodeBlock {
            range,
            body: convert_code(v.body(), offsets),
        },
        ast::Expr::ContentBlock(v) => JAstExpr::ContentBlock {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Parenthesized(v) => JAstExpr::Parenthesized {
            range,
            expr: Box::new(convert_expr(v.expr(), offsets)),
        },
        ast::Expr::Array(v) => JAstExpr::Array {
            range,
            items: v.items().map(|i| convert_array_item(i, offsets)).collect(),
        },
        ast::Expr::Dict(v) => JAstExpr::Dict {
            range,
            items: v.items().map(|i| convert_dict_item(i, offsets)).collect(),
        },

        // Operations
        ast::Expr::Unary(v) => JAstExpr::Unary {
            range,
            op: convert_unop(v.op()),
            expr: Box::new(convert_expr(v.expr(), offsets)),
        },
        ast::Expr::Binary(v) => JAstExpr::Binary {
            range,
            op: convert_binop(v.op()),
            lhs: Box::new(convert_expr(v.lhs(), offsets)),
            rhs: Box::new(convert_expr(v.rhs(), offsets)),
        },
        ast::Expr::FieldAccess(v) => JAstExpr::FieldAccess {
            range,
            target: Box::new(convert_expr(v.target(), offsets)),
            field: v.field().get().to_string(),
        },
        ast::Expr::FuncCall(v) => JAstExpr::FuncCall {
            range,
            callee: Box::new(convert_expr(v.callee(), offsets)),
            args: v.args().items().map(|a| convert_arg(a, offsets)).collect(),
        },
        ast::Expr::Closure(v) => JAstExpr::Closure {
            range,
            name: v.name().map(|n| n.get().to_string()),
            params: v
                .params()
                .children()
                .map(|p| convert_param(p, offsets))
                .collect(),
            body: Box::new(convert_expr(v.body(), offsets)),
        },

        // Bindings
        ast::Expr::LetBinding(v) => JAstExpr::LetBinding {
            range,
            binding_kind: convert_let_binding_kind(v.kind(), offsets),
            init: v.init().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::DestructAssignment(v) => JAstExpr::DestructAssignment {
            range,
            pattern: convert_pattern(v.pattern(), offsets),
            value: Box::new(convert_expr(v.value(), offsets)),
        },

        // Rules
        ast::Expr::SetRule(v) => JAstExpr::SetRule {
            range,
            target: Box::new(convert_expr(v.target(), offsets)),
            args: v.args().items().map(|a| convert_arg(a, offsets)).collect(),
            condition: v.condition().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::ShowRule(v) => JAstExpr::ShowRule {
            range,
            selector: v.selector().map(|e| Box::new(convert_expr(e, offsets))),
            transform: Box::new(convert_expr(v.transform(), offsets)),
        },
        ast::Expr::Contextual(v) => JAstExpr::Contextual {
            range,
            body: Box::new(convert_expr(v.body(), offsets)),
        },

        // Control flow
        ast::Expr::Conditional(v) => JAstExpr::Conditional {
            range,
            condition: Box::new(convert_expr(v.condition(), offsets)),
            if_body: Box::new(convert_expr(v.if_body(), offsets)),
            else_body: v.else_body().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::WhileLoop(v) => JAstExpr::WhileLoop {
            range,
            condition: Box::new(convert_expr(v.condition(), offsets)),
            body: Box::new(convert_expr(v.body(), offsets)),
        },
        ast::Expr::ForLoop(v) => JAstExpr::ForLoop {
            range,
            pattern: convert_pattern(v.pattern(), offsets),
            iterable: Box::new(convert_expr(v.iterable(), offsets)),
            body: Box::new(convert_expr(v.body(), offsets)),
        },

        // Module
        ast::Expr::ModuleImport(v) => JAstExpr::ModuleImport {
            range,
            source: Box::new(convert_expr(v.source(), offsets)),
            new_name: v.new_name().map(|n| n.get().to_string()),
            imports: v.imports().map(|i| convert_imports(i, offsets)),
        },
        ast::Expr::ModuleInclude(v) => JAstExpr::ModuleInclude {
            range,
            source: Box::new(convert_expr(v.source(), offsets)),
        },

        // Jump
        ast::Expr::LoopBreak(_) => JAstExpr::LoopBreak { range },
        ast::Expr::LoopContinue(_) => JAstExpr::LoopContinue { range },
        ast::Expr::FuncReturn(v) => JAstExpr::FuncReturn {
            range,
            body: v.body().map(|e| Box::new(convert_expr(e, offsets))),
        },
    }
}

fn convert_math_text_kind(kind: ast::MathTextKind) -> JAstMathTextKind {
    match kind {
        ast::MathTextKind::Character(c) => JAstMathTextKind::Character { value: c },
        ast::MathTextKind::Number(n) => JAstMathTextKind::Number {
            value: n.to_string(),
        },
    }
}

fn convert_unit(unit: ast::Unit) -> JAstUnit {
    match unit {
        ast::Unit::Pt => JAstUnit::Pt,
        ast::Unit::Mm => JAstUnit::Mm,
        ast::Unit::Cm => JAstUnit::Cm,
        ast::Unit::In => JAstUnit::In,
        ast::Unit::Rad => JAstUnit::Rad,
        ast::Unit::Deg => JAstUnit::Deg,
        ast::Unit::Em => JAstUnit::Em,
        ast::Unit::Fr => JAstUnit::Fr,
        ast::Unit::Percent => JAstUnit::Percent,
    }
}

fn convert_unop(op: ast::UnOp) -> JAstUnOp {
    match op {
        ast::UnOp::Pos => JAstUnOp::Pos,
        ast::UnOp::Neg => JAstUnOp::Neg,
        ast::UnOp::Not => JAstUnOp::Not,
    }
}

fn convert_binop(op: ast::BinOp) -> JAstBinOp {
    match op {
        ast::BinOp::Add => JAstBinOp::Add,
        ast::BinOp::Sub => JAstBinOp::Sub,
        ast::BinOp::Mul => JAstBinOp::Mul,
        ast::BinOp::Div => JAstBinOp::Div,
        ast::BinOp::And => JAstBinOp::And,
        ast::BinOp::Or => JAstBinOp::Or,
        ast::BinOp::Eq => JAstBinOp::Eq,
        ast::BinOp::Neq => JAstBinOp::Neq,
        ast::BinOp::Lt => JAstBinOp::Lt,
        ast::BinOp::Leq => JAstBinOp::Leq,
        ast::BinOp::Gt => JAstBinOp::Gt,
        ast::BinOp::Geq => JAstBinOp::Geq,
        ast::BinOp::Assign => JAstBinOp::Assign,
        ast::BinOp::In => JAstBinOp::In,
        ast::BinOp::NotIn => JAstBinOp::NotIn,
        ast::BinOp::AddAssign => JAstBinOp::AddAssign,
        ast::BinOp::SubAssign => JAstBinOp::SubAssign,
        ast::BinOp::MulAssign => JAstBinOp::MulAssign,
        ast::BinOp::DivAssign => JAstBinOp::DivAssign,
    }
}

fn convert_array_item(item: ast::ArrayItem, offsets: &OffsetMap) -> JAstArrayItem {
    match item {
        ast::ArrayItem::Pos(e) => JAstArrayItem::Pos {
            expr: convert_expr(e, offsets),
        },
        ast::ArrayItem::Spread(s) => JAstArrayItem::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_dict_item(item: ast::DictItem, offsets: &OffsetMap) -> JAstDictItem {
    match item {
        ast::DictItem::Named(n) => JAstDictItem::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::DictItem::Keyed(k) => JAstDictItem::Keyed {
            key: convert_expr(k.key(), offsets),
            expr: convert_expr(k.expr(), offsets),
        },
        ast::DictItem::Spread(s) => JAstDictItem::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_arg(arg: ast::Arg, offsets: &OffsetMap) -> JAstArg {
    match arg {
        ast::Arg::Pos(e) => JAstArg::Pos {
            expr: convert_expr(e, offsets),
        },
        ast::Arg::Named(n) => JAstArg::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::Arg::Spread(s) => JAstArg::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_param(param: ast::Param, offsets: &OffsetMap) -> JAstParam {
    match param {
        ast::Param::Pos(p) => JAstParam::Pos {
            pattern: convert_pattern(p, offsets),
        },
        ast::Param::Named(n) => JAstParam::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::Param::Spread(s) => JAstParam::Spread {
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
            sink_expr: s.sink_expr().map(|e| convert_expr(e, offsets)),
        },
    }
}

fn convert_pattern(pattern: ast::Pattern, offsets: &OffsetMap) -> JAstPattern {
    match pattern {
        ast::Pattern::Normal(e) => JAstPattern::Normal {
            expr: Box::new(convert_expr(e, offsets)),
        },
        ast::Pattern::Placeholder(u) => JAstPattern::Placeholder {
            range: range_of(u.to_untyped(), offsets),
        },
        ast::Pattern::Parenthesized(p) => JAstPattern::Parenthesized {
            expr: Box::new(convert_expr(p.expr(), offsets)),
        },
        ast::Pattern::Destructuring(d) => JAstPattern::Destructuring {
            range: range_of(d.to_untyped(), offsets),
            items: d
                .items()
                .map(|i| convert_destructuring_item(i, offsets))
                .collect(),
        },
    }
}

fn convert_destructuring_item(
    item: ast::DestructuringItem,
    offsets: &OffsetMap,
) -> JAstDestructuringItem {
    match item {
        ast::DestructuringItem::Pattern(p) => JAstDestructuringItem::Pattern {
            pattern: convert_pattern(p, offsets),
        },
        ast::DestructuringItem::Named(n) => JAstDestructuringItem::Named {
            name: n.name().get().to_string(),
            pattern: convert_pattern(n.pattern(), offsets),
        },
        ast::DestructuringItem::Spread(s) => JAstDestructuringItem::Spread {
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_let_binding_kind(kind: ast::LetBindingKind, offsets: &OffsetMap) -> JAstLetBindingKind {
    match kind {
        ast::LetBindingKind::Normal(p) => JAstLetBindingKind::Normal {
            pattern: convert_pattern(p, offsets),
        },
        ast::LetBindingKind::Closure(i) => JAstLetBindingKind::Closure {
            name: i.get().to_string(),
        },
    }
}

fn convert_imports(imports: ast::Imports, _offsets: &OffsetMap) -> JAstImports {
    match imports {
        ast::Imports::Wildcard => JAstImports::Wildcard,
        ast::Imports::Items(items) => JAstImports::Items {
            items: items.iter().map(convert_import_item).collect(),
        },
    }
}

fn convert_import_item(item: ast::ImportItem) -> JAstImportItem {
    match item {
        ast::ImportItem::Simple(path) => JAstImportItem::Simple {
            path: path.iter().map(|i| i.get().to_string()).collect(),
            name: path.name().get().to_string(),
        },
        ast::ImportItem::Renamed(r) => JAstImportItem::Renamed {
            path: r.path().iter().map(|i| i.get().to_string()).collect(),
            original_name: r.original_name().get().to_string(),
            new_name: r.new_name().get().to_string(),
        },
    }
}

// --- Public API ---

#[derive(Serialize)]
pub struct ParseAstResult {
    pub root: Vec<JAstExpr>,
    pub errors: Vec<super::JError>,
}

pub fn make_ast_result(
    root: &SyntaxNode,
    mode: &super::ParseMode,
) -> Result<ParseAstResult, String> {
    let linked = LinkedNode::new(root);
    let mut offsets = OffsetMap::new();
    build_offset_map(&linked, &mut offsets);

    let exprs = match mode {
        super::ParseMode::Markup => {
            let markup: ast::Markup = root.cast().ok_or("Failed to cast root to Markup")?;
            convert_markup(markup, &offsets)
        }
        super::ParseMode::Code => {
            let code: ast::Code = root.cast().ok_or("Failed to cast root to Code")?;
            convert_code(code, &offsets)
        }
        super::ParseMode::Math => {
            let math: ast::Math = root.cast().ok_or("Failed to cast root to Math")?;
            convert_math(math, &offsets)
        }
    };

    let errors = super::collect_errors(&linked);

    Ok(ParseAstResult {
        root: exprs,
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(text: &str, mode: super::super::ParseMode) -> ParseAstResult {
        let root = match mode {
            super::super::ParseMode::Markup => typst_syntax::parse(text),
            super::super::ParseMode::Code => typst_syntax::parse_code(text),
            super::super::ParseMode::Math => typst_syntax::parse_math(text),
        };

        make_ast_result(&root, &mode).expect("make_ast_result should not fail")
    }

    #[test]
    fn handles_erroneous_code_without_panicking() {
        for src in ["#show:", "#for x in", "not"] {
            let result = parse_ok(src, super::super::ParseMode::Code);
            assert!(
                !result.errors.is_empty(),
                "expected syntax errors for input: {src:?}"
            );
        }
    }

    #[test]
    fn uses_null_range_for_placeholder_nodes() {
        let result = parse_ok("#show:", super::super::ParseMode::Code);
        let show_rule = result.root.into_iter().find_map(|expr| match expr {
            JAstExpr::ShowRule { transform, .. } => Some(transform),
            _ => None,
        });
        let Some(transform) = show_rule else {
            panic!("expected show rule");
        };

        match *transform {
            JAstExpr::None { range } => assert!(range.is_none()),
            _ => panic!("expected placeholder none expression"),
        }
    }

    #[test]
    fn handles_erroneous_math_without_panicking() {
        for src in ["√", "x_"] {
            let result = parse_ok(src, super::super::ParseMode::Math);
            assert!(
                !result.errors.is_empty(),
                "expected syntax errors for input: {src:?}"
            );
        }
    }

    #[test]
    fn handles_erroneous_markup_without_panicking() {
        let result = parse_ok("[*", super::super::ParseMode::Markup);
        assert!(!result.errors.is_empty());
    }
}
