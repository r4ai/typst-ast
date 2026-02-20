export interface SyntaxNode {
  kind: string;
  range: [number, number];
  text?: string;
  children: SyntaxNode[];
}

export interface ParseError {
  message: string;
  range: [number, number];
}

export interface ParseResult {
  root: SyntaxNode;
  errors: ParseError[];
}

export type ParseMode = "markup" | "code" | "math";

export interface ParseOptions {
  mode?: ParseMode;
}

export declare function parse(
  text: string,
  options?: ParseOptions,
): ParseResult;

// --- AST types ---

export type Range = [number, number] | null;

export type AstExpr =
  | AstText
  | AstSpace
  | AstLinebreak
  | AstParbreak
  | AstEscape
  | AstShorthand
  | AstSmartQuote
  | AstStrong
  | AstEmph
  | AstRaw
  | AstLink
  | AstLabel
  | AstRef
  | AstHeading
  | AstListItem
  | AstEnumItem
  | AstTermItem
  | AstEquation
  | AstMath
  | AstMathText
  | AstMathIdent
  | AstMathShorthand
  | AstMathAlignPoint
  | AstMathDelimited
  | AstMathAttach
  | AstMathPrimes
  | AstMathFrac
  | AstMathRoot
  | AstIdent
  | AstNone
  | AstAuto
  | AstBool
  | AstInt
  | AstFloat
  | AstNumeric
  | AstStr
  | AstCodeBlock
  | AstContentBlock
  | AstParenthesized
  | AstArray
  | AstDict
  | AstUnary
  | AstBinary
  | AstFieldAccess
  | AstFuncCall
  | AstClosure
  | AstLetBinding
  | AstDestructAssignment
  | AstSetRule
  | AstShowRule
  | AstContextual
  | AstConditional
  | AstWhileLoop
  | AstForLoop
  | AstModuleImport
  | AstModuleInclude
  | AstLoopBreak
  | AstLoopContinue
  | AstFuncReturn;

// Markup

export interface AstText {
  kind: "text";
  range: Range;
  text: string;
}

export interface AstSpace {
  kind: "space";
  range: Range;
}

export interface AstLinebreak {
  kind: "linebreak";
  range: Range;
}

export interface AstParbreak {
  kind: "parbreak";
  range: Range;
}

export interface AstEscape {
  kind: "escape";
  range: Range;
  character: string;
}

export interface AstShorthand {
  kind: "shorthand";
  range: Range;
  character: string;
}

export interface AstSmartQuote {
  kind: "smartQuote";
  range: Range;
  double: boolean;
}

export interface AstStrong {
  kind: "strong";
  range: Range;
  body: AstExpr[];
}

export interface AstEmph {
  kind: "emph";
  range: Range;
  body: AstExpr[];
}

export interface AstRaw {
  kind: "raw";
  range: Range;
  lines: string[];
  lang: string | null;
  block: boolean;
}

export interface AstLink {
  kind: "link";
  range: Range;
  url: string;
}

export interface AstLabel {
  kind: "label";
  range: Range;
  name: string;
}

export interface AstRef {
  kind: "ref";
  range: Range;
  target: string;
  supplement: AstExpr[] | null;
}

export interface AstHeading {
  kind: "heading";
  range: Range;
  depth: number;
  body: AstExpr[];
}

export interface AstListItem {
  kind: "listItem";
  range: Range;
  body: AstExpr[];
}

export interface AstEnumItem {
  kind: "enumItem";
  range: Range;
  number: number | null;
  body: AstExpr[];
}

export interface AstTermItem {
  kind: "termItem";
  range: Range;
  term: AstExpr[];
  description: AstExpr[];
}

export interface AstEquation {
  kind: "equation";
  range: Range;
  body: AstExpr[];
  block: boolean;
}

// Math

export interface AstMath {
  kind: "math";
  range: Range;
  body: AstExpr[];
}

export type AstMathTextKind =
  | { kind: "character"; value: string }
  | { kind: "number"; value: string };

export interface AstMathText {
  kind: "mathText";
  range: Range;
  text: AstMathTextKind;
}

export interface AstMathIdent {
  kind: "mathIdent";
  range: Range;
  name: string;
}

export interface AstMathShorthand {
  kind: "mathShorthand";
  range: Range;
  character: string;
}

export interface AstMathAlignPoint {
  kind: "mathAlignPoint";
  range: Range;
}

export interface AstMathDelimited {
  kind: "mathDelimited";
  range: Range;
  open: AstExpr;
  body: AstExpr[];
  close: AstExpr;
}

export interface AstMathAttach {
  kind: "mathAttach";
  range: Range;
  base: AstExpr;
  bottom: AstExpr | null;
  top: AstExpr | null;
  primes: number | null;
}

export interface AstMathPrimes {
  kind: "mathPrimes";
  range: Range;
  count: number;
}

export interface AstMathFrac {
  kind: "mathFrac";
  range: Range;
  num: AstExpr;
  denom: AstExpr;
}

export interface AstMathRoot {
  kind: "mathRoot";
  range: Range;
  index: number | null;
  radicand: AstExpr;
}

// Literals

export interface AstIdent {
  kind: "ident";
  range: Range;
  name: string;
}

export interface AstNone {
  kind: "none";
  range: Range;
}

export interface AstAuto {
  kind: "auto";
  range: Range;
}

export interface AstBool {
  kind: "bool";
  range: Range;
  value: boolean;
}

