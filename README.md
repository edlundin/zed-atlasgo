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

The proxy is downloaded at runtime from this extension's GitHub release assets, using the same semantic version as the extension. For extension version `0.1.0`, the proxy URL is:

```text
https://github.com/edlundin/zed-atlasgo/releases/download/v0.1.0/atlas-ls-zed-proxy-{os}-{arch}
```

Supported asset names:

- `atlas-ls-zed-proxy-darwin-arm64`
- `atlas-ls-zed-proxy-darwin-amd64`
- `atlas-ls-zed-proxy-linux-arm64`
- `atlas-ls-zed-proxy-linux-amd64`
- `atlas-ls-zed-proxy-windows-amd64.exe`

The CI workflow builds these assets and publishes them to the matching GitHub release when pushing a `v*` tag.
