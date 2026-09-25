// prefs.js — the study view's display preferences, one versioned record in
// localStorage (`kuna.d2.prefs`). DOM-free: the storage object is passed in.
// The choices of the cycling ones are listed in CHOICES; `hoverDelay` -1 is off.
// Version 1 records (the first layout) are migrated: the view and the settings a
// student chose carry over, the columns whose defaults changed take the new ones.

export const PREFS_KEY = 'kuna.d2.prefs';

export const DEFAULT_PREFS = Object.freeze({
  v: 2,
  view: 'c',
  asmAddr: 'abs',
  asmBytes: false,
  asmBytesSplit: false,
  asmCMode: 'heading',
  asmArrows: true,
  asmInfer: true,
  cLineNumbers: true,
  cLineAddrs: false,
  hoverDelay: 450,
  rail: true,
  theme: 'system',
  tipSeen: false,
});

const CHOICES = {
  view: ['c', 'split', 'asm', 'bytes', 'stack'],
  asmAddr: ['abs', 'rel', 'both'],
  asmCMode: ['heading', 'comment', 'off'],
  hoverDelay: [0, 250, 450, 800, -1],
  theme: ['system', 'light', 'dark'],
};

function pick(out, raw, key) {
  const value = raw[key];
  if (CHOICES[key]) {
    if (CHOICES[key].includes(value)) out[key] = value;
  } else if (typeof value === typeof DEFAULT_PREFS[key]) {
    out[key] = value;
  }
}

function fromV1(raw) {
  const out = { ...DEFAULT_PREFS };
  for (const key of ['asmAddr', 'asmArrows', 'asmInfer', 'cLineNumbers', 'hoverDelay', 'rail']) pick(out, raw, key);
  const tab = ['c', 'asm', 'bytes', 'stack'].includes(raw.tab) ? raw.tab : 'c';
  out.view = raw.split === true && (tab === 'c' || tab === 'asm') ? 'split' : tab;
  return out;
}

/** Keep only known keys with values of the right shape; fill the rest with defaults. */
export function normalizePrefs(raw) {
  if (!raw || typeof raw !== 'object') return { ...DEFAULT_PREFS };
  if (raw.v === 1) return fromV1(raw);
  const out = { ...DEFAULT_PREFS };
  if (raw.v !== 2) return out;
  if (raw.asmCMode === 'interleave') raw = { ...raw, asmCMode: 'heading' };
  for (const key of Object.keys(DEFAULT_PREFS)) if (key !== 'v') pick(out, raw, key);
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
