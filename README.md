# typst-ast

A WebAssembly library that parses [Typst](https://typst.app/) source code and returns its CST/AST as a JSON-serializable object. Built with Rust and [`typst-syntax`](https://crates.io/crates/typst-syntax), distributed as npm packages via `wasm-pack`.

## Packages

| Package                                                                      | Target        |
| ---------------------------------------------------------------------------- | ------------- |
| [`@r4ai/typst-ast-web`](https://www.npmjs.com/package/@r4ai/typst-ast-web)   | Web (ESM, Browser) |
| [`@r4ai/typst-ast-node`](https://www.npmjs.com/package/@r4ai/typst-ast-node) | Node.js       |

## Usage

### Browser

```ts
import init, { parse, parseAst } from "@r4ai/typst-ast-web";

await init();

// CST (Concrete Syntax Tree)
const cst = parse("= Hello\n\nThis is *typst*.", { mode: "markup" });
console.log(cst);
// {
//   root: { kind: "Markup", range: [0, 25], children: [...] },
//   errors: []
// }

// AST (Abstract Syntax Tree)
const ast = parseAst("= Hello\n\nThis is *typst*.", { mode: "markup" });
console.log(ast);
// {
//   root: [
//     { kind: "heading", ... },
//     { kind: "parbreak", ... },
//     ...
//   ],
//   errors: []
// }
```

### Node.js

```ts
import { parse, parseAst } from "@r4ai/typst-ast-node";

const cst = parse("#let x = 1 + 2", { mode: "code" });
const ast = parseAst("#let x = 1 + 2", { mode: "code" });
```

### API

Both functions accept the same parameters:

- `text: string` — Typst source code to parse
- `options.mode?: "markup" | "code" | "math"` — Parse mode (default: `"markup"`)

#### `parse(text, options?)`

Returns the CST (Concrete Syntax Tree) — a lossless syntax tree that preserves all tokens including whitespace and punctuation.

```ts
interface ParseResult {
  root: SyntaxNode;
  errors: ParseError[];
}

interface SyntaxNode {
  kind: string;
  range: [number, number];
  text?: string;
  children: SyntaxNode[];
}
```

#### `parseAst(text, options?)`

Returns the AST (Abstract Syntax Tree) — a typed, semantic tree where each node is a tagged union discriminated by `kind`. Unlike the CST, the AST extracts semantic information (e.g. heading depth, function callee, binary operator) into dedicated fields.

```ts
interface ParseAstResult {
  root: AstExpr[];
  errors: ParseError[];
}

// AstExpr is a discriminated union of 60 node types.
// Examples:
type AstExpr =
  | { kind: "text"; range: [number, number]; text: string }
  | { kind: "heading"; range: [number, number]; depth: number; body: AstExpr[] }
  | { kind: "strong"; range: [number, number]; body: AstExpr[] }
  | { kind: "funcCall"; range: [number, number]; callee: AstExpr; args: AstArg[] }
  | { kind: "binary"; range: [number, number]; op: AstBinOp; lhs: AstExpr; rhs: AstExpr }
  | // ... and 55 more variants
```

See [`src/types.ts`](./src/types.ts) for the full type definitions.

## Development

### Prerequisites

- Rust (stable) with `wasm32-unknown-unknown` target
- Node.js 24
- pnpm 10
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)

### Commands

```sh
# Install Node.js and pnpm via mise
mise install

# Install JS dependencies
pnpm install

# Build WASM packages (outputs to dist/web and dist/node)
pnpm run build:lib

# Lint
cargo fmt --check
cargo clippy -- -D warnings
pnpm run check
```

### Release

```sh
# Bump version and create a release commit + tag
bash scripts/release.sh 1.2.3
```

## Examples

### React app

A React-based example app is located at [`examples/app`](./examples/app).

```sh
cd examples/app
pnpm install
pnpm dev
```

## License

MIT
