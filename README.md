# Atlas HCL for Zed

Zed extension for Atlas HCL files.

## Features

- Tree-sitter HCL syntax highlighting.
- Atlas language-server integration for completion, references, diagnostics, and formatting.
- Recognizes `atlas.hcl` and Atlas schema suffixes: `.my.hcl`, `.ma.hcl`, `.pg.hcl`, `.lt.hcl`, `.ch.hcl`, `.ms.hcl`, `.rs.hcl`, `.dbx.hcl`, `.oc.hcl`, `.sf.hcl`, `.sp.hcl`.
- Maps Zed languages to Atlas LS language IDs like `atlas-schema-postgresql`, matching the official VS Code extension.

## Language server

The extension requires the Atlas CLI (`atlas`) on `PATH` and starts the language server with the official VS Code extension command: `atlas tool lsp --stdio`.

Install Atlas from https://atlasgo.io/getting-started/ before using completions. The extension does not download Atlas or `atlas-ls` binaries.

Zed is stricter than VS Code when deserializing LSP capabilities. Atlas LS currently returns `executeCommandProvider.commands: null`, so the extension starts the server through a small native Rust stdio proxy that patches this to `[]`.

The proxy is bundled into the extension WASM with `include_bytes!`, written to the extension work directory at runtime, and executed from there. No separate proxy install is needed.

The CI workflow builds proxy binaries for supported platforms, downloads them into `bundled-proxy/`, then checks the extension with those artifacts present. Built proxy binaries are uploaded as workflow artifacts, not committed.
