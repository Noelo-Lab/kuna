// wrap-c.js — how a line of C too long for its row is split, the way the
// Hex-Rays decompiler splits it at its right margin (120 columns, or the pane
// when that is narrower). Only three things split: a `&&`/`||` chain puts each
// operand on its own row with the operator leading it, a call puts each
// argument on its own row two columns in from the call's name (a signature's
// parameters eight in), and a comma list puts each element on its own row
// after its bracket. A chain nested in redundant brackets of the same operator
// (`((a && b) && c)`) is read as one chain and those brackets are left out of
// the display. Arithmetic, casts and assignments never split; what still does
// not fit the pane soft-wraps.
//
// DOM-free: `wrapPlan(text, margin)` returns `{breaks, drops}` — break before
// offset `at`, drop `eat` characters there and start the row at column
// `indent`; `drops` are the offsets of brackets left out.

const OPS = ['<<=', '>>=', '&&', '||', '==', '!=', '<=', '>=', '<<', '>>', '+=', '-=', '*=', '/=', '%=',
  '&=', '|=', '^=', '=', '<', '>', '+', '-', '*', '/', '%', '&', '|', '^', '?', ':'];

const PREC = {
  ';': 0, ',': 1, '?': 3, ':': 3, '||': 4, '&&': 5, '|': 6, '^': 7, '&': 8, '==': 9, '!=': 9,
  '<': 10, '>': 10, '<=': 10, '>=': 10, '<<': 11, '>>': 11, '+': 12, '-': 12, '*': 13, '/': 13, '%': 13,
};
const precOf = (op) => PREC[op] ?? 2;
const KEYWORDS = new Set(['if', 'while', 'for', 'switch', 'return', 'do', 'else', 'case']);
const IDENT = /[A-Za-z0-9_]/;

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

/** The word right before offset `i` (skipping spaces) and where it starts. */
function wordBefore(text, i) {
  let j = i;
  while (j > 0 && text[j - 1] === ' ') j--;
  let k = j;
  while (k > 0 && IDENT.test(text[k - 1])) k--;
  return { word: text.slice(k, j), start: k };
}

/**
 * The bracket groups of `text`, each with the operators directly inside it,
 * and where the code ends (a trailing `//` comment is not code).
 */
function parse(text) {
  const root = { open: -1, close: text.length, subs: [], ops: [], call: false, kw: null };
  const stack = [root];
  let end = text.length;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    const g = stack[stack.length - 1];
    if (c === '"' || c === "'") {
      const close = literalEnd(text, i);
      if (close >= 0) i = close;
      continue;
    }
    if (c === '/' && text[i + 1] === '/') { end = i; break; }
    if (c === '/' && text[i + 1] === '*') {
      const close = text.indexOf('*/', i + 2);
      i = close < 0 ? text.length : close + 1;
      continue;
    }
    if (c === '(' || c === '[') {
      const { word, start } = wordBefore(text, i);
      const named = c === '(' && !!word && !/^\d/.test(word);
      const prev = g.subs[g.subs.length - 1];
      const viaPointer = c === '(' && text[i - 1] === ')' && prev?.close === i - 1 && text[prev.open + 1] === '*';
      const sub = {
        open: i, close: text.length, subs: [], ops: [], name: viaPointer ? prev.open : start,
        call: viaPointer || (named && !KEYWORDS.has(word) && text[i - 1] !== ' '),
        kw: named && KEYWORDS.has(word) ? word : null,
      };
      g.subs.push(sub);
      stack.push(sub);
    } else if ((c === ')' || c === ']') && stack.length > 1) {
      g.close = i;
      stack.pop();
    } else if (c === ',' || (c === ';' && stack.length > 1)) {
      g.ops.push({ op: c, pre: i, after: i + 1 });
    } else if (c === ' ') {
      for (const op of OPS) {
        if (text.startsWith(op, i + 1) && text[i + 1 + op.length] === ' ') {
          g.ops.push({ op, pre: i, after: i + 1 + op.length });
          i += op.length;
          break;
        }
      }
    }
  }
  while (end > 0 && text[end - 1] === ' ') end--;
  return { root, end };
}

