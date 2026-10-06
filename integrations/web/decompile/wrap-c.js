// wrap-c.js — where to break a line of C that is wider than the pane. The
// line is read as nested bracket groups; a group that does not fit breaks at
// its loosest operators first (`;` then `,` then `?:`, `||`, `&&`, `|` … `*`),
// all of that kind at once, and its continuation lines line up after its
// opening bracket. Only then are the pieces, and the groups inside them,
// broken further, a piece's own continuation rows four columns in from it. An assignment's `=` breaks only when breaking inside its
// right-hand side cannot make the line fit. Strings and comments never break.
//
// DOM-free: `wrapBreaks(text, cols)` returns `[{at, eat, indent}]` — break
// before offset `at`, drop `eat` characters there (the space a break
// replaces), and start the next row at column `indent`.

const OPS = ['<<=', '>>=', '&&', '||', '==', '!=', '<=', '>=', '<<', '>>', '+=', '-=', '*=', '/=', '%=',
  '&=', '|=', '^=', '=', '<', '>', '+', '-', '*', '/', '%', '&', '|', '^', '?', ':'];

const PREC = {
  ';': 0, ',': 1, '?': 3, ':': 3, '||': 4, '&&': 5, '|': 6, '^': 7, '&': 8, '==': 9, '!=': 9,
  '<': 10, '>': 10, '<=': 10, '>=': 10, '<<': 11, '>>': 11, '+': 12, '-': 12, '*': 13, '/': 13, '%': 13,
};
const ASSIGN = 2;
const precOf = (op) => PREC[op] ?? ASSIGN;

/** The end of a quoted literal starting at `i` (index of its closing quote), or -1 when it is not one. */
function literalEnd(text, i) {
  const q = text[i];
  for (let j = i + 1; j < text.length; j++) {
    if (text[j] === '\\') { j++; continue; }
    if (text[j] === q) return j;
    if (q === "'" && j - i > 10) return -1;
  }
  return q === '"' ? text.length - 1 : -1;
}

/** The bracket groups of `text`, each with the break points directly inside it. */
function parse(text) {
  const root = { open: -1, close: text.length, subs: [], cands: [] };
  const stack = [root];
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    const g = stack[stack.length - 1];
    if (c === '"' || c === "'") {
      const end = literalEnd(text, i);
      if (end >= 0) i = end;
      continue;
    }
    if (c === '/' && text[i + 1] === '/') break;
    if (c === '/' && text[i + 1] === '*') {
      const end = text.indexOf('*/', i + 2);
      i = end < 0 ? text.length : end + 1;
      continue;
    }
    if (c === '(' || c === '[') {
      const sub = { open: i, close: text.length, subs: [], cands: [] };
      g.subs.push(sub);
      stack.push(sub);
    } else if ((c === ')' || c === ']') && stack.length > 1) {
      g.close = i;
      stack.pop();
    } else if (c === ',') {
      g.cands.push({ at: i + 1, eat: text[i + 1] === ' ' ? 1 : 0, prec: PREC[','] });
    } else if (c === ';' && stack.length > 1 && text[i + 1] === ' ') {
      g.cands.push({ at: i + 1, eat: 1, prec: PREC[';'] });
    } else if (c === ' ') {
      for (const op of OPS) {
        if (text.startsWith(op, i + 1) && text[i + 1 + op.length] === ' ') {
          g.cands.push({ at: i + 1 + op.length, eat: 1, prec: precOf(op) });
          i += op.length;
          break;
        }
      }
    }
  }
  return root;
}

/**
 * Where to break `text` so that each row fits in `cols` columns (as far as
 * its break points allow). Empty when it already fits.
 */
export function wrapBreaks(text, cols) {
  if (!text || text.length <= cols || cols < 8) return [];
  const root = parse(text);
  const base = text.length - text.trimStart().length;
  const breaks = [];
  let overflow = 0;

  const alignAfter = (col, outer) => (col <= Math.max(cols * 0.6, outer + 4) ? col : Math.min(col, outer + 4));

  function layout(g, s, e, col, align, tail, keepAssign = false) {
    if (col + (e - s) + tail <= cols) return col + (e - s);
    const own = g.cands.filter((k) => k.at > s && k.at < e && !(keepAssign && k.prec === ASSIGN));
    if (own.length) {
      const p = Math.min(...own.map((k) => k.prec));
      if (p === ASSIGN) {
        const save = breaks.length;
        const before = overflow;
        const end = layout(g, s, e, col, align, tail, true);
        if (overflow === before) return end;
        breaks.length = save;
        overflow = before;
      }
      let at = s;
      let c = col;
      for (const k of own) {
        if (k.prec !== p) continue;
        layout(g, at, k.at, c, c + 4, 0, keepAssign);
        breaks.push({ at: k.at, eat: k.eat, indent: align });
        at = k.at + k.eat;
        c = align;
      }
      return layout(g, at, e, c, c + 4, tail, keepAssign);
    }
    return inner(g, s, e, col, align, tail);
  }

  function inner(g, s, e, col, align, tail) {
    const subs = g.subs.filter((x) => x.open >= s && x.close < e);
    if (!subs.length) {
      overflow++;
      return col + (e - s);
    }
    let c = col;
    let at = s;
    subs.forEach((sub, n) => {
      c += sub.open + 1 - at;
      const next = n + 1 < subs.length ? subs[n + 1].open + 1 : e;
      const subTail = next - sub.close + (n + 1 < subs.length ? 0 : tail);
      c = layout(sub, sub.open + 1, sub.close, c, alignAfter(c, align), subTail);
      at = sub.close;
    });
    return c + (e - at);
  }

  layout(root, 0, text.length, 0, base + 4, 0);
  return breaks.sort((a, b) => a.at - b.at);
}

/** `text` with its breaks applied (for tests and plain-text copies). */
export function applyBreaks(text, breaks) {
  let out = '';
  let at = 0;
  for (const b of breaks) {
    out += text.slice(at, b.at) + '\n' + ' '.repeat(b.indent);
    at = b.at + b.eat;
  }
  return out + text.slice(at);
}
