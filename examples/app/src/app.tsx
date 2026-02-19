import init, * as typst from "@r4ai/typst-ast-web";
import { useEffect, useRef, useState } from "react";
import "./app.css";

type Mode = "markup" | "code" | "math";
type OutputType = "cst" | "ast";

export function App() {
  const [ready, setReady] = useState(false);
  const [source, setSource] = useState("= Hello\n\nThis is *typst*.");
  const [mode, setMode] = useState<Mode>("markup");
  const [outputType, setOutputType] = useState<OutputType>("ast");
  const [result, setResult] = useState<
    typst.ParseResult | typst.ParseAstResult | null
  >(null);
  const [error, setError] = useState<string | null>(null);
  const initialized = useRef(false);

  useEffect(() => {
    if (initialized.current) return;
    initialized.current = true;
    init().then(() => setReady(true));
  }, []);

  const parse = () => {
    if (!ready) return;
    try {
      const r =
        outputType === "ast"
          ? typst.parseAst(source, { mode })
          : typst.parse(source, { mode });
      setResult(r);
      setError(null);
    } catch (e) {
      setError(String(e));
      setResult(null);
    }
  };

  return (
    <div className="container">
      <h1>typst-ast playground</h1>
      <div className="controls">
        <select value={mode} onChange={(e) => setMode(e.target.value as Mode)}>
          <option value="markup">markup</option>
          <option value="code">code</option>
          <option value="math">math</option>
        </select>
        <select
          value={outputType}
          onChange={(e) => setOutputType(e.target.value as OutputType)}
        >
          <option value="ast">AST</option>
          <option value="cst">CST</option>
        </select>
        <button type="button" onClick={parse} disabled={!ready}>
          {ready ? "Parse" : "Loading..."}
        </button>
      </div>
      <textarea
        value={source}
        onChange={(e) => setSource(e.target.value)}
        rows={10}
        spellCheck={false}
      />
      <pre className={error ? "output error" : "output"}>
        {error ?? (result !== null ? JSON.stringify(result, null, 2) : "")}
      </pre>
    </div>
  );
}
