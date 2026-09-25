// ctype.js — the C the study view writes back to the engine: identifier and
// type validation, declarators (`char[16]` + `buf` → `char buf[16]`), and
// signature parsing/building for the `prototype` directive. DOM-free.

const KEYWORDS = new Set(('auto break case char const continue default do double else enum extern float ' +
  'for goto if inline int long register restrict return short signed sizeof static struct switch ' +
  'typedef union unsigned void volatile while _Bool _Complex bool').split(' '));

/** Scalar spellings offered for retyping, with LP64 sizes (`long` shrinks on LLP64). */
export const C_SCALARS = [
  ['char', 1], ['signed char', 1], ['unsigned char', 1], ['short', 2], ['unsigned short', 2],
  ['int', 4], ['unsigned int', 4], ['long', 8], ['unsigned long', 8], ['long long', 8],
  ['unsigned long long', 8], ['float', 4], ['double', 8], ['_Bool', 1], ['wchar_t', 4], ['void', 0],
  ['int1', 1], ['int2', 2], ['int4', 4], ['int8', 8], ['uint1', 1], ['uint2', 2], ['uint4', 4], ['uint8', 8],
  ['undefined1', 1], ['undefined2', 2], ['undefined4', 4], ['undefined8', 8], ['float4', 4], ['float8', 8],
  ['bool', 1], ['byte', 1], ['word', 2], ['dword', 4], ['qword', 8], ['code', 1],
];
const SCALAR_SIZE = new Map(C_SCALARS);

/** An error message for a bad C identifier, or null. */
export function validateIdent(name) {
  const n = (name || '').trim();
  if (!n) return 'a name is required';
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(n)) return 'use letters, digits and _ (not starting with a digit)';
  if (KEYWORDS.has(n)) return `${n} is a C keyword`;
  if (n.length > 200) return 'that name is too long';
  return null;
}

/** Collapse whitespace and tidy `*` and `[]` spacing: `char  * *` → `char **`. */
export function normalizeType(text) {
  return (text || '')
    .replace(/\s+/g, ' ')
    .replace(/\s*\*\s*/g, ' *')
    .replace(/\* \*/g, '**')
    .replace(/\*\s+(?=\*)/g, '*')
    .replace(/\s*\[\s*/g, '[')
    .replace(/\s*\]/g, ']')
    .replace(/\(\s+/g, '(')
    .replace(/\s+\)/g, ')')
    .trim();
}

const TYPE_RE = /^(?:(?:const|volatile|signed|unsigned|struct|union|enum|long|short)\s+)*[A-Za-z_][A-Za-z0-9_]*(?:\s+(?:const|volatile|int|long|char|double))*(?:\s*\*(?:\s*const)?)*\s*(?:\[\d*\])*$/;
const FNPTR_RE = /^[A-Za-z_][A-Za-z0-9_ ]*\**\s*\(\s*\*+\s*\)\s*\([^()]*\)$/;

/**
 * An error message for text that is not a C type the directive grammar takes,
 * or null. Only the shape is checked here; the engine decides whether it
 * knows the name.
 */
