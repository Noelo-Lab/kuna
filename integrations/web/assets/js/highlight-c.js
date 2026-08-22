// highlight-c.js — the decompiler-output syntax highlighter shared by the
// compare panes on the landing page and the function view on /decompile.
//
// One linear scan with a regex alternation; every emitted substring goes
// through escapeHtml (identifiers come from the binary and are
// attacker-controlled, so nothing reaches innerHTML unescaped). Colors are the
// `.tok-*` classes in assets/css/site.css.
//
// The scanner is language-neutral: only the keyword and type sets differ
// between C and Rust, so `highlight()` picks a dialect and the rest is shared.
// The file keeps its name (and the `highlightC` export) because the landing
// page's compare panes are C-only and import it by path.

const C = {
  keywords: new Set(('if else while do for return break continue goto switch case default ' +
    'sizeof struct union enum typedef static extern const volatile unsigned signed register').split(' ')),
  types: new Set(('void char short int long float double bool undefined byte word dword qword ' +
    'uint ushort ulong uchar code char16_t wchar_t size_t').split(' ')),
};
for (let n = 1; n <= 8; n++) ['undefined', 'int', 'uint'].forEach((b) => C.types.add(b + n));

const RUST = {
  keywords: new Set(('if else while loop for in match return break continue let mut fn unsafe ' +
    'extern as move ref impl struct enum trait const static type where pub use crate self ' +
    'true false panic').split(' ')),
  types: new Set(('i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char str ' +
    'Option Result Vec String Box').split(' ')),
};

// kuna's Rust output always opens with the `#[allow(...)]` attribute and always
// declares with `let mut`; either is decisive, and neither can appear in C.
function dialectFor(code) {
  return /^#\[allow\(|^\s*unsafe (extern "C" )?fn |\blet mut \b/m.test(code) ? RUST : C;
}

const TOK_RE = /\/\*[\s\S]*?(?:\*\/|$)|\/\/[^\n]*|"(?:\\.|[^"\\\n])*"|'(?:\\.|[^'\\\n])*'|0[xX][0-9a-fA-F]+[uUlL]*|(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?[fFlLuU]*|[A-Za-z_][A-Za-z0-9_]*/g;

function classify(tok, next, dialect) {
  const c = tok[0];
  if (c === '/') return 'tok-com'; // only comments start with '/' in TOK_RE
  if (c === '"') return 'tok-str';
  if (c === "'") return 'tok-chr';
  if (c === '.' || (c >= '0' && c <= '9')) return 'tok-num';
  if (/^dat_[0-9a-fA-F]+$/.test(tok)) return 'tok-glob';
  if (dialect.keywords.has(tok)) return 'tok-kw';
  if (dialect.types.has(tok) || tok.endsWith('_t')) return 'tok-type';
  if (next === '(' || next === '!') return 'tok-fn';
  return '';
}

function run(code, dialect) {
  TOK_RE.lastIndex = 0;
  let out = '', last = 0, m;
  while ((m = TOK_RE.exec(code))) {
    const tok = m[0], end = m.index + tok.length;
    if (m.index > last) out += escapeHtml(code.slice(last, m.index));
    const cls = classify(tok, code[end], dialect);
    out += cls ? `<span class="${cls}">${escapeHtml(tok)}</span>` : escapeHtml(tok);
    last = end;
  }
  return out + escapeHtml(code.slice(last));
}

/** Highlight decompiler output, picking the dialect from the text itself. */
export function highlight(code) {
  return run(code, dialectFor(code));
}

/** Highlight as C regardless of content (the landing page's compare panes). */
export function highlightC(code) {
  return run(code, C);
}

/** Highlight as Rust regardless of content. */
export function highlightRust(code) {
  return run(code, RUST);
}

export function escapeHtml(s) {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
}
