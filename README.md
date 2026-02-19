# typst-ast

A WebAssembly library that parses [Typst](https://typst.app/) source code and returns its AST as a JSON-serializable object. Built with Rust and [`typst-syntax`](https://crates.io/crates/typst-syntax), distributed as npm packages via `wasm-pack`.

## Packages

| Package                                                                      | Target        |
| ---------------------------------------------------------------------------- | ------------- |
| [`@r4ai/typst-ast-web`](https://www.npmjs.com/package/@r4ai/typst-ast-web)   | Web (ESM, Browser) |
| [`@r4ai/typst-ast-node`](https://www.npmjs.com/package/@r4ai/typst-ast-node) | Node.js       |

## Usage

### Browser

```ts
import init, { parse } from "@r4ai/typst-ast-web";

await init();

const result = parse("= Hello\n\nThis is *typst*.", { mode: "markup" });
console.log(result);
// {
//   root: { kind: "Markup", range: [0, 25], children: [...] },
//   errors: []
// }
```

### Node.js

```ts
import { parse } from "@r4ai/typst-ast-node";

const result = parse("#let x = 1 + 2", { mode: "code" });
console.log(result);
```

### API

#### `parse(text, options?)`

Parses Typst source text and returns the AST.

**Parameters**

- `text: string` — Typst source code to parse
- `options.mode?: "markup" | "code" | "math"` — Parse mode (default: `"markup"`)

**Return value**

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

interface ParseError {
  message: string;
  range: [number, number];
}
```

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