export function validateCType(text) {
  const t = normalizeType(text);
  if (!t) return 'a type is required';
  if (/[;{}#\n\r]/.test(t)) return 'a type cannot contain ; { } # or a newline';
  if (TYPE_RE.test(t) || FNPTR_RE.test(t)) return null;
  return `not a C type: ${t}`;
}

/** The base name of a type (`const struct foo **[4]` → `struct foo`). */
export function baseType(text) {
  return normalizeType(text).replace(/\[.*$/, '').replace(/\*/g, '').replace(/\b(const|volatile)\b/g, '').replace(/\s+/g, ' ').trim();
}

/**
 * A declaration of `name` with type `ctype`: arrays and function pointers put
 * the name inside the declarator, pointers bind to the name.
 */
export function cDeclare(ctype, name) {
  const t = normalizeType(ctype);
  const fp = /^(.*?\(\s*\*+)\s*(\)\s*\(.*\))$/.exec(t);
  if (fp) return `${fp[1]}${name}${fp[2]}`;
  const arr = /^(.*?)((?:\[\d*\])+)$/.exec(t);
  const base = arr ? arr[1].trim() : t;
  const dims = arr ? arr[2] : '';
  return base.endsWith('*') ? `${base}${name}${dims}` : `${base} ${name}${dims}`;
}

/** Split `a, b(c, d), e` at top-level commas. */
function splitTop(text) {
  const out = [];
  let depth = 0;
  let cur = '';
  for (const ch of text) {
    if (ch === '(' || ch === '[') depth++;
    if (ch === ')' || ch === ']') depth--;
    if (ch === ',' && depth === 0) { out.push(cur); cur = ''; } else cur += ch;
  }
  if (cur.trim() || out.length) out.push(cur);
  return out.map((s) => s.trim());
}

const CONVENTIONS = /\b(__stdcall|__cdecl|__fastcall|__thiscall|__vectorcall|MSABI|syscall)\b/;

/** One parameter declaration → `{type, name}` (name '' when unnamed). */
export function parseParam(text) {
  const t = normalizeType(text);
  const fp = /^(.*?\(\s*\*+)\s*([A-Za-z_]\w*)\s*(\)\s*\(.*\))$/.exec(t);
  if (fp) return { type: normalizeType(fp[1] + fp[3]), name: fp[2] };
  const m = /^(.*?[\s*])([A-Za-z_]\w*)\s*((?:\[\d*\])*)$/.exec(t);
  if (m && !KEYWORDS.has(m[2]) && m[1].trim() && !/^(const|volatile|signed|unsigned|struct|union|enum)$/.test(m[1].trim())) {
    return { type: normalizeType(m[1] + m[3]), name: m[2] };
  }
  return { type: t, name: '' };
}

/**
 * Parse a C function declaration into `{ret, conv, name, params, varargs}`,
 * or null. `int main(int argc,char **argv)` → ret `int`, params
 * `[{type:'int',name:'argc'},{type:'char **',name:'argv'}]`.
 */
export function parseSignature(proto) {
  const text = (proto || '').replace(/;\s*$/, '').trim();
  const open = text.indexOf('(');
  const close = text.lastIndexOf(')');
  if (open < 0 || close < open) return null;
  let head = text.slice(0, open).trim();
  const convMatch = CONVENTIONS.exec(head);
  const conv = convMatch ? convMatch[1] : null;
  if (conv) head = head.replace(CONVENTIONS, ' ').replace(/\s+/g, ' ').trim();
  const nm = /([A-Za-z_][\w:~]*)\s*$/.exec(head);
  if (!nm) return null;
  const ret = normalizeType(head.slice(0, nm.index)) || 'int';
  const inner = text.slice(open + 1, close).trim();
  let params = [];
  let varargs = false;
  if (inner && inner !== 'void') {
    for (const part of splitTop(inner)) {
      if (part === '...') varargs = true;
      else params.push(parseParam(part));
    }
  }
  params = params.map((p) => ({ type: p.type, name: p.name }));
  return { ret, conv, name: nm[1], params, varargs };
}

/**
 * A Rust signature as kuna prints it (`unsafe fn main(mut argc: i32, mut argv:
 * *mut *mut u8) -> i32`, attribute lines allowed) → `{ret, name, params}`, for
 * display only: edits are C declarations.
 */
export function parseRustSignature(proto) {
  const text = (proto || '').replace(/^#\[[^\n]*\]\s*$/gm, '').replace(/;\s*$/, '').trim();
  const m = /\bfn\s+([A-Za-z_]\w*)\s*\(([\s\S]*)\)\s*(?:->\s*(.+))?$/.exec(text);
  if (!m) return null;
  const params = m[2].trim()
    ? splitTop(m[2]).map((p) => {
      const pm = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(.+)$/.exec(p);
      return pm ? { name: pm[1], type: pm[2].trim() } : { name: '', type: p };
    })
    : [];
  return { ret: (m[3] || '()').trim(), conv: null, name: m[1], params, varargs: false };
}

/** The declaration `parseSignature` would read back. */
export function buildPrototype(sig) {
  const params = sig.params.map((p, i) => (p.name ? cDeclare(p.type, p.name) : p.type || `undefined${i}`));
  if (sig.varargs) params.push('...');
  const list = params.length ? params.join(', ') : 'void';
  const conv = sig.conv ? `${sig.conv} ` : '';
  const ret = normalizeType(sig.ret);
  return `${ret.endsWith('*') ? ret : ret + ' '}${conv}${sig.name}(${list})`;
}

/**
 * Bytes a type occupies, when it can be told from the spelling; `bits` is the
 * pointer width and `llp64` selects Windows' 4-byte `long`. Null when unknown.
 */
export function typeSize(ctype, { bits = 64, llp64 = false, known = new Map() } = {}) {
  const t = normalizeType(ctype);
  if (!t) return null;
  if (/\(\s*\*/.test(t)) return bits / 8;
  const arr = /^(.*?)((?:\[\d+\])+)$/.exec(t);
  if (arr) {
    const inner = typeSize(arr[1], { bits, llp64, known });
    if (inner === null) return null;
    return [...arr[2].matchAll(/\[(\d+)\]/g)].reduce((n, m) => n * Number(m[1]), inner);
  }
  if (/\*$/.test(t)) return bits / 8;
  const base = baseType(t).replace(/^signed /, '');
  if (known.has(base)) return known.get(base);
  if (llp64 && /^(unsigned )?long$/.test(base)) return 4;
  const size = SCALAR_SIZE.get(base) ?? SCALAR_SIZE.get(base.replace(/^unsigned /, ''));
  if (size !== undefined) return size;
  const sized = /^(?:u?int|undefined|float)(\d+)$/.exec(base);
  return sized ? Number(sized[1]) : null;
}

/**
 * Every type name worth offering in the retype list: the scalars, the types
 * this function uses, the engine's known types and the session's typedefs.
 */
export function knownTypes({ types = [], known = [], typedefs = [] } = {}) {
  const names = new Set(C_SCALARS.map(([n]) => n).filter((n) => n !== 'void'));
  for (const t of types) if (t?.name) names.add(t.name);
  for (const t of known) if (t?.name) names.add(t.name);
  for (const tag of typedefs) names.add(tag);
  const pointers = ['char *', 'void *', 'unsigned char *', 'int *', 'long *'];
  return [...names, ...pointers].sort((a, b) => a.localeCompare(b));
}
