# decompile.run

The published files for **[decompile.run](https://decompile.run)**, the in-browser
decompiler from [Kuna](https://github.com/Noelo-Lab/kuna). The decompiler runs as
WebAssembly entirely in the visitor's browser; nothing they open is uploaded.

**Do not edit this repository or open pull requests here.** Every file is generated, and
each deploy replaces the whole branch with a single new commit.

## How it relates to Kuna

- The source lives in [Noelo-Lab/kuna](https://github.com/Noelo-Lab/kuna), under
  `integrations/web/` (the page) and `decompiler/` (the engine, built to `wasm32-wasip1`).
  This README itself is `integrations/web/decompile-run/README.md` there.
- On every push to Kuna's `main` that touches the site, the engine, or the processor specs,
  Kuna's `.github/workflows/pages.yml` builds the site, splits it with
  `integrations/web/split-sites.sh`, and force-pushes the decompiler half here with a
  deploy key. Each commit message names the Kuna commit it was built from.
- GitHub Pages serves this branch at `decompile.run`. The other half, Kuna's landing page,
  stays at [kuna.noelo.org](https://kuna.noelo.org), whose `/decompile/` forwards here.

To change the site, change Kuna. Bugs go to
[Kuna's issues](https://github.com/Noelo-Lab/kuna/issues).

## Contents

| Path | What |
|---|---|
| `index.html`, `decompile/` | The page and its modules |
| `kuna_wasm.wasm` | The decompiler engine |
| `kuna-web.js`, `kuna-worker*.js`, `zip.js`, `sha256.js`, `vendor/` | The Worker that runs the engine, and its helpers and WASI runtime |
| `specs/`, `specs-small.json` | Processor specifications, fetched as each program needs them |
| `assets/` | Fonts, styles, and the code highlighter |