export interface AstInt {
  kind: "int";
  range: Range;
  value: number;
}

export interface AstFloat {
  kind: "float";
  range: Range;
  value: number;
}

export type AstUnit =
  | "pt"
  | "mm"
  | "cm"
  | "in"
  | "rad"
  | "deg"
  | "em"
  | "fr"
  | "percent";

export interface AstNumeric {
  kind: "numeric";
  range: Range;
  value: number;
  unit: AstUnit;
}

export interface AstStr {
  kind: "str";
  range: Range;
  value: string;
}

// Code structures

export interface AstCodeBlock {
  kind: "codeBlock";
  range: Range;
  body: AstExpr[];
}

export interface AstContentBlock {
  kind: "contentBlock";
  range: Range;
  body: AstExpr[];
}

export interface AstParenthesized {
  kind: "parenthesized";
  range: Range;
  expr: AstExpr;
}

export type AstArrayItem =
  | { kind: "pos"; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstArray {
  kind: "array";
  range: Range;
  items: AstArrayItem[];
}

export type AstDictItem =
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "keyed"; key: AstExpr; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstDict {
  kind: "dict";
  range: Range;
  items: AstDictItem[];
}

// Operations

export type AstUnOp = "pos" | "neg" | "not";

export type AstBinOp =
  | "add"
  | "sub"
  | "mul"
  | "div"
  | "and"
  | "or"
  | "eq"
  | "neq"
  | "lt"
  | "leq"
  | "gt"
  | "geq"
  | "assign"
  | "in"
  | "notIn"
  | "addAssign"
  | "subAssign"
  | "mulAssign"
  | "divAssign";

export interface AstUnary {
  kind: "unary";
  range: Range;
  op: AstUnOp;
  expr: AstExpr;
}

export interface AstBinary {
  kind: "binary";
  range: Range;
  op: AstBinOp;
  lhs: AstExpr;
  rhs: AstExpr;
}

export interface AstFieldAccess {
  kind: "fieldAccess";
  range: Range;
  target: AstExpr;
  field: string;
}

export type AstArg =
  | { kind: "pos"; expr: AstExpr }
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstFuncCall {
  kind: "funcCall";
  range: Range;
  callee: AstExpr;
  args: AstArg[];
}

export type AstParam =
  | { kind: "pos"; pattern: AstPattern }
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "spread"; sinkIdent: string | null; sinkExpr: AstExpr | null };

export interface AstClosure {
  kind: "closure";
  range: Range;
  name: string | null;
  params: AstParam[];
  body: AstExpr;
}

// Patterns

export type AstDestructuringItem =
  | { kind: "pattern"; pattern: AstPattern }
  | { kind: "named"; name: string; pattern: AstPattern }
  | { kind: "spread"; sinkIdent: string | null };

export type AstPattern =
  | { kind: "normal"; expr: AstExpr }
  | { kind: "placeholder"; range: Range }
  | { kind: "parenthesized"; expr: AstExpr }
  | {
      kind: "destructuring";
      range: Range;
      items: AstDestructuringItem[];
    };

// Bindings

export type AstLetBindingKind =
  | { kind: "normal"; pattern: AstPattern }
  | { kind: "closure"; name: string };

export interface AstLetBinding {
  kind: "letBinding";
  range: Range;
  bindingKind: AstLetBindingKind;
  init: AstExpr | null;
}

export interface AstDestructAssignment {
  kind: "destructAssignment";
  range: Range;
  pattern: AstPattern;
  value: AstExpr;
}

// Rules

export interface AstSetRule {
  kind: "setRule";
  range: Range;
  target: AstExpr;
  args: AstArg[];
  condition: AstExpr | null;
}

export interface AstShowRule {
  kind: "showRule";
  range: Range;
  selector: AstExpr | null;
  transform: AstExpr;
}

export interface AstContextual {
  kind: "contextual";
  range: Range;
  body: AstExpr;
}

// Control flow

export interface AstConditional {
  kind: "conditional";
  range: Range;
  condition: AstExpr;
  ifBody: AstExpr;
  elseBody: AstExpr | null;
}

export interface AstWhileLoop {
  kind: "whileLoop";
  range: Range;
  condition: AstExpr;
  body: AstExpr;
}

export interface AstForLoop {
  kind: "forLoop";
  range: Range;
  pattern: AstPattern;
  iterable: AstExpr;
  body: AstExpr;
}

// Module

export type AstImportItem =
  | { kind: "simple"; path: string[]; name: string }
  | {
      kind: "renamed";
      path: string[];
      originalName: string;
      newName: string;
    };

export type AstImports =
  | { kind: "wildcard" }
  | { kind: "items"; items: AstImportItem[] };

export interface AstModuleImport {
  kind: "moduleImport";
  range: Range;
  source: AstExpr;
  newName: string | null;
  imports: AstImports | null;
}

export interface AstModuleInclude {
  kind: "moduleInclude";
  range: Range;
  source: AstExpr;
}

// Jump

export interface AstLoopBreak {
  kind: "loopBreak";
  range: Range;
}

export interface AstLoopContinue {
  kind: "loopContinue";
  range: Range;
}

export interface AstFuncReturn {
  kind: "funcReturn";
  range: Range;
  body: AstExpr | null;
}

// Parse AST result

export interface ParseAstResult {
  root: AstExpr[];
  errors: ParseError[];
}

export declare function parseAst(
  text: string,
  options?: ParseOptions,
): ParseAstResult;