/** Where `text` splits to fit `margin` columns: no breaks when it fits or nothing in it splits. */
export function wrapPlan(text, margin) {
  const breaks = [];
  const drops = new Set();
  if (!text || margin < 8) return { breaks, drops };
  const { root, end } = parse(text);
  if (end <= margin) return { breaks, drops };
  const base = text.length - text.trimStart().length;
  const width = (s, e) => {
    let n = e - s;
    for (const d of drops) if (d >= s && d < e) n--;
    return n;
  };
  const opsIn = (g, s, e) => g.ops.filter((o) => o.pre >= s && o.after <= e);

  /** Lay out `[s, e)`, direct content of group `g`, from column `col`; returns the column after it. */
  function expr(g, s, e, col, tail, cont) {
    if (col + width(s, e) + tail <= margin) return col + width(s, e);
    const own = opsIn(g, s, e);
    for (const op of ['||', '&&']) {
      if (own.some((o) => o.op === op)) return chain(g, s, e, col, tail, op, cont);
    }
    return inner(g, s, e, col, tail, cont);
  }

  /** Are the brackets of `sub` redundant inside an `op` chain? */
  function flattens(sub, op) {
    if (sub.call || sub.kw || sub.close >= text.length || text[sub.open] !== '(') return false;
    const own = opsIn(sub, sub.open + 1, sub.close);
    return own.some((o) => o.op === op) && own.every((o) => o.op === op || precOf(o.op) > PREC[op]);
  }

  /** The operands of the `op` chain in `[s, e)` of `g`, read through redundant brackets. */
  function operands(g, s, e, op, lead, out) {
    let at = s;
    let pre = lead;
    const cut = (to) => {
      const sub = g.subs.find((x) => x.open === at && x.close === to - 1);
      if (sub && flattens(sub, op)) {
        drops.add(sub.open);
        drops.add(sub.close);
        operands(sub, sub.open + 1, sub.close, op, pre, out);
      } else {
        out.push({ g, s: at, e: to, pre });
      }
    };
    for (const o of opsIn(g, s, e)) {
      if (o.op !== op) continue;
      cut(o.pre);
      at = o.after + 1;
      pre = o.pre;
    }
    cut(e);
    return out;
  }

  function chain(g, s, e, col, tail, op, cont) {
    const list = operands(g, s, e, op, null, []);
    let c = col;
    list.forEach((o, k) => {
      if (k) {
        breaks.push({ at: o.pre, eat: 1, indent: cont });
        c = cont + op.length + 1;
      }
      c = expr(o.g, o.s, o.e, c, k === list.length - 1 ? tail : 0, cont);
    });
    return c;
  }

  /** No chain at this level: split the groups inside it, left to right. */
  function inner(g, s, e, col, tail, cont) {
    const subs = g.subs.filter((x) => x.open >= s && x.close < e);
    let c = col;
    let at = s;
    subs.forEach((sub, k) => {
      c += width(at, sub.open + 1);
      const last = k === subs.length - 1;
      c = group(sub, c, width(sub.close, last ? e : subs[k + 1].open + 1) + (last ? tail : 0), cont);
      at = sub.close;
    });
    return c + width(at, e);
  }

  /** Lay out the inside of `sub`, whose content starts at column `c`. */
  function group(sub, c, tail, cont) {
    const s = sub.open + 1;
    const e = sub.close;
    if (c + width(s, e) + tail <= margin) return c + width(s, e);
    const own = opsIn(sub, s, e);
    const commas = own.filter((o) => o.op === ',' || o.op === ';');
    if (!commas.length) {
      if (own.some((o) => o.op === '||' || o.op === '&&')) return expr(sub, s, e, c, tail, sub.kw ? base + 4 : c);
      return inner(sub, s, e, c, tail, cont);
    }
    let indent = c;
    if (sub.call) {
      indent = base === 0 ? 8 : c - 1 - (sub.open - sub.name) + 2;
      breaks.push({ at: s, eat: 0, indent });
    }
    let at = s;
    let col = sub.call ? indent : c;
    for (const o of commas) {
      expr(sub, at, o.pre, col, 1, indent + 4);
      const eat = text[o.after] === ' ' ? 1 : 0;
      breaks.push({ at: o.after, eat, indent });
      at = o.after + eat;
      col = indent;
    }
    return expr(sub, at, e, indent, tail, indent + 4);
  }

  expr(root, 0, end, 0, 0, base + 4);
  breaks.sort((a, b) => a.at - b.at);
  return { breaks, drops };
}

/** `text` as the plan shows it (for tests and plain-text copies). */
export function applyPlan(text, { breaks, drops }) {
  let out = '';
  let b = 0;
  for (let i = 0; i < text.length; i++) {
    if (b < breaks.length && breaks[b].at === i) {
      out += '\n' + ' '.repeat(breaks[b].indent);
      i += breaks[b].eat - 1;
      b++;
      continue;
    }
    if (!drops.has(i)) out += text[i];
  }
  return out;
}
