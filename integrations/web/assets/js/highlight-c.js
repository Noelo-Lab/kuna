// highlight-c.js — the C syntax highlighter shared by the compare panes on the
// landing page and the function view on /decompile.
//
// One linear scan with a regex alternation; every emitted substring goes
// through escapeHtml (identifiers come from the binary and are
// attacker-controlled, so nothing reaches innerHTML unescaped). Colors are the
// `.tok-*` classes in assets/css/site.css.

const KEYWORDS = new Set(('if else while do for return break continue goto switch case default ' +
  'sizeof struct union enum typedef static extern const volatile unsigned signed register').split(' '));
const TYPES = new Set(('void char short int long float double bool undefined byte word dword qword ' +
  'uint ushort ulong uchar code char16_t wchar_t size_t').split(' '));
for (let n = 1; n <= 8; n++) ['undefined', 'int', 'uint'].forEach((b) => TYPES.add(b + n));

const TOK_RE = /\/\*[\s\S]*?(?:\*\/|$)|\/\/[^\n]*|"(?:\\.|[^"\\\n])*"|'(?:\\.|[^'\\\n])*'|0[xX][0-9a-fA-F]+[uUlL]*|(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?[fFlLuU]*|[A-Za-z_][A-Za-z0-9_]*/g;

function classify(tok, next) {
  const c = tok[0];
  if (c === '/') return 'tok-com'; // only comments start with '/' in TOK_RE
  if (c === '"') return 'tok-str';
  if (c === "'") return 'tok-chr';
  if (c === '.' || (c >= '0' && c <= '9')) return 'tok-num';
  if (/^dat_[0-9a-fA-F]+$/.test(tok)) return 'tok-glob';
  if (KEYWORDS.has(tok)) return 'tok-kw';
  if (TYPES.has(tok) || tok.endsWith('_t')) return 'tok-type';
  if (next === '(') return 'tok-fn';
  return '';
}

export function highlightC(code) {
  TOK_RE.lastIndex = 0;
  let out = '', last = 0, m;
  while ((m = TOK_RE.exec(code))) {
    const tok = m[0], end = m.index + tok.length;
    if (m.index > last) out += escapeHtml(code.slice(last, m.index));
    const cls = classify(tok, code[end]);
    out += cls ? `<span class="${cls}">${escapeHtml(tok)}</span>` : escapeHtml(tok);
    last = end;
  }
  return out + escapeHtml(code.slice(last));
}

export function escapeHtml(s) {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
}
