// tags.js — the short notes kuna puts at the end of a line (`// no-return`,
// `// branch-flip`, `// early-return x10`) in plain words, for the Explain
// panel and the hover card. A note this file does not know is shown as the
// decompiler wrote it. DOM-free.

const TAGS = {
  'no-return': () => 'The function called here never comes back (it ends the program or loops forever), so nothing after the call runs.',
  'jump-as-call': () => 'The machine code jumps into another function here instead of calling it (a tail call); the decompiler shows the jump as a call.',
  'jump-as-return': () => 'A jump through a register leaves the function here, so the decompiler shows it as a return.',
  'tail-call': () => 'The function ends by jumping straight into another one (a tail call); the decompiler shows it as a call followed by a return.',
  'inline-failed': () => 'The decompiler was asked to paste the called function in here, but could not.',
  'writes-rodata': () => 'This writes to memory the program file marks read-only.',
  'branch-flip': () => 'The decompiler flipped this if: the machine code jumps when the opposite is true, and the two branches were swapped so the condition reads the positive way.',
  'return-dupe': (n) => `The machine code shares one return between several paths; the decompiler gave ${n > 1 ? `${n} paths` : 'this path'} a return of its own instead of a goto.`,
  'crossjump-dupe': () => 'The compiler merged identical code from two places into one; the decompiler copied it back so the code reads without a goto.',
  'int3-pad': (n) => `The code runs into ${n > 1 ? `${n} bytes of ` : ''}int3 padding (debug traps) here, so the function probably ends before this point.`,
  outlined: () => 'The compiler moved part of this function into a separate piece of code; the decompiler put it back here.',
  'early-return': (n) => `The decompiler moved ${n > 1 ? `${n} return statements` : 'a return statement'} up to the test that leads to ${n > 1 ? 'them' : 'it'}, so the function exits early instead of jumping to one shared end.`,
  'switch-return': (n) => `${n > 1 ? `${n} switch cases that only return a value were` : 'A switch case that only returns a value was'} turned into a direct return.`,
  'ite-dedupe': (n) => `Code repeated at the end of both the if and the else ${n > 1 ? `(${n} times) ` : ''}was merged into one copy after them.`,
  ternary: (n) => `${n > 1 ? `${n} if/else blocks that only pick a value were` : 'An if/else that only picks a value was'} written as a ?: expression.`,
  'else-flattened': (n) => `${n > 1 ? `${n} else parts were` : 'An else part was'} dropped because the if part always leaves; the code that was in the else now follows the if.`,
  injected: () => 'This call is described by a built-in model of what the function does, not by its machine code.',
  'symbol-size-mismatch': () => 'A symbol here does not match the size the code uses, so the decompiler did not use its name.',
};

const KNOWN = Object.keys(TAGS).join('|');
const SPLIT = new RegExp(`,\\s*(?=(?:jt: )?(?:warn: |inlined: |(?:${KNOWN})(?: x\\d+)?\\s*(?:,|$)))`);

/** Where the end-of-line `//` comment of a C line starts (outside strings), or -1. */
export function commentStart(text) {
  let quote = null;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quote) {
      if (c === '\\') i++;
      else if (c === quote) quote = null;
    } else if (c === '"' || c === "'") {
      quote = c;
    } else if (c === '/' && text[i + 1] === '/') {
      return i;
    }
  }
  return -1;
}

/** The end-of-line comment's text, or null. */
export function trailingComment(text) {
  const at = commentStart(text);
  return at < 0 ? null : text.slice(at + 2).trim();
}

/** One note in words: `{tag, words}`, or null for a note that is not kuna's (a register name, a user note). */
export function explainTag(raw) {
  let slug = raw.trim();
  let prefix = '';
  if (slug.startsWith('jt: ')) {
    slug = slug.slice(4);
    prefix = 'About this switch: ';
  }
  if (slug.startsWith('warn: ')) return { tag: raw, words: `${prefix}A warning from the decompiler: ${slug.slice(6)}` };
  if (slug.startsWith('inlined: ')) return { tag: raw, words: `${prefix}The compiler pasted the code of ${slug.slice(9)} in here (inlined it).` };
  const m = /^([a-z0-9-]+)(?: x(\d+))?$/.exec(slug);
  const say = m && TAGS[m[1]];
  if (!say) return null;
  const words = say(m[2] ? Number(m[2]) : 1);
  return { tag: raw, words: prefix ? prefix + words[0].toLowerCase() + words.slice(1) : words };
}

/** The notes kuna left on `text`, explained; empty when there are none. */
export function lineTags(text) {
  const comment = trailingComment(text || '');
  if (!comment) return [];
  return comment.split(SPLIT).map(explainTag).filter(Boolean);
}
