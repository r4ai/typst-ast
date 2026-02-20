use std::num::NonZeroUsize;

use typst_syntax::ast::{self, AstNode};

use super::expr::AstExpr;
use super::offset::{range_of, OffsetMap};
use super::types::*;

pub fn convert_markup(markup: ast::Markup, offsets: &OffsetMap) -> Vec<AstExpr> {
    markup.exprs().map(|e| convert_expr(e, offsets)).collect()
}

pub fn convert_code(code: ast::Code, offsets: &OffsetMap) -> Vec<AstExpr> {
    code.exprs().map(|e| convert_expr(e, offsets)).collect()
}

pub fn convert_math(math: ast::Math, offsets: &OffsetMap) -> Vec<AstExpr> {
    math.exprs().map(|e| convert_expr(e, offsets)).collect()
}

pub fn convert_expr(expr: ast::Expr, offsets: &OffsetMap) -> AstExpr {
    let node = expr.to_untyped();
    let range = range_of(node, offsets);

    match expr {
        ast::Expr::Text(v) => AstExpr::Text {
            range,
            text: v.get().to_string(),
        },
        ast::Expr::Space(_) => AstExpr::Space { range },
        ast::Expr::Linebreak(_) => AstExpr::Linebreak { range },
        ast::Expr::Parbreak(_) => AstExpr::Parbreak { range },
        ast::Expr::Escape(v) => AstExpr::Escape {
            range,
            character: v.get(),
        },
        ast::Expr::Shorthand(v) => AstExpr::Shorthand {
            range,
            character: v.get(),
        },
        ast::Expr::SmartQuote(v) => AstExpr::SmartQuote {
            range,
            double: v.double(),
        },
        ast::Expr::Strong(v) => AstExpr::Strong {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Emph(v) => AstExpr::Emph {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Raw(v) => AstExpr::Raw {
            range,
            lines: v.lines().map(|l| l.get().to_string()).collect(),
            lang: v.lang().map(|l| l.get().to_string()),
            block: v.block(),
        },
        ast::Expr::Link(v) => AstExpr::Link {
            range,
            url: v.get().to_string(),
        },
        ast::Expr::Label(v) => AstExpr::Label {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::Ref(v) => AstExpr::Ref {
            range,
            target: v.target().to_string(),
            supplement: v.supplement().map(|s| convert_markup(s.body(), offsets)),
        },
        ast::Expr::Heading(v) => AstExpr::Heading {
            range,
            depth: NonZeroUsize::get(v.depth()),
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::ListItem(v) => AstExpr::ListItem {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::EnumItem(v) => AstExpr::EnumItem {
            range,
            number: v.number(),
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::TermItem(v) => AstExpr::TermItem {
            range,
            term: convert_markup(v.term(), offsets),
            description: convert_markup(v.description(), offsets),
        },
        ast::Expr::Equation(v) => AstExpr::Equation {
            range,
            body: convert_math(v.body(), offsets),
            block: v.block(),
        },

        // Math
        ast::Expr::MathText(v) => AstExpr::MathText {
            range,
            text: convert_math_text_kind(v.get()),
        },
        ast::Expr::MathIdent(v) => AstExpr::MathIdent {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::MathShorthand(v) => AstExpr::MathShorthand {
            range,
            character: v.get(),
        },
        ast::Expr::MathAlignPoint(_) => AstExpr::MathAlignPoint { range },
        ast::Expr::MathDelimited(v) => AstExpr::MathDelimited {
            range,
            open: Box::new(convert_expr(v.open(), offsets)),
            body: convert_math(v.body(), offsets),
            close: Box::new(convert_expr(v.close(), offsets)),
        },
        ast::Expr::MathAttach(v) => AstExpr::MathAttach {
            range,
            base: Box::new(convert_expr(v.base(), offsets)),
            bottom: v.bottom().map(|e| Box::new(convert_expr(e, offsets))),
            top: v.top().map(|e| Box::new(convert_expr(e, offsets))),
            primes: v.primes().map(|p| p.count()),
        },
        ast::Expr::MathPrimes(v) => AstExpr::MathPrimes {
            range,
            count: v.count(),
        },
        ast::Expr::MathFrac(v) => AstExpr::MathFrac {
            range,
            num: Box::new(convert_expr(v.num(), offsets)),
            denom: Box::new(convert_expr(v.denom(), offsets)),
        },
        ast::Expr::MathRoot(v) => AstExpr::MathRoot {
            range,
            index: v.index(),
            radicand: Box::new(convert_expr(v.radicand(), offsets)),
        },
        ast::Expr::Math(v) => AstExpr::Math {
            range,
            body: convert_math(v, offsets),
        },

        // Literals
        ast::Expr::Ident(v) => AstExpr::Ident {
            range,
            name: v.get().to_string(),
        },
        ast::Expr::None(_) => AstExpr::None { range },
        ast::Expr::Auto(_) => AstExpr::Auto { range },
        ast::Expr::Bool(v) => AstExpr::Bool {
            range,
            value: v.get(),
        },
        ast::Expr::Int(v) => AstExpr::Int {
            range,
            value: v.get(),
        },
        ast::Expr::Float(v) => AstExpr::Float {
            range,
            value: v.get(),
        },
        ast::Expr::Numeric(v) => {
            let (value, unit) = v.get();
            AstExpr::Numeric {
                range,
                value,
                unit: convert_unit(unit),
            }
        }
        ast::Expr::Str(v) => AstExpr::Str {
            range,
            value: v.get().to_string(),
        },

        // Code structures
        ast::Expr::CodeBlock(v) => AstExpr::CodeBlock {
            range,
            body: convert_code(v.body(), offsets),
        },
        ast::Expr::ContentBlock(v) => AstExpr::ContentBlock {
            range,
            body: convert_markup(v.body(), offsets),
        },
        ast::Expr::Parenthesized(v) => AstExpr::Parenthesized {
            range,
            expr: Box::new(convert_expr(v.expr(), offsets)),
        },
        ast::Expr::Array(v) => AstExpr::Array {
            range,
            items: v.items().map(|i| convert_array_item(i, offsets)).collect(),
        },
        ast::Expr::Dict(v) => AstExpr::Dict {
            range,
            items: v.items().map(|i| convert_dict_item(i, offsets)).collect(),
        },

        // Operations
        ast::Expr::Unary(v) => AstExpr::Unary {
            range,
            op: convert_unop(v.op()),
            expr: Box::new(convert_expr(v.expr(), offsets)),
        },
        ast::Expr::Binary(v) => AstExpr::Binary {
            range,
            op: convert_binop(v.op()),
            lhs: Box::new(convert_expr(v.lhs(), offsets)),
            rhs: Box::new(convert_expr(v.rhs(), offsets)),
        },
        ast::Expr::FieldAccess(v) => AstExpr::FieldAccess {
            range,
            target: Box::new(convert_expr(v.target(), offsets)),
            field: v.field().get().to_string(),
        },
        ast::Expr::FuncCall(v) => AstExpr::FuncCall {
            range,
            callee: Box::new(convert_expr(v.callee(), offsets)),
            args: v.args().items().map(|a| convert_arg(a, offsets)).collect(),
        },
        ast::Expr::Closure(v) => AstExpr::Closure {
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
        ast::Expr::LetBinding(v) => AstExpr::LetBinding {
            range,
            binding_kind: convert_let_binding_kind(v.kind(), offsets),
            init: v.init().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::DestructAssignment(v) => AstExpr::DestructAssignment {
            range,
            pattern: convert_pattern(v.pattern(), offsets),
            value: Box::new(convert_expr(v.value(), offsets)),
        },

        // Rules
        ast::Expr::SetRule(v) => AstExpr::SetRule {
            range,
            target: Box::new(convert_expr(v.target(), offsets)),
            args: v.args().items().map(|a| convert_arg(a, offsets)).collect(),
            condition: v.condition().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::ShowRule(v) => AstExpr::ShowRule {
            range,
            selector: v.selector().map(|e| Box::new(convert_expr(e, offsets))),
            transform: Box::new(convert_expr(v.transform(), offsets)),
        },
        ast::Expr::Contextual(v) => AstExpr::Contextual {
            range,
            body: Box::new(convert_expr(v.body(), offsets)),
        },

        // Control flow
        ast::Expr::Conditional(v) => AstExpr::Conditional {
            range,
            condition: Box::new(convert_expr(v.condition(), offsets)),
            if_body: Box::new(convert_expr(v.if_body(), offsets)),
            else_body: v.else_body().map(|e| Box::new(convert_expr(e, offsets))),
        },
        ast::Expr::WhileLoop(v) => AstExpr::WhileLoop {
            range,
            condition: Box::new(convert_expr(v.condition(), offsets)),
            body: Box::new(convert_expr(v.body(), offsets)),
        },
        ast::Expr::ForLoop(v) => AstExpr::ForLoop {
            range,
            pattern: convert_pattern(v.pattern(), offsets),
            iterable: Box::new(convert_expr(v.iterable(), offsets)),
            body: Box::new(convert_expr(v.body(), offsets)),
        },

        // Module
        ast::Expr::ModuleImport(v) => AstExpr::ModuleImport {
            range,
            source: Box::new(convert_expr(v.source(), offsets)),
            new_name: v.new_name().map(|n| n.get().to_string()),
            imports: v.imports().map(|i| convert_imports(i)),
        },
        ast::Expr::ModuleInclude(v) => AstExpr::ModuleInclude {
            range,
            source: Box::new(convert_expr(v.source(), offsets)),
        },

        // Jump
        ast::Expr::LoopBreak(_) => AstExpr::LoopBreak { range },
        ast::Expr::LoopContinue(_) => AstExpr::LoopContinue { range },
        ast::Expr::FuncReturn(v) => AstExpr::FuncReturn {
            range,
            body: v.body().map(|e| Box::new(convert_expr(e, offsets))),
        },
    }
}

fn convert_math_text_kind(kind: ast::MathTextKind) -> MathTextKind {
    match kind {
        ast::MathTextKind::Character(c) => MathTextKind::Character { value: c },
        ast::MathTextKind::Number(n) => MathTextKind::Number {
            value: n.to_string(),
        },
    }
}

fn convert_unit(unit: ast::Unit) -> Unit {
    match unit {
        ast::Unit::Pt => Unit::Pt,
        ast::Unit::Mm => Unit::Mm,
        ast::Unit::Cm => Unit::Cm,
        ast::Unit::In => Unit::In,
        ast::Unit::Rad => Unit::Rad,
        ast::Unit::Deg => Unit::Deg,
        ast::Unit::Em => Unit::Em,
        ast::Unit::Fr => Unit::Fr,
        ast::Unit::Percent => Unit::Percent,
    }
}

fn convert_unop(op: ast::UnOp) -> UnOp {
    match op {
        ast::UnOp::Pos => UnOp::Pos,
        ast::UnOp::Neg => UnOp::Neg,
        ast::UnOp::Not => UnOp::Not,
    }
}

fn convert_binop(op: ast::BinOp) -> BinOp {
    match op {
        ast::BinOp::Add => BinOp::Add,
        ast::BinOp::Sub => BinOp::Sub,
        ast::BinOp::Mul => BinOp::Mul,
        ast::BinOp::Div => BinOp::Div,
        ast::BinOp::And => BinOp::And,
        ast::BinOp::Or => BinOp::Or,
        ast::BinOp::Eq => BinOp::Eq,
        ast::BinOp::Neq => BinOp::Neq,
        ast::BinOp::Lt => BinOp::Lt,
        ast::BinOp::Leq => BinOp::Leq,
        ast::BinOp::Gt => BinOp::Gt,
        ast::BinOp::Geq => BinOp::Geq,
        ast::BinOp::Assign => BinOp::Assign,
        ast::BinOp::In => BinOp::In,
        ast::BinOp::NotIn => BinOp::NotIn,
        ast::BinOp::AddAssign => BinOp::AddAssign,
        ast::BinOp::SubAssign => BinOp::SubAssign,
        ast::BinOp::MulAssign => BinOp::MulAssign,
        ast::BinOp::DivAssign => BinOp::DivAssign,
    }
}

fn convert_array_item(item: ast::ArrayItem, offsets: &OffsetMap) -> ArrayItem {
    match item {
        ast::ArrayItem::Pos(e) => ArrayItem::Pos {
            expr: convert_expr(e, offsets),
        },
        ast::ArrayItem::Spread(s) => ArrayItem::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_dict_item(item: ast::DictItem, offsets: &OffsetMap) -> DictItem {
    match item {
        ast::DictItem::Named(n) => DictItem::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::DictItem::Keyed(k) => DictItem::Keyed {
            key: convert_expr(k.key(), offsets),
            expr: convert_expr(k.expr(), offsets),
        },
        ast::DictItem::Spread(s) => DictItem::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_arg(arg: ast::Arg, offsets: &OffsetMap) -> Arg {
    match arg {
        ast::Arg::Pos(e) => Arg::Pos {
            expr: convert_expr(e, offsets),
        },
        ast::Arg::Named(n) => Arg::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::Arg::Spread(s) => Arg::Spread {
            expr: convert_expr(s.expr(), offsets),
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_param(param: ast::Param, offsets: &OffsetMap) -> Param {
    match param {
        ast::Param::Pos(p) => Param::Pos {
            pattern: convert_pattern(p, offsets),
        },
        ast::Param::Named(n) => Param::Named {
            name: n.name().get().to_string(),
            expr: convert_expr(n.expr(), offsets),
        },
        ast::Param::Spread(s) => Param::Spread {
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
            sink_expr: s.sink_expr().map(|e| convert_expr(e, offsets)),
        },
    }
}

fn convert_pattern(pattern: ast::Pattern, offsets: &OffsetMap) -> Pattern {
    match pattern {
        ast::Pattern::Normal(e) => Pattern::Normal {
            expr: Box::new(convert_expr(e, offsets)),
        },
        ast::Pattern::Placeholder(u) => Pattern::Placeholder {
            range: range_of(u.to_untyped(), offsets),
        },
        ast::Pattern::Parenthesized(p) => Pattern::Parenthesized {
            expr: Box::new(convert_expr(p.expr(), offsets)),
        },
        ast::Pattern::Destructuring(d) => Pattern::Destructuring {
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
) -> DestructuringItem {
    match item {
        ast::DestructuringItem::Pattern(p) => DestructuringItem::Pattern {
            pattern: convert_pattern(p, offsets),
        },
        ast::DestructuringItem::Named(n) => DestructuringItem::Named {
            name: n.name().get().to_string(),
            pattern: convert_pattern(n.pattern(), offsets),
        },
        ast::DestructuringItem::Spread(s) => DestructuringItem::Spread {
            sink_ident: s.sink_ident().map(|i| i.get().to_string()),
        },
    }
}

fn convert_let_binding_kind(kind: ast::LetBindingKind, offsets: &OffsetMap) -> LetBindingKind {
    match kind {
        ast::LetBindingKind::Normal(p) => LetBindingKind::Normal {
            pattern: convert_pattern(p, offsets),
        },
        ast::LetBindingKind::Closure(i) => LetBindingKind::Closure {
            name: i.get().to_string(),
        },
    }
}

fn convert_imports(imports: ast::Imports) -> Imports {
    match imports {
        ast::Imports::Wildcard => Imports::Wildcard,
        ast::Imports::Items(items) => Imports::Items {
            items: items.iter().map(convert_import_item).collect(),
        },
    }
}

fn convert_import_item(item: ast::ImportItem) -> ImportItem {
    match item {
        ast::ImportItem::Simple(path) => ImportItem::Simple {
            path: path.iter().map(|i| i.get().to_string()).collect(),
            name: path.name().get().to_string(),
        },
        ast::ImportItem::Renamed(r) => ImportItem::Renamed {
            path: r.path().iter().map(|i| i.get().to_string()).collect(),
            original_name: r.original_name().get().to_string(),
            new_name: r.new_name().get().to_string(),
        },
    }
}
