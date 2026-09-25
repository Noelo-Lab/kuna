// prefs.js — the study view's display preferences, one versioned record in
// localStorage (`kuna.d2.prefs`). DOM-free: the storage object is passed in.
// The choices of the cycling ones are listed in CHOICES; `hoverDelay` -1 is off.

export const PREFS_KEY = 'kuna.d2.prefs';

export const DEFAULT_PREFS = Object.freeze({
  v: 1,
  asmAddr: 'abs',
  asmBytes: true,
  asmCMode: 'comment',
  asmArrows: true,
  asmHints: true,
  cLineNumbers: true,
  cLineAddrs: true,
  hoverDelay: 450,
  split: false,
  tab: 'c',
  rail: true,
});

const CHOICES = {
  asmAddr: ['abs', 'rel', 'both'],
  asmCMode: ['comment', 'interleave', 'off'],
  hoverDelay: [0, 250, 450, 800, -1],
  tab: ['c', 'asm', 'bytes', 'stack', 'src'],
};

/** Keep only known keys with values of the right shape; fill the rest with defaults. */
export function normalizePrefs(raw) {
  const out = { ...DEFAULT_PREFS };
  if (!raw || typeof raw !== 'object' || raw.v !== 1) return out;
  for (const [key, fallback] of Object.entries(DEFAULT_PREFS)) {
    const value = raw[key];
    if (CHOICES[key]) {
      if (CHOICES[key].includes(value)) out[key] = value;
    } else if (typeof value === typeof fallback) {
      out[key] = value;
    }
  }
  return out;
}

export function loadPrefs(storage) {
  try {
    return normalizePrefs(JSON.parse(storage?.getItem(PREFS_KEY) || 'null'));
  } catch (_) {
    return { ...DEFAULT_PREFS };
  }
}

export function savePrefs(storage, prefs) {
  try {
    storage?.setItem(PREFS_KEY, JSON.stringify(normalizePrefs(prefs)));
    return true;
  } catch (_) {
    return false;
  }
}

/** The next value of a cycling preference (`o` cycles the address column). */
export function cycle(prefs, key) {
  const list = CHOICES[key];
  const at = list.indexOf(prefs[key]);
  return { ...prefs, [key]: list[(at + 1) % list.length] };
}
