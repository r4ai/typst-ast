use serde::Serialize;

use super::offset::Range;
use super::types::*;

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AstExpr {
    // Markup
    Text {
        range: Range,
        text: String,
    },
    Space {
        range: Range,
    },
    Linebreak {
        range: Range,
    },
    Parbreak {
        range: Range,
    },
    Escape {
        range: Range,
        character: char,
    },
    Shorthand {
        range: Range,
        character: char,
    },
    SmartQuote {
        range: Range,
        double: bool,
    },
    Strong {
        range: Range,
        body: Vec<AstExpr>,
    },
    Emph {
        range: Range,
        body: Vec<AstExpr>,
    },
    Raw {
        range: Range,
        lines: Vec<String>,
        lang: Option<String>,
        block: bool,
    },
    Link {
        range: Range,
        url: String,
    },
    Label {
        range: Range,
        name: String,
    },
    Ref {
        range: Range,
        target: String,
        supplement: Option<Vec<AstExpr>>,
    },
    Heading {
        range: Range,
        depth: usize,
        body: Vec<AstExpr>,
    },
    ListItem {
        range: Range,
        body: Vec<AstExpr>,
    },
    EnumItem {
        range: Range,
        number: Option<u64>,
        body: Vec<AstExpr>,
    },
    TermItem {
        range: Range,
        term: Vec<AstExpr>,
        description: Vec<AstExpr>,
    },
    Equation {
        range: Range,
        body: Vec<AstExpr>,
        block: bool,
    },

    // Math
    Math {
        range: Range,
        body: Vec<AstExpr>,
    },
    MathText {
        range: Range,
        text: MathTextKind,
    },
    MathIdent {
        range: Range,
        name: String,
    },
    MathShorthand {
        range: Range,
        character: char,
    },
    MathAlignPoint {
        range: Range,
    },
    MathDelimited {
        range: Range,
        open: Box<AstExpr>,
        body: Vec<AstExpr>,
        close: Box<AstExpr>,
    },
    MathAttach {
        range: Range,
        base: Box<AstExpr>,
        bottom: Option<Box<AstExpr>>,
        top: Option<Box<AstExpr>>,
        primes: Option<usize>,
    },
    MathPrimes {
        range: Range,
        count: usize,
    },
    MathFrac {
        range: Range,
        num: Box<AstExpr>,
        denom: Box<AstExpr>,
    },
    MathRoot {
        range: Range,
        index: Option<u8>,
        radicand: Box<AstExpr>,
    },

    // Literals
    Ident {
        range: Range,
        name: String,
    },
    None {
        range: Range,
    },
    Auto {
        range: Range,
    },
    Bool {
        range: Range,
        value: bool,
    },
    Int {
        range: Range,
        value: i64,
    },
    Float {
        range: Range,
        value: f64,
    },
    Numeric {
        range: Range,
        value: f64,
        unit: Unit,
    },
    Str {
        range: Range,
        value: String,
    },

    // Code structures
    CodeBlock {
        range: Range,
        body: Vec<AstExpr>,
    },
    ContentBlock {
        range: Range,
        body: Vec<AstExpr>,
    },
    Parenthesized {
        range: Range,
        expr: Box<AstExpr>,
    },
    Array {
        range: Range,
        items: Vec<ArrayItem>,
    },
    Dict {
        range: Range,
        items: Vec<DictItem>,
    },

    // Operations
    Unary {
        range: Range,
        op: UnOp,
        expr: Box<AstExpr>,
    },
    Binary {
        range: Range,
        op: BinOp,
        lhs: Box<AstExpr>,
        rhs: Box<AstExpr>,
    },
    FieldAccess {
        range: Range,
        target: Box<AstExpr>,
        field: String,
    },
    FuncCall {
        range: Range,
        callee: Box<AstExpr>,
        args: Vec<Arg>,
    },
    Closure {
        range: Range,
        name: Option<String>,
        params: Vec<Param>,
        body: Box<AstExpr>,
    },

    // Bindings
    LetBinding {
        range: Range,
        binding_kind: LetBindingKind,
        init: Option<Box<AstExpr>>,
    },
    DestructAssignment {
        range: Range,
        pattern: Pattern,
        value: Box<AstExpr>,
    },

    // Rules
    SetRule {
        range: Range,
        target: Box<AstExpr>,
        args: Vec<Arg>,
        condition: Option<Box<AstExpr>>,
    },
    ShowRule {
        range: Range,
        selector: Option<Box<AstExpr>>,
        transform: Box<AstExpr>,
    },
    Contextual {
        range: Range,
        body: Box<AstExpr>,
    },

    // Control flow
    Conditional {
        range: Range,
        condition: Box<AstExpr>,
        if_body: Box<AstExpr>,
        else_body: Option<Box<AstExpr>>,
    },
    WhileLoop {
        range: Range,
        condition: Box<AstExpr>,
        body: Box<AstExpr>,
    },
    ForLoop {
        range: Range,
        pattern: Pattern,
        iterable: Box<AstExpr>,
        body: Box<AstExpr>,
    },

    // Module
    ModuleImport {
        range: Range,
        source: Box<AstExpr>,
        new_name: Option<String>,
        imports: Option<Imports>,
    },
    ModuleInclude {
        range: Range,
        source: Box<AstExpr>,
    },

    // Jump
    LoopBreak {
        range: Range,
    },
    LoopContinue {
        range: Range,
    },
    FuncReturn {
        range: Range,
        body: Option<Box<AstExpr>>,
    },
}
