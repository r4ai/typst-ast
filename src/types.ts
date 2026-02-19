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
  range: [number, number];
  text: string;
}

export interface AstSpace {
  kind: "space";
  range: [number, number];
}

export interface AstLinebreak {
  kind: "linebreak";
  range: [number, number];
}

export interface AstParbreak {
  kind: "parbreak";
  range: [number, number];
}

export interface AstEscape {
  kind: "escape";
  range: [number, number];
  character: string;
}

export interface AstShorthand {
  kind: "shorthand";
  range: [number, number];
  character: string;
}

export interface AstSmartQuote {
  kind: "smartQuote";
  range: [number, number];
  double: boolean;
}

export interface AstStrong {
  kind: "strong";
  range: [number, number];
  body: AstExpr[];
}

export interface AstEmph {
  kind: "emph";
  range: [number, number];
  body: AstExpr[];
}

export interface AstRaw {
  kind: "raw";
  range: [number, number];
  lines: string[];
  lang: string | null;
  block: boolean;
}

export interface AstLink {
  kind: "link";
  range: [number, number];
  url: string;
}

export interface AstLabel {
  kind: "label";
  range: [number, number];
  name: string;
}

export interface AstRef {
  kind: "ref";
  range: [number, number];
  target: string;
  supplement: AstExpr[] | null;
}

export interface AstHeading {
  kind: "heading";
  range: [number, number];
  depth: number;
  body: AstExpr[];
}

export interface AstListItem {
  kind: "listItem";
  range: [number, number];
  body: AstExpr[];
}

export interface AstEnumItem {
  kind: "enumItem";
  range: [number, number];
  number: number | null;
  body: AstExpr[];
}

export interface AstTermItem {
  kind: "termItem";
  range: [number, number];
  term: AstExpr[];
  description: AstExpr[];
}

export interface AstEquation {
  kind: "equation";
  range: [number, number];
  body: AstExpr[];
  block: boolean;
}

// Math

export interface AstMath {
  kind: "math";
  range: [number, number];
  body: AstExpr[];
}

export type AstMathTextKind =
  | { kind: "character"; value: string }
  | { kind: "number"; value: string };

export interface AstMathText {
  kind: "mathText";
  range: [number, number];
  text: AstMathTextKind;
}

export interface AstMathIdent {
  kind: "mathIdent";
  range: [number, number];
  name: string;
}

export interface AstMathShorthand {
  kind: "mathShorthand";
  range: [number, number];
  character: string;
}

export interface AstMathAlignPoint {
  kind: "mathAlignPoint";
  range: [number, number];
}

export interface AstMathDelimited {
  kind: "mathDelimited";
  range: [number, number];
  open: AstExpr;
  body: AstExpr[];
  close: AstExpr;
}

export interface AstMathAttach {
  kind: "mathAttach";
  range: [number, number];
  base: AstExpr;
  bottom: AstExpr | null;
  top: AstExpr | null;
  primes: number | null;
}

export interface AstMathPrimes {
  kind: "mathPrimes";
  range: [number, number];
  count: number;
}

export interface AstMathFrac {
  kind: "mathFrac";
  range: [number, number];
  num: AstExpr;
  denom: AstExpr;
}

export interface AstMathRoot {
  kind: "mathRoot";
  range: [number, number];
  index: number | null;
  radicand: AstExpr;
}

// Literals

export interface AstIdent {
  kind: "ident";
  range: [number, number];
  name: string;
}

export interface AstNone {
  kind: "none";
  range: [number, number];
}

export interface AstAuto {
  kind: "auto";
  range: [number, number];
}

export interface AstBool {
  kind: "bool";
  range: [number, number];
  value: boolean;
}

export interface AstInt {
  kind: "int";
  range: [number, number];
  value: number;
}

export interface AstFloat {
  kind: "float";
  range: [number, number];
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
  range: [number, number];
  value: number;
  unit: AstUnit;
}

export interface AstStr {
  kind: "str";
  range: [number, number];
  value: string;
}

// Code structures

export interface AstCodeBlock {
  kind: "codeBlock";
  range: [number, number];
  body: AstExpr[];
}

export interface AstContentBlock {
  kind: "contentBlock";
  range: [number, number];
  body: AstExpr[];
}

export interface AstParenthesized {
  kind: "parenthesized";
  range: [number, number];
  expr: AstExpr;
}

export type AstArrayItem =
  | { kind: "pos"; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstArray {
  kind: "array";
  range: [number, number];
  items: AstArrayItem[];
}

export type AstDictItem =
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "keyed"; key: AstExpr; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstDict {
  kind: "dict";
  range: [number, number];
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
  range: [number, number];
  op: AstUnOp;
  expr: AstExpr;
}

export interface AstBinary {
  kind: "binary";
  range: [number, number];
  op: AstBinOp;
  lhs: AstExpr;
  rhs: AstExpr;
}

export interface AstFieldAccess {
  kind: "fieldAccess";
  range: [number, number];
  target: AstExpr;
  field: string;
}

export type AstArg =
  | { kind: "pos"; expr: AstExpr }
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "spread"; expr: AstExpr; sinkIdent: string | null };

export interface AstFuncCall {
  kind: "funcCall";
  range: [number, number];
  callee: AstExpr;
  args: AstArg[];
}

export type AstParam =
  | { kind: "pos"; pattern: AstPattern }
  | { kind: "named"; name: string; expr: AstExpr }
  | { kind: "spread"; sinkIdent: string | null; sinkExpr: AstExpr | null };

export interface AstClosure {
  kind: "closure";
  range: [number, number];
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
  | { kind: "placeholder"; range: [number, number] }
  | { kind: "parenthesized"; expr: AstExpr }
  | {
      kind: "destructuring";
      range: [number, number];
      items: AstDestructuringItem[];
    };

// Bindings

export type AstLetBindingKind =
  | { kind: "normal"; pattern: AstPattern }
  | { kind: "closure"; name: string };

export interface AstLetBinding {
  kind: "letBinding";
  range: [number, number];
  bindingKind: AstLetBindingKind;
  init: AstExpr | null;
}

export interface AstDestructAssignment {
  kind: "destructAssignment";
  range: [number, number];
  pattern: AstPattern;
  value: AstExpr;
}

// Rules

export interface AstSetRule {
  kind: "setRule";
  range: [number, number];
  target: AstExpr;
  args: AstArg[];
  condition: AstExpr | null;
}

export interface AstShowRule {
  kind: "showRule";
  range: [number, number];
  selector: AstExpr | null;
  transform: AstExpr;
}

export interface AstContextual {
  kind: "contextual";
  range: [number, number];
  body: AstExpr;
}

// Control flow

export interface AstConditional {
  kind: "conditional";
  range: [number, number];
  condition: AstExpr;
  ifBody: AstExpr;
  elseBody: AstExpr | null;
}

export interface AstWhileLoop {
  kind: "whileLoop";
  range: [number, number];
  condition: AstExpr;
  body: AstExpr;
}

export interface AstForLoop {
  kind: "forLoop";
  range: [number, number];
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
  range: [number, number];
  source: AstExpr;
  newName: string | null;
  imports: AstImports | null;
}

export interface AstModuleInclude {
  kind: "moduleInclude";
  range: [number, number];
  source: AstExpr;
}

// Jump

export interface AstLoopBreak {
  kind: "loopBreak";
  range: [number, number];
}

export interface AstLoopContinue {
  kind: "loopContinue";
  range: [number, number];
}

export interface AstFuncReturn {
  kind: "funcReturn";
  range: [number, number];
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
