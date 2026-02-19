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
