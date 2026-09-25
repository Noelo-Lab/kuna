// app.js — the /decompile2 study view: loads a program into the decompiler
// Worker, lists its functions in plain groups, and shows one function as
// linked views (C code, side by side, assembly, bytes, stack) with an Explain
// panel. Every engine string is escaped by the pure renderers this module
// mounts; this file owns the DOM, the operation model and the keyboard.
import {
  KunaWorkerCancelledError,
  KunaWorkerClient,
} from '../kuna-worker-client.js';
import { escapeHtml, highlightC } from '../assets/js/highlight-c.js';
import { compileQuery, searchKey } from '../assets/js/fnfilter.js';
import {
  normalizeInspect,
  lineSegments,
  buildIndex,
  renderC,
  localDecls,
  changedLines,
} from './render-c.js';
import { loadPrefs, savePrefs, cycle, DEFAULT_PREFS } from './prefs.js';
import { groupFunctions, groupOf, firstFunction } from './groups.js';
import { renderAsm, renderInsnRows, formatAddr, spacedBytes, inferLines, spellInsn } from './asm-view.js';
import { createHover } from './hover.js';
import { createSync } from './sync.js';
import { Session, cliCommand } from './session.js';
import {
  validateIdent, validateCType, parseSignature, parseRustSignature, buildPrototype, typeSize, knownTypes, normalizeType,
} from './ctype.js';
import { hashBytes, SessionStore } from './persist.js';
import { entryOffset, bare } from './addr.js';
import { createDialogs } from './dialogs.js';
import { createRail } from './rail.js';
import {
  functionCells, hexRows, renderHex, applyPatches, parseHex, patchedName, fileOffsetFor, originalByteAt,
} from './bytes-view.js';
import { archFrom, nopFill } from './arch.js';
import { explain, idioms } from './mnemonics.js';
import { frameModel, renderFrame, slotIndex } from './stack-frame.js';
import { renderXrefs, renderLocalCalls, localCallees } from './xrefs-view.js';
import { helpHtml } from './help.js';

const $ = (id) => document.getElementById(id);
const els = {
  status: $('status'), cancel: $('cancelbtn'), dl: $('dlbtn'), patch: $('patchbtn'), patchWhy: $('patchwhy'),
  mode: $('mode'), lang: $('lang'), example: $('examplebtn'), pick: $('pick'), file: $('file'),
  help: $('helpbtn'), keys: $('keysbtn'), more: $('morebtn'), moreMenu: $('moremenu'), theme: $('themebtn'),
  crumb: $('crumb'), crumbName: $('crumbname'), crumbCount: $('crumbcount'), fnsBtn: $('fnsbtn'),
  progress: $('progress'), work: $('work'), codearea: $('codearea'), tip: $('tip'), tipBtn: $('tipbtn'),
  narrow: $('narrownote'), list: $('fnlist'), filter: $('fnfilter'), none: $('fnnone'),
  empty: $('empty'), drop: $('dropzone'), dropVeil: $('dropveil'),
  welcomeOpen: $('welcomeopen'), welcomeExample: $('welcomeexample'),
  vhead: $('vhead'), vname: $('vname'), vmeta: $('vmeta'), explain: $('explainbtn'),
  back: $('backbtn'), fwd: $('fwdbtn'), fnRename: $('fnrenamebtn'), proto: $('protobtn'),
  tabbar: $('tabbar'), tabs: $('tabs'), split: $('splitbtn'), viewBtn: $('viewbtn'), viewMenu: $('viewmenu'),
  panes: $('panes'), ccode: $('ccode'), asmcode: $('asmcode'), bytesbar: $('bytesbar'),
  hexdump: $('hexdump'), stackframe: $('stackframe'), srccode: $('srccode'), hint: $('hint'),
  rail: $('rail'), railBody: $('railbody'), railBtn: $('railbtn'),
  card: $('d2card'), pop: $('d2pop'), toasts: $('d2toasts'), helpDialog: $('help'),
};

const storage = (() => {
  try { return window.localStorage; } catch (_) { return null; }
})();

const state = {
  kuna: null,
  prefs: loadPrefs(storage),
  binary: null,
  inventory: null,
  rows: [],
  groups: {},
  byAddr: new Map(),
  byName: new Map(),
  current: null,
  cache: new Map(),
  caps: { inspect: null, assert: null },
  exampleSource: null,
  hist: { index: 0, max: 0 },
  view: 'c',
  rendered: new Set(),
};
const CACHE_MAX = 32;

// ── status, progress, toasts ────────────────────────────────────────────────

/** The status bar: a plain sentence; details such as timings go in its tooltip. */
function setStatus(msg, cls = 'busy', title = '') {
  els.status.className = 'status ' + cls;
  els.status.innerHTML = '<span class="dot"></span>' + escapeHtml(msg);
  if (title) els.status.title = title;
  else els.status.removeAttribute('title');
}

const dialogs = createDialogs({ pop: els.pop, toasts: els.toasts });
const toast = dialogs.toast;

// ── the operation model: one engine request at a time ──────────────────────

let active = null;
let opSeq = 0;
const idleQueue = [];

function syncButtons() {
  els.cancel.disabled = !active;
  els.cancel.hidden = !active;
  els.dl.disabled = !state.inventory || !!active;
  els.progress.hidden = !(active && (active.kind === 'edit' || active.kind === 'function'));
}

function beginOperation(kind, { clearSession = false } = {}) {
  if (active) {
    const prev = active;
    active = null;
    if (clearSession) state.kuna.clear('superseded by a new binary');
    else state.kuna.cancel('superseded by another operation');
    prev.onCancel?.();
    if (prev.kind === 'project' && kind !== 'project') toast('The C code download was stopped. Start it again from the ⋯ menu.', { kind: 'warn' });
  }
  active = { id: ++opSeq, kind };
  syncButtons();
  return active;
}

const isCurrent = (op) => active?.id === op.id;

function finishOperation(op) {
  if (!isCurrent(op)) return false;
  active = null;
  syncButtons();
  queueMicrotask(drainIdle);
  return true;
}

/** Run `task` once no engine request is in flight (xrefs, reads). */
function whenIdle(task) {
  if (!active) task();
  else idleQueue.push(task);
}

function drainIdle() {
  while (!active && idleQueue.length) idleQueue.shift()();
}

els.cancel.addEventListener('click', () => {
  if (!active || !state.kuna) return;
  const cancelled = active;
  active = null;
  opSeq++;
  if (cancelled.kind === 'load') {
    state.kuna.clear('cancelled by user');
    resetBinary();
  } else {
    state.kuna.cancel('cancelled by user');
  }
  cancelled.onCancel?.();
  idleQueue.length = 0;
  syncButtons();
  setStatus(state.inventory ? `Stopped. ${state.binary.name} is still open` : 'Stopped', state.inventory ? 'ok' : '');
});

// ── engine calls with graceful degradation ─────────────────────────────────

/**
 * What the engine can do, read off the inventory document itself: the
 * study-view engine's `list` carries `sections` and `target`, and the same
 * build answers `inspect`, `read` and `--assert`.
 */
function capsFrom(doc) {
  const modern = Array.isArray(doc?.sections) && !!doc?.target;
  return { inspect: modern, assert: modern };
}

/** The first line of what a failed request said. */
function errorLine(e) {
  return String(e?.detail?.stderr || e?.message || e).split('\n')[0].replace(/^error:\s*/, '');
}

/**
 * The sent directive a failed request names as unparseable, or null: the
 * engine's documented `error: --assert "<directive>": <why>`, matched against
 * the exact directives this request carried.
 */
function refusedDirective(e, sent) {
  if (!(e?.detail?.exitCode > 0)) return null;
  const text = e.detail.stderr || '';
  return sent.find((d) => text.includes(`--assert ${JSON.stringify(d)}:`)) || null;
}

/**
 * Run `call(directives)`; a directive the engine cannot parse is marked
 * refused (the rail says why and it is no longer sent) and the call is
 * retried without it.
 */
async function withDirectives(call, directives) {
  let list = directives;
  for (;;) {
    try {
      return await call(list);
    } catch (e) {
      const bad = e instanceof KunaWorkerCancelledError ? null : refusedDirective(e, list);
      if (!bad) throw e;
      session.markRefused(bad, errorLine(e));
      toast('One of your changes could not be read, so it is no longer applied.', { kind: 'err', detail: `${bad} — ${errorLine(e)}` });
      list = list.filter((d) => d !== bad);
      renderRail();
    }
  }
}

/** Directives for one function; empty when the engine cannot take them. */
function directivesFor(addrHex) {
  return state.caps.assert ? session.assertionsFor(addrHex) : [];
}

/** Inspect one function, or plainly decompile it on an engine without `inspect`. */
async function fetchFunction(fn) {
  return withDirectives(
    (list) => (state.caps.inspect
      ? state.kuna.inspect(fn.address_hex, { assertions: list })
      : state.kuna.decompile(fn.address_hex, { assertions: list })),
    directivesFor(fn.address_hex),
  );
}

// ── the session: the student's edits as --assert directives ───────────────

let session = new Session();
const store = new SessionStore(storage);

/** A function's name as the student sees it (their rename, else the engine's). */
function displayName(fn) {
  return session.functionName(fn.address_hex) || fn.name;
}

const nameOfAddr = (addrHex) => displayName(state.byAddr.get(addrHex) || { address_hex: addrHex, name: addrHex });

function persist() {
  const hash = state.binary?.hash;
  if (!hash) return;
  if (session.size) store.save(hash, state.binary.name, JSON.stringify(session.toJSON()));
  else store.remove(hash);
}

// ── startup ────────────────────────────────────────────────────────────────

try {
  state.kuna = new KunaWorkerClient({ wasmUrl: '../kuna_wasm.wasm', specRoot: '../specs' });
  await state.kuna.ready();
  setStatus('Ready. Open a program or try an example', 'ok');
  els.pick.removeAttribute('aria-disabled');
  els.example.disabled = false;
  els.welcomeOpen.disabled = false;
  els.welcomeExample.disabled = false;
} catch (e) {
  setStatus('The decompiler could not start', 'err', e.message);
  toast('The decompiler could not start.', { kind: 'err', detail: e.message });
  console.error(e);
}

// ── inventory and sidebar ──────────────────────────────────────────────────

const isStub = (fn) => fn.kind === 'plt' || fn.kind === 'thunk';

const GROUPS = [
  ['program', 'Your program', true],
  ['startup', 'Startup & runtime', false],
  ['imports', 'Imported functions', false],
];

function resetList() {
  els.list.innerHTML = '';
  state.rows = [];
  state.groups = {};
  els.filter.value = '';
  els.filter.disabled = true;
  els.filter.classList.remove('bad');
  els.filter.removeAttribute('title');
  els.none.hidden = true;
}

function resetBinary() {
  state.inventory = null;
  state.binary = null;
  state.current = null;
  state.cache.clear();
  state.byAddr.clear();
  state.byName.clear();
  resetList();
  showEmpty(true);
  syncButtons();
}

/** The welcome screen (nothing loaded) or the function views. */
function showEmpty(on) {
  els.empty.hidden = !on;
  els.vhead.hidden = on;
  els.tabbar.hidden = on;
  els.codearea.hidden = on;
  els.panes.hidden = on;
  els.work.classList.toggle('welcome', on && !state.inventory);
  if (on && !state.inventory) {
    els.crumb.hidden = true;
    els.work.classList.remove('drawer', 'fns');
  }
  syncLayout();
}

/**
 * Filter the sidebar. A group with matches opens while a search is typed and
 * goes back to how the student left it when the search is cleared.
 */
function applyFilter() {
  const query = compileQuery(els.filter.value);
  let any = false;
  for (const g of Object.values(state.groups)) g.shown = 0;
  for (const entry of state.rows) {
    const hit = query.test(entry.key);
    entry.row.hidden = !hit;
    if (hit) { any = true; state.groups[entry.group].shown++; }
  }
  for (const g of Object.values(state.groups)) {
    g.count.textContent = query.empty ? `(${g.total})` : `(${g.shown} of ${g.total})`;
    if (query.empty) {
      if (g.userOpen !== null) { g.el.open = g.userOpen; g.userOpen = null; }
      g.el.hidden = g.total === 0;
    } else {
      if (g.userOpen === null) g.userOpen = g.el.open;
      g.el.open = g.shown > 0;
      g.el.hidden = g.shown === 0;
    }
  }
  els.filter.classList.toggle('bad', !!query.error);
  if (query.error) els.filter.title = query.error;
  else els.filter.removeAttribute('title');
  els.none.textContent = query.error ? `That search is not valid: ${query.error}` : 'No functions match';
  els.none.hidden = state.rows.length === 0 || any;
}

const visibleRows = () => state.rows.filter((entry) => !entry.row.hidden && state.groups[entry.group].el.open);

els.filter.addEventListener('input', applyFilter);
els.filter.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    e.preventDefault();
    if (els.filter.value) { els.filter.value = ''; applyFilter(); } else els.filter.blur();
  } else if (e.key === 'Enter') {
    e.preventDefault();
    visibleRows()[0]?.row.click();
  } else if (e.key === 'ArrowDown') {
    e.preventDefault();
    visibleRows()[0]?.row.focus();
  }
});

/** Sidebar rows show the session's names (a renamed function reads as renamed). */
function refreshRowNames() {
  for (const entry of state.rows) {
    const shown = displayName(entry.fn);
    entry.row.querySelector('.nm').textContent = shown;
    entry.row.classList.toggle('renamed', shown !== entry.fn.name);
    entry.row.title = shown === entry.fn.name ? entry.fn.address_hex : `${entry.fn.address_hex} · first called ${entry.fn.name}`;
    entry.key = searchKey({ ...entry.fn, aliases: [...(entry.fn.aliases || []), shown] });
  }
}

function buildSidebar(functions) {
  const grouped = groupFunctions(functions);
  const frag = document.createDocumentFragment();
  for (const [id, title, open] of GROUPS) {
    const details = document.createElement('details');
    details.className = 'd2-group';
    details.dataset.group = id;
    details.open = open;
    details.innerHTML = `<summary>${escapeHtml(title)} <span class="d2-count"></span></summary>`;
    state.groups[id] = { el: details, count: details.querySelector('.d2-count'), total: grouped[id].length, shown: 0, userOpen: null };
    for (const fn of grouped[id]) {
      const row = document.createElement('div');
      row.className = 'fn' + (isStub(fn) ? ' stub' : '') + (fn.error ? ' bad' : '');
      row.tabIndex = 0;
      row.setAttribute('role', 'button');
      row.dataset.addr = fn.address_hex;
      row.innerHTML = `<span class="nm">${escapeHtml(fn.name)}</span>`;
      row.addEventListener('click', () => openFunction(fn));
      row.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openFunction(fn); return; }
        if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp') return;
        e.preventDefault();
        const shown = visibleRows();
        const at = shown.findIndex((entry) => entry.row === row);
        shown[at + (e.key === 'ArrowDown' ? 1 : -1)]?.row.focus();
      });
      details.appendChild(row);
      state.rows.push({ row, fn, key: searchKey(fn), stub: isStub(fn), group: id });
    }
    frag.appendChild(details);
  }
  els.list.appendChild(frag);
  els.filter.disabled = false;
  state.firstFn = firstFunction(grouped);
  refreshRowNames();
  applyFilter();
}

function markSelectedRow(addrHex) {
  for (const entry of state.rows) {
    const on = entry.fn.address_hex === addrHex;
    entry.row.classList.toggle('sel', on);
    if (on) {
      const group = state.groups[entry.group];
      if (!group.el.open) group.el.open = true;
      entry.row.scrollIntoView({ block: 'nearest' });
    }
  }
}

/**
 * Load `source` (a File, or `{name, bytes}`) and list its functions. `keep`
 * reopens the function that was showing (a mode/language change).
 */
async function indexBinary(source, { example = false, keep = null } = {}) {
  if (!source || !state.kuna) return;
  const op = beginOperation('load', { clearSession: true });
  const name = source.name || 'binary';
  setStatus(`Opening ${name}…`);
  resetList();
  showEmpty(true);
  state.inventory = null;
  state.current = null;
  state.cache.clear();
  let bytes;
  let inventory;
  const t0 = performance.now();
  try {
    bytes = source.bytes || new Uint8Array(await source.arrayBuffer());
    const hash = await hashBytes(bytes);
    if (!isCurrent(op)) return;
    if (hash !== state.binary?.hash) {
      session = restoreSession(hash);
      state.restored = session.size ? { count: session.size, mark: session.mark(), toasted: false } : null;
    } else {
      state.restored = null;
    }
    state.binary = { name, bytes, example, format: null, hash };
    setStatus(`Finding the functions in ${name}…`, 'busy', `${bytes.length.toLocaleString()} bytes`);
    const load = (assertions) => state.kuna.load(bytes, {
      fileName: name, mode: els.mode.value, language: els.lang.value, assertions,
    });
    const globals = listAssertions();
    try {
      inventory = await withDirectives(load, globals);
    } catch (e) {
      if (!globals.length || e instanceof KunaWorkerCancelledError || !isCurrent(op)) throw e;
      inventory = await load([]);
      if (capsFrom(inventory).assert) {
        toast(`Opened ${name} without ${globals.length} of your saved change${globals.length === 1 ? '' : 's'}.`, {
          kind: 'warn', detail: `${errorLine(e)} — ${globals.join(' · ')}`,
        });
      }
    }
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    setStatus(`Could not open ${name}`, 'err', e.message);
    toast(`Could not open ${name}.`, { kind: 'err', detail: errorLine(e) });
    console.error(e);
    renderRail();
    finishOperation(op);
    return;
  }
  if (!isCurrent(op)) return;
  state.caps = capsFrom(inventory);
  if (!state.caps.assert && session.size) {
    toast('This version of the decompiler cannot apply changes.', {
      kind: 'warn', detail: 'Your changes are kept and can be exported.',
    });
  }
  state.binary.format = inventory.format;
  state.inventory = inventory;
  session.recordOutcomes(inventory.assertions);
  syncPatchButton();
  state.byAddr.clear();
  state.byName.clear();
  for (const fn of inventory.functions) {
    fn.address_hex = fn.address_hex.toLowerCase();
    state.byAddr.set(fn.address_hex, fn);
    state.byName.set(fn.name, fn);
    for (const alias of fn.aliases || []) if (!state.byName.has(alias)) state.byName.set(alias, fn);
  }
  buildSidebar(inventory.functions);
  $('tab-src').hidden = !(example && state.exampleSource);
  els.crumb.hidden = false;
  els.crumbName.textContent = name;
  els.crumbCount.textContent = `${inventory.functions.length} function${inventory.functions.length === 1 ? '' : 's'}`;
  els.work.classList.remove('welcome');
  els.rail.hidden = false;
  syncLayout();
  renderRail();
  if (state.restored && !state.restored.toasted) {
    state.restored.toasted = true;
    toast(`Restored ${state.restored.count} change${state.restored.count === 1 ? '' : 's'} you made to ${name} before.`);
  }
  const dt = Math.round(performance.now() - t0);
  setStatus(`Opened ${name}`, 'ok', `${inventory.format} · ${inventory.functions.length} functions found in ${dt} ms`);
  finishOperation(op);
  const fromHash = state.byAddr.get((location.hash || '').slice(1).toLowerCase());
  const first = (keep && state.byAddr.get(keep)) || fromHash || state.firstFn;
  if (first) openFunction(first, { replace: true });
}

function restoreSession(hash) {
  try {
    const text = store.load(hash);
    return text ? Session.fromJSON(JSON.parse(text)) : new Session();
  } catch (_) {
    return new Session();
  }
}

/**
 * The directives `list` gets: the global ones except function renames, which
 * the sidebar overlays itself so the inventory keeps the engine's own names.
 */
function listAssertions() {
  if (state.caps.assert === false || !session.size) return [];
  return session.globalAssertions().filter((d) => !d.startsWith('function '));
}

/** Read a picked or dropped file, then open it. Reading comes first: see the input handler. */
async function openFile(f) {
  if (!f || !state.kuna) return;
  try {
    const bytes = new Uint8Array(await f.arrayBuffer());
    indexBinary({ name: f.name, bytes });
  } catch (e) {
    setStatus(`Could not read ${f.name}`, 'err', e.message);
  }
}

// Read before clearing the input: current Chrome returns an empty buffer for a read still pending when its input is cleared.
els.file.addEventListener('change', async () => {
  const f = els.file.files[0];
  if (!f) return;
  try {
    await openFile(f);
  } finally {
    els.file.value = '';
  }
});
els.welcomeOpen.addEventListener('click', () => els.file.click());

let dragDepth = 0;
const hasFiles = (e) => [...(e.dataTransfer?.types || [])].includes('Files');
window.addEventListener('dragenter', (e) => {
  if (!hasFiles(e)) return;
  e.preventDefault();
  if (dragDepth++ === 0) {
    els.dropVeil.hidden = false;
    els.drop.classList.add('over');
  }
});
window.addEventListener('dragover', (e) => {
  if (!hasFiles(e)) return;
  e.preventDefault();
  e.dataTransfer.dropEffect = 'copy';
});
window.addEventListener('dragleave', (e) => {
  if (!hasFiles(e)) return;
  if (--dragDepth <= 0) {
    dragDepth = 0;
    els.dropVeil.hidden = true;
    els.drop.classList.remove('over');
  }
});
window.addEventListener('drop', (e) => {
  if (!hasFiles(e)) return;
  e.preventDefault();
  dragDepth = 0;
  els.dropVeil.hidden = true;
  els.drop.classList.remove('over');
  const f = e.dataTransfer.files[0];
  if (f && !els.pick.hasAttribute('aria-disabled')) openFile(f);
});

const reindex = () => {
  if (state.binary) indexBinary(state.binary, { example: state.binary.example, keep: state.current?.fn.address_hex });
};
els.mode.addEventListener('change', reindex);
els.lang.addEventListener('change', reindex);

async function openExample() {
  els.example.disabled = true;
  try {
    const [elf, src] = await Promise.all([
      fetch('./examples/sample.elf').then((r) => {
        if (!r.ok) throw new Error(`the example is not in this build (${r.status})`);
        return r.arrayBuffer();
      }),
      fetch('./examples/sample.c').then((r) => (r.ok ? r.text() : null)).catch(() => null),
    ]);
    state.exampleSource = src;
    await indexBinary({ name: 'sample.elf', bytes: new Uint8Array(elf) }, { example: true });
  } catch (e) {
    toast('Could not load the example.', { kind: 'err', detail: e.message });
  } finally {
    els.example.disabled = false;
  }
}
els.example.addEventListener('click', openExample);
els.welcomeExample.addEventListener('click', openExample);

// ── opening a function ─────────────────────────────────────────────────────

/** A function's body depends on its own directives and the global ones, so they are the cache key. */
function cacheKey(addrHex) {
  return `${addrHex}\n${directivesFor(addrHex).join('\n')}`;
}

/** The cached body for the session as it is now, without touching the LRU order. */
function cachedBody(addrHex) {
  return state.cache.get(cacheKey(addrHex)) || null;
}

function cacheGet(addrHex) {
  const key = cacheKey(addrHex);
  const hit = state.cache.get(key);
  if (hit) { state.cache.delete(key); state.cache.set(key, hit); }
  return hit;
}

function cacheSet(addrHex, data) {
  const key = cacheKey(addrHex);
  state.cache.delete(key);
  state.cache.set(key, data);
  while (state.cache.size > CACHE_MAX) state.cache.delete(state.cache.keys().next().value);
}

function pushHistory(addrHex, replace) {
  if (replace) {
    history.replaceState({ d2: state.hist.index, addr: addrHex }, '', '#' + addrHex);
  } else {
    state.hist.index++;
    state.hist.max = state.hist.index;
    history.pushState({ d2: state.hist.index, addr: addrHex }, '', '#' + addrHex);
  }
  syncHistoryButtons();
}

function syncHistoryButtons() {
  els.back.disabled = state.hist.index <= 0;
  els.fwd.disabled = state.hist.index >= state.hist.max;
  els.back.hidden = state.hist.max === 0;
  els.fwd.hidden = state.hist.max === 0;
}

window.addEventListener('popstate', (e) => {
  const addr = e.state?.addr || (location.hash || '').slice(1).toLowerCase();
  if (Number.isInteger(e.state?.d2)) state.hist.index = e.state.d2;
  syncHistoryButtons();
  const fn = state.byAddr.get(addr);
  if (fn && fn.address_hex !== state.current?.fn.address_hex) openFunction(fn, { push: false });
});
els.back.addEventListener('click', () => history.back());
els.fwd.addEventListener('click', () => history.forward());

/**
 * Show `fn`: from the cache when its body is known, else by asking the engine.
 * `push`/`replace` choose the history entry; `keepView` leaves the previous
 * render visible while the request runs; `focusAddr` selects an instruction.
 */
async function openFunction(fn, { push = true, replace = false, keepView = false, focusAddr = null } = {}) {
  if (!fn || !state.inventory) return;
  if (bytesState.burst) flushBytes({ send: false });
  bytesState.edit = null;
  state.dataView = null;
  if (push || replace) pushHistory(fn.address_hex, replace);
  markSelectedRow(fn.address_hex);
  showEmpty(false);
  const cached = cacheGet(fn.address_hex);
  if (cached) {
    if (active?.kind === 'function') {
      active = null;
      opSeq++;
      state.kuna.cancel('superseded by a cached function');
      syncButtons();
    }
    showFunction(fn, cached, { focusAddr });
    if (!active) setStatus(cached.error ? `Could not decompile ${displayName(fn)}` : `Showing ${displayName(fn)}`, cached.error ? 'err' : 'ok');
    return;
  }
  const op = beginOperation('function');
  const label = displayName(fn);
  setStatus(`Decompiling ${label}…`);
  if (!keepView) {
    state.current = null;
    els.vname.textContent = label;
    els.vmeta.textContent = '';
    els.vmeta.title = fn.address_hex;
    els.ccode.innerHTML = `<div class="d2note">Decompiling ${escapeHtml(label)}…</div>`;
    for (const pane of [els.asmcode, els.hexdump, els.stackframe]) pane.innerHTML = '';
  }
  const t0 = performance.now();
  try {
    const doc = await fetchFunction(fn);
    if (!isCurrent(op)) return;
    const data = normalizeInspect(doc);
    session.recordOutcomes(data.assertions);
    cacheSet(fn.address_hex, data);
    showFunction(fn, data, { focusAddr });
    const dt = Math.round(performance.now() - t0);
    setStatus(data.error ? `Could not decompile ${label}` : `Showing ${label}`, data.error ? 'err' : 'ok',
      `${fn.address_hex} · decompiled in ${dt} ms`);
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    els.ccode.innerHTML = `<div class="d2note">${escapeHtml(`Could not decompile ${label}: ${errorLine(e)}`)}</div>`;
    setStatus(`Could not decompile ${label}`, 'err', e.message);
    console.error(e);
  } finally {
    finishOperation(op);
  }
}

// ── rendering ──────────────────────────────────────────────────────────────

/** The header's one muted line: the signature, then sizes in words; the address in its tooltip. */
function fnMeta(fn, data) {
  const parts = [];
  if (data.proto) parts.push(`<code>${escapeHtml(data.proto)}</code>`);
  if (data.hasInstructions) parts.push(`${data.instructions.length} instruction${data.instructions.length === 1 ? '' : 's'}`);
  if (data.size) parts.push(`${data.size} bytes`);
  if (isStub(fn)) parts.push(fn.kind === 'plt' ? 'imported from a library' : 'a jump to another function');
  if (data.error) parts.push('could not be decompiled');
  return parts.join(' · ');
}

function showFunction(fn, data, { focusAddr = null, keep = false } = {}) {
  const { segs } = lineSegments(data);
  const arch = archFrom(data.target || state.inventory?.target, data.instructions);
  const inferred = state.prefs.asmInfer && data.hasInstructions ? inferLines(data.instructions, arch.family || 'x86') : null;
  const index = buildIndex(data, segs, { inferred });
  const frame = data.hasInstructions ? frameModel(data, arch) : null;
  if (frame?.supported) Object.assign(index, slotIndex(frame));
  state.current = {
    fn, data, segs, index, arch, frame, inferred,
    decls: localDecls(data.code),
    codeLines: data.code.split('\n'),
    rust: /rust/i.test(data.language || ''),
    hints: idioms(data.instructions, arch.family || 'x86', { nameAt: (hex) => state.byAddr.get(hex)?.name || null }),
  };
  state.rendered.clear();
  if (!keep) state.sel = null;
  state.cursor = null;
  hover.hide();
  sync.setIndex(index);
  els.vname.textContent = displayName(fn);
  els.vname.title = fn.address_hex;
  els.vmeta.innerHTML = fnMeta(fn, data);
  els.vmeta.title = `${fn.address_hex}${data.size ? ` · ${data.size} bytes` : ''}`;
  const row = state.rows.find((r) => r.fn === fn)?.row;
  row?.classList.toggle('bad', !!data.error);
  els.tip.hidden = state.prefs.tipSeen;
  renderVisible();
  setHint(null);
  renderRail();
  if (state.refsOpen) loadRefs();
  if (focusAddr) selectTarget({ addr: focusAddr }, null);
}

const DEFAULT_HINT = 'Tip: hover a line to see its assembly';

/** The status bar's right side: what is selected and which keys act on it. */
function setHint(html) {
  els.hint.innerHTML = html || DEFAULT_HINT;
}

els.tipBtn.addEventListener('click', () => {
  updatePrefs({ tipSeen: true });
  els.tip.hidden = true;
});

function paneFor(tab) {
  return { c: 'pane-c', asm: 'pane-asm', bytes: 'pane-bytes', stack: 'pane-stack', src: 'pane-src' }[tab];
}

const narrowView = window.matchMedia('(max-width: 899px)');

/** The panes on screen: C and Assembly for Side by side (one at a time on a narrow window), else the view's own. */
function visibleTabs() {
  if (state.view !== 'split') return [state.view];
  return narrowView.matches ? ['c'] : ['c', 'asm'];
}

function renderVisible() {
  if (!state.current) return;
  for (const tab of visibleTabs()) {
    if (state.rendered.has(tab)) continue;
    state.rendered.add(tab);
    RENDER[tab]?.();
  }
}

function needsInspect(what) {
  return `<div class="d2note">${escapeHtml(what)} needs a newer version of the decompiler than this page was built with. ` +
    'The C code works; this view fills in with the newer version.</div>';
}

const RENDER = {
  c() {
    const { data, segs, index } = state.current;
    els.ccode.innerHTML = renderC(data, {
      index,
      segs,
      fnByName: state.byName,
      globalsByName: new Map((data.globals || []).map((g) => [g.name, g.address_hex])),
    });
    els.ccode.classList.toggle('no-addrs', index.lineToInsns.size === 0);
    applyPaneClasses();
    sync.refresh('c');
  },
  asm() {
    const { data } = state.current;
    if (!data.hasInstructions) {
      els.asmcode.innerHTML = needsInspect('The assembly view');
      return;
    }
    els.asmcode.innerHTML = renderAsm(data, asmContext(data));
    applyPaneClasses();
    sync.refresh('asm');
    state.asmRendered = inSplit() ? 'split' : 'asm';
  },
  bytes() {
    const { data } = state.current;
    if (!data.hasInstructions && !state.dataView) {
      els.bytesbar.innerHTML = '';
      els.hexdump.innerHTML = needsInspect('The bytes view');
      return;
    }
    renderBytes();
  },
  stack() {
    const { frame } = state.current;
    if (!frame) {
      els.stackframe.innerHTML = needsInspect('The stack-frame diagram');
      return;
    }
    els.stackframe.innerHTML = renderFrame(frame, { selectedSym: state.sel?.sym || null });
    sync.refresh('stack');
  },
  src() {
    els.srccode.innerHTML = state.exampleSource ? highlightC(state.exampleSource) : '';
  },
};

function applyPaneClasses() {
  const p = state.prefs;
  els.ccode.classList.toggle('no-ln', !p.cLineNumbers);
  els.ccode.classList.toggle('no-la', !p.cLineAddrs);
  els.asmcode.classList.toggle('addr-rel', p.asmAddr === 'rel');
  els.asmcode.classList.toggle('addr-both', p.asmAddr === 'both');
  els.asmcode.classList.toggle('no-bytes', inSplit() ? !p.asmBytesSplit : !p.asmBytes);
  els.asmcode.classList.toggle('no-ccom', inSplit() || p.asmCMode === 'off');
}

function inSplit() {
  return visibleTabs().length > 1;
}

/**
 * How the assembly pane shows the C: headings (or comments) in the Assembly
 * view, nothing beside the C in Side by side (the bands carry the link).
 */
function asmContext(data) {
  const cur = state.current;
  return {
    prefs: { ...state.prefs, asmCMode: inSplit() ? 'off' : state.prefs.asmCMode },
    codeLines: cur.codeLines,
    fnByAddr: state.byAddr,
    nameOf: displayName,
    patched: patchedInsns(data),
    slotOf: cur.frame?.supported ? (reg, disp) => cur.frame.slotOf(reg, disp) : null,
    inferred: cur.inferred,
  };
}

/** Instruction addresses with at least one patched byte. */
function patchedInsns(data) {
  const out = new Set();
  if (!session.bytes.size) return out;
  for (const insn of data.instructions) {
    const base = BigInt(insn.address_hex);
    for (let k = 0; k < insn.size; k++) {
      if (session.bytes.has(base + BigInt(k))) { out.add(insn.address_hex); break; }
    }
  }
  return out;
}

/** Re-render panes whose HTML depends on `prefs` (not just their classes). */
function rerender(...tabs) {
  for (const tab of tabs) state.rendered.delete(tab);
  renderVisible();
}

// ── linked selection and the hover card ────────────────────────────────────

const sync = createSync();
const isShown = (tab) => visibleTabs().includes(tab);

/** A pane as sync sees it: which elements a target marks, and how to reveal it. */
function markerView(elementsFor, reveal) {
  const marked = new Map();
  return {
    mark(sets, cls, target) {
      for (const el of marked.get(cls) || []) el.classList.remove(cls);
      const list = sets ? elementsFor(sets, cls, target) : [];
      for (const el of list) el.classList.add(cls);
      marked.set(cls, list);
    },
    reveal,
  };
}

sync.register('c', markerView((sets, cls, target) => {
  const out = [];
  if (!target?.sym || cls === 'hl-hover') {
    for (const line of sets.lines) { const el = $('c-L' + line); if (el) out.push(el); }
  }
  if (cls === 'hl-sel') {
    for (const sym of sets.syms) out.push(...els.ccode.querySelectorAll(`.t[data-sym="${CSS.escape(sym)}"]`));
  }
  return out;
}, (target, sets) => {
  if (!isShown('c') || !sets.lines.size) return;
  $('c-L' + Math.min(...sets.lines))?.scrollIntoView({ block: 'nearest' });
}));

sync.register('asm', markerView((sets) => {
  const out = [];
  for (const a of sets.addrs) { const el = $('a-' + a); if (el) out.push(el); }
  return out;
}, (target, sets) => {
  if (!isShown('asm') || !sets.addrs.size) return;
  const index = state.current?.index;
  const first = [...sets.addrs].sort((a, b) => (index?.insnIndex.get(a) ?? 0) - (index?.insnIndex.get(b) ?? 0))[0];
  $('a-' + first)?.scrollIntoView({ block: 'nearest' });
}));

sync.register('bytes', markerView((sets) => {
  const index = state.current?.index;
  if (!index || !isShown('bytes')) return [];
  const out = [];
  for (const a of sets.addrs) {
    const i = index.insnIndex.get(a);
    if (i !== undefined) out.push(...els.hexdump.querySelectorAll(`.hb[data-i="${i}"]`));
  }
  return out;
}, (target, sets) => {
  if (!isShown('bytes') || !sets.addrs.size) return;
  const i = state.current.index.insnIndex.get([...sets.addrs][0]);
  els.hexdump.querySelector(`.hb[data-i="${i}"]`)?.scrollIntoView({ block: 'nearest' });
}));

sync.register('stack', markerView((sets) => {
  if (!isShown('stack')) return [];
  return [...els.stackframe.querySelectorAll('tr[data-slot]')]
    .filter((tr) => sets.syms.has(tr.dataset.sym) || sets.slots.has(Number(tr.dataset.slot)));
}, (target, sets) => {
  if (!isShown('stack')) return;
  const tr = [...els.stackframe.querySelectorAll('tr[data-slot]')].find((r) => sets.syms.has(r.dataset.sym) || sets.slots.has(Number(r.dataset.slot)));
  tr?.scrollIntoView({ block: 'nearest' });
}));

els.stackframe.addEventListener('click', (e) => {
  const tr = e.target.closest('tr[data-slot]');
  if (!tr || !state.current) return;
  if (tr.dataset.sym) {
    const tok = els.ccode.querySelector(`.t[data-sym="${CSS.escape(tr.dataset.sym)}"]`);
    selectTarget({ sym: tr.dataset.sym }, 'stack', tok);
  } else {
    selectTarget({ slot: Number(tr.dataset.slot) }, 'stack');
  }
});

/** Describe a variable from every source the engine gives. */
/** An instruction as the student reads it (easy spelling unless they chose the decoder's exact text). */
const asmText = (insn) => spellInsn(insn, state.prefs.asmSpelling).text;

/** `29 bytes into main` (or `before main` for code above the entry); the raw offset goes in a title. */
function intoWords(addrHex) {
  const { data, fn } = state.current;
  const d = Number(BigInt(addrHex) - BigInt(data.address_hex));
  return d >= 0 ? `${d} byte${d === 1 ? '' : 's'} into ${displayName(fn)}` : `${-d} bytes before ${displayName(fn)}`;
}

const INVENTED = /^(?:v\d+|a\d+|param_\d+|local_[0-9a-f]+|[a-z]{1,3}Var\d+|dat_[0-9a-f]+|DAT_[0-9a-f]+|sub_[0-9a-f]+|FUN_[0-9a-f]+)$/i;

/** `undefined8` → `8 bytes of unknown type`; other types as written. */
function typeWords(type) {
  const m = /^undefined(\d*)$/.exec((type || '').trim());
  if (m) return `${m[1] || 1} byte${m[1] === '1' ? '' : 's'} of unknown type`;
  return type || '';
}

/** Where a value lives, in words: `register RAX`, `the stack, 20 bytes below the return address`. */
function whereWords(storage, stackOffset) {
  const reg = /^([a-z][a-z0-9]*)$/i.exec((storage || '').trim());
  if (reg) return `register ${reg[1].toUpperCase()}`;
  const st = /^stack\s*([-+])\s*(0x[0-9a-f]+|\d+)$/i.exec((storage || '').trim());
  const off = st ? (st[1] === '-' ? -1 : 1) * Number(st[2]) : stackOffset;
  if (Number.isInteger(off)) {
    return off < 0 ? `the stack, ${-off} bytes below the return address` : `the stack, ${off} bytes above the return address`;
  }
  return storage || '';
}

function describeVar(name) {
  const { data, decls } = state.current;
  const decl = decls.find((d) => d.name === name);
  const vars = data.variables.filter((v) => v.name === name);
  const param = vars.find((v) => v.kind === 'arg');
  const stack = vars.find((v) => Number.isInteger(v.stack_offset) && v.kind !== 'arg');
  const sigIndex = shownSignature()?.params.findIndex((p) => p.name === name) ?? -1;
  const isParam = !!param || sigIndex >= 0;
  const global = !isParam && !decl && !stack && /^dat_|^g_/.test(name);
  const kind = isParam ? 'parameter' : global ? 'global' : 'local variable';
  const type = decl?.type || param?.type || stack?.type || shownSignature()?.params[sigIndex]?.type || '';
  const index = param?.arg_index ?? (sigIndex >= 0 ? sigIndex : null);
  const input = isParam && Number.isInteger(index) ? index + 1 : null;
  const where = decl?.storage ? whereWords(decl.storage) : stack ? whereWords('', stack.stack_offset) : '';
  const raw = decl?.storage ? decl.storage : stack ? entryOffset(stack.stack_offset) : '';
  return { name, kind, type, input, where, raw };
}

const ordinal = (n) => `${n}${n % 100 >= 11 && n % 100 <= 13 ? 'th' : ['th', 'st', 'nd', 'rd'][n % 10] || 'th'}`;

function varSummary(name) {
  const v = describeVar(name);
  return [v.name, v.kind, typeWords(v.type)].filter(Boolean).join(' · ');
}

/** A C line's instructions in address order: exact ones, then (when on) the inferred ones. */
function lineInsns(n) {
  const { index } = state.current;
  const exact = index.lineToInsns.get(n) || [];
  const inferred = index.lineToInferred?.get(n) || [];
  const at = (a) => index.insnIndex.get(a) ?? -1;
  const lastExact = Math.max(-1, ...exact.map(at));
  const setup = inferred.filter((a) => at(a) < lastExact).length;
  const all = [...exact, ...inferred].sort((a, b) => at(a) - at(b));
  return { exact, inferred: new Set(inferred), all, setup, after: inferred.length - setup };
}

const MAX_CARD_ROWS = 10;

function lineCard(n, varTok) {
  const { data, index, decls, codeLines } = state.current;
  const { exact, inferred, all, setup, after } = lineInsns(n);
  const text = codeLines[n - 1] || '';
  let html;
  if (exact.length) {
    const insns = all.map((a) => index.addrToInsn.get(a)).filter(Boolean);
    const extra = [setup ? `+${setup} that set it up` : '', after ? `+${after} after it` : ''].filter(Boolean);
    html = `<div class="ch">Line ${n} → ${exact.length} instruction${exact.length === 1 ? '' : 's'}` +
      `${extra.length ? ` <span class="cm">${extra.join(', ')}</span>` : ''}</div>`;
    if (insns.length) {
      html += renderInsnRows(insns, { startHex: data.address_hex, prefs: state.prefs, max: MAX_CARD_ROWS, inferred });
      if (insns.length > MAX_CARD_ROWS) html += `<div class="cm">and ${insns.length - MAX_CARD_ROWS} more</div>`;
      html += '<div class="cf">Click the line to keep it highlighted</div>';
    } else {
      html += `<div class="cm">at ${escapeHtml(exact.map(bare).join(', '))}</div>`;
    }
  } else {
    const decl = decls.find((d) => d.line === n);
    let why;
    if (decl) why = `This line declares ${decl.name}. Declarations do not become instructions; the name is one the decompiler chose.`;
    else if (data.proto && text.trim().replace(/;$/, '') === data.proto) why = `This is the signature: what ${displayName(state.current.fn)} takes and returns.`;
    else if (/^\s*#\[/.test(text)) why = 'This is a Rust attribute the decompiler adds; no instruction belongs to it.';
    else if (/^\s*[{}]?\s*$/.test(text)) why = 'This line is only punctuation; no instruction belongs to it.';
    else if (!data.hasInstructions && !data.line_mappings.length) why = 'This version of the decompiler cannot link lines to instructions.';
    else why = 'This line has no instructions of its own: its work was merged into a neighbouring line.';
    html = `<div class="ch">Line ${n}</div><div class="cx">${escapeHtml(why)}</div>`;
  }
  if (varTok) html += `<div class="cx"><b>${escapeHtml(varSummary(varTok.textContent))}</b></div>`;
  return html;
}

function calleeCard(addrHex) {
  const fn = state.byAddr.get(addrHex);
  if (!fn) return `<div class="ch">${escapeHtml(bare(addrHex))}</div><div class="cm">Not a function this page found in the program</div>`;
  const cached = cachedBody(addrHex);
  const what = isStub(fn) ? 'Imported from a library' : groupOf(fn) === 'startup' ? 'Startup code the compiler added' : 'A function in this program';
  let html = `<div class="ch">${escapeHtml(displayName(fn))}</div><div class="cm">${what}${fn.size ? ` · ${fn.size} bytes` : ''}</div>`;
  if (cached?.proto) html += `<pre>${escapeHtml(cached.proto)}</pre>`;
  html += '<div class="cf">Double-click to open it</div>';
  return html;
}

/** Why an unmapped instruction is shown under a line (or as setup/cleanup), or null. */
function inferredNote(addrHex) {
  const { index, inferred, codeLines } = state.current;
  const i = index.insnIndex.get(addrHex);
  const row = inferred?.[i];
  if (row?.role === 'prologue') return 'Function setup: prepares the stack before the first line runs.';
  if (row?.role === 'epilogue') return 'Function cleanup: runs after the last line.';
  if (!row?.inferred) return null;
  const line = row.lines[0];
  const laterExact = (index.lineToInsns.get(line) || []).some((a) => index.insnIndex.get(a) > i);
  return `Part of line ${line} (${laterExact ? 'it sets up that line' : 'it finishes that line'}): ${(codeLines[line - 1] || '').trim()}`;
}

function insnCard(addrHex) {
  const { data, index, codeLines, hints, arch } = state.current;
  const insn = index.addrToInsn.get(addrHex);
  if (!insn) return null;
  const lines = index.insnToLines.get(addrHex) || [];
  let html = `<div class="ch">${escapeHtml(asmText(insn))}</div>` +
    `<div class="cm">${insn.size} byte${insn.size === 1 ? '' : 's'} at ${escapeHtml(bare(addrHex))}, ${escapeHtml(intoWords(addrHex))}` +
    `${state.prefs.asmBytes ? ` · ${escapeHtml(spacedBytes(insn.bytes))}` : ''}</div>`;
  const note = explain(insn.mnemonic, arch?.family || 'x86');
  if (note) html += `<div class="cx">${escapeHtml(note[0].toUpperCase() + note.slice(1))}.</div>`;
  const idiom = hints.get(addrHex);
  if (idiom) html += `<div class="cx cm">${escapeHtml(idiom[0].toUpperCase() + idiom.slice(1))}.</div>`;
  const inferredText = lines.length ? null : inferredNote(addrHex);
  if (lines.length) html += `<div class="cx">From line ${lines.join(', ')}: <code>${escapeHtml((codeLines[lines[0] - 1] || '').trim())}</code></div>`;
  else if (inferredText) html += `<div class="cx">${escapeHtml(inferredText)}</div>`;
  else html += '<div class="cx cm">Not linked to a line of C.</div>';
  return html;
}

function targetCard(hex) {
  const { data, index } = state.current;
  if (index.addrToInsn.has(hex)) {
    return `<div class="ch">${escapeHtml(bare(hex))}</div><div class="cm">Further on in ${escapeHtml(displayName(state.current.fn))} (${escapeHtml(formatAddr(hex, data.address_hex, 'rel'))}). Click to jump there.</div>`;
  }
  return calleeCard(hex);
}

/** What a stack slot holds, in words. */
function slotWords(off) {
  const hit = state.current.frame?.slots.find((s) => off >= s.offset && off < s.offset + s.size);
  const pos = off < 0 ? `${-off} bytes below the return address` : off === 0 ? 'the return address itself' : `${off} bytes above the return address`;
  return { hit, pos };
}

function slotCard(so) {
  const slot = so.dataset.slot;
  let html = `<div class="ch">A place on the stack</div><div class="cm"><code>${escapeHtml(so.textContent)}</code></div>`;
  if (slot !== undefined) {
    const off = Number(slot);
    const { hit, pos } = slotWords(off);
    html += `<div class="cx">${escapeHtml(pos[0].toUpperCase() + pos.slice(1))}.</div>`;
    if (hit) html += `<div class="cx">Holds <b>${escapeHtml(hit.name)}</b>${hit.type ? ` (${escapeHtml(typeWords(hit.type))})` : ''}${hit.offset !== off ? `, from byte ${off - hit.offset}` : ''}.</div>`;
  }
  return html;
}

// ── the Explain panel's "Selected" card ────────────────────────────────────

const lineLink = (n) => `<a class="x-line" data-act="sel-line" data-line="${n}">${n}</a>`;

function varSelectedCard(name) {
  const v = describeVar(name);
  const lines = [...(state.current.index.symToLines.get(name) || [])].sort((a, b) => a - b);
  const rust = state.current.rust;
  const canRetype = !rust && !(v.kind === 'parameter' && rust);
  let html = `<div class="x-card"><div><span class="x-name">${escapeHtml(name)}</span><span class="x-chip">${escapeHtml(v.kind)}</span></div>` +
    '<dl class="x-dl">' +
    (v.type ? `<dt>Type</dt><dd><code>${escapeHtml(v.type)}</code>${/^undefined/.test(v.type) ? ` <span class="d2muted">(${escapeHtml(typeWords(v.type))})</span>` : ''}</dd>` : '') +
    (v.input ? `<dt>Input</dt><dd>${ordinal(v.input)} input of ${escapeHtml(displayName(state.current.fn))}</dd>` : '') +
    (v.where ? `<dt>Lives in</dt><dd title="${escapeHtml(v.raw)}">${escapeHtml(v.where)}</dd>` : '') +
    (lines.length ? `<dt>Used on lines</dt><dd class="x-lines">${lines.map(lineLink).join(', ')}</dd>` : '') +
    '</dl>';
  if (INVENTED.test(name)) html += `<p class="x-note">${escapeHtml(name)} is a name the decompiler made up; rename it to what it means.</p>`;
  html += '<div class="x-acts"><button class="d2-btn small" data-act="sel-rename">Rename</button>' +
    `<button class="d2-btn small" data-act="sel-retype"${canRetype ? '' : ' disabled title="Changing a type needs the C view: set Show code as to C"'}>Change type</button></div></div>`;
  return html;
}

function insnSelectedCard(addrHex) {
  const { index, codeLines, hints, arch, data } = state.current;
  const insn = index.addrToInsn.get(addrHex);
  if (!insn) return '';
  const lines = index.insnToLines.get(addrHex) || [];
  const note = explain(insn.mnemonic, arch?.family || 'x86');
  const idiom = hints.get(addrHex);
  const inferredText = lines.length ? null : inferredNote(addrHex);
  const sp = spellInsn(insn, state.prefs.asmSpelling);
  let html = `<div class="x-card"><div class="x-code" title="${escapeHtml(insn.text)}">${escapeHtml(sp.text)}</div>` +
    `<p class="x-note" title="${escapeHtml(formatAddr(addrHex, data.address_hex, 'both'))}">${insn.size} byte${insn.size === 1 ? '' : 's'} at ${escapeHtml(bare(addrHex))}, ${escapeHtml(intoWords(addrHex))}</p>`;
  if (note) html += `<p class="x-note" style="color:var(--text)"><code>${escapeHtml(sp.mnemonic)}</code>: ${escapeHtml(note)}.</p>`;
  if (idiom) html += `<p class="x-note">${escapeHtml(idiom[0].toUpperCase() + idiom.slice(1))}.</p>`;
  if (lines.length) html += `<p class="x-note">From line ${lines.map(lineLink).join(', ')}: <code>${escapeHtml((codeLines[lines[0] - 1] || '').trim())}</code></p>`;
  else if (inferredText) html += `<p class="x-note">${escapeHtml(inferredText)}</p>`;
  html += `<div class="x-acts">${patchButtons(insn)}<button class="d2-btn small" data-act="sel-comment">Add a note</button></div></div>`;
  return html;
}

function lineSelectedCard(n) {
  const { index, codeLines } = state.current;
  const { exact, inferred, all } = lineInsns(n);
  const text = (codeLines[n - 1] || '').trim();
  let html = `<div class="x-card"><div class="x-code">${escapeHtml(text || '(empty line)')}</div>`;
  if (!all.length) return html + `<p class="x-note">Line ${n} has no instructions of its own.</p></div>`;
  html += `<p class="x-note" style="color:var(--text)">Line ${n} turns into ${all.length} instruction${all.length === 1 ? '' : 's'}` +
    `${inferred.size ? ` (${exact.length} linked by the decompiler, ${inferred.size} that set it up)` : ''}.</p><ul class="x-insns">`;
  for (const a of all) {
    const insn = index.addrToInsn.get(a);
    if (!insn) continue;
    html += `<li data-act="sel-insn" data-addr="${a}"${inferred.has(a) ? ' class="inf"' : ''} title="Show it in the assembly">` +
      `<span class="xa">${escapeHtml(bare(a))}</span><span class="xm">${escapeHtml(spellInsn(insn, state.prefs.asmSpelling).mnemonic)}</span>` +
      `<span>${escapeHtml(spellInsn(insn, state.prefs.asmSpelling).operands)}</span></li>`;
  }
  return html + '</ul></div>';
}

function slotSelectedCard(off) {
  const { hit, pos } = slotWords(off);
  let html = '<div class="x-card">';
  if (hit) {
    html += `<div><span class="x-name">${escapeHtml(hit.kind === 'pad' ? 'unused space' : hit.name)}</span></div><dl class="x-dl">` +
      (hit.type && hit.kind !== 'pad' ? `<dt>Type</dt><dd><code>${escapeHtml(hit.type)}</code></dd>` : '') +
      `<dt>Size</dt><dd>${hit.size} bytes</dd><dt>Where</dt><dd title="${escapeHtml(entryOffset(off))}">${escapeHtml(pos)}</dd></dl>`;
    if (hit.alias) html += `<p class="x-note">The decompiler also calls this ${escapeHtml(hit.alias)}.</p>`;
    if (hit.dim) html += '<p class="x-note">Only the debug info mentions it; the C code never uses it.</p>';
  } else {
    html += `<p class="x-note">${escapeHtml(pos[0].toUpperCase() + pos.slice(1))}.</p>`;
  }
  const fp = state.current.frame?.fp;
  if (fp !== null && fp !== undefined) html += `<p class="x-note">The frame pointer RBP points ${-fp} bytes below the return address, so <code>[RBP - d]</code> is d bytes further down.</p>`;
  return html + '</div>';
}

/** The card for the current selection (empty when nothing is selected). */
function selectedCard(target = state.sel) {
  if (!target || !state.current) return '';
  if (target.sym) return varSelectedCard(target.sym);
  if (target.addr) return insnSelectedCard(target.addr);
  if (Number.isInteger(target.line)) return lineSelectedCard(target.line);
  if (target.slot !== undefined && target.slot !== null) return slotSelectedCard(target.slot);
  return '';
}

function resolveHover(el) {
  if (!state.current || !el?.closest) return null;
  if (els.ccode.contains(el)) {
    const row = el.closest('.d2-cl');
    if (!row) return null;
    const n = Number(row.dataset.line);
    const tok = el.closest('.t');
    const callee = tok?.dataset.kind === 'funcname' ? tok.dataset.callee : null;
    if (callee && callee !== state.current.data.address_hex) {
      return { key: `callee:${callee}:${n}`, html: calleeCard(callee), anchor: tok };
    }
    const varTok = tok?.dataset.kind === 'variable' ? tok : null;
    return { key: `L${n}:${varTok ? varTok.textContent : ''}`, html: lineCard(n, varTok), anchor: row.querySelector('.ct') || row };
  }
  if (els.asmcode.contains(el)) {
    const xt = el.closest('a.xt');
    if (xt) return { key: 'xt:' + xt.dataset.goto, html: targetCard(xt.dataset.goto), anchor: xt };
    const so = el.closest('.so');
    const row = el.closest('.d2-ar');
    if (so && row) return { key: 'so:' + row.dataset.addr, html: slotCard(so), anchor: so };
    if (row) {
      const html = insnCard(row.dataset.addr);
      return html ? { key: 'A' + row.dataset.addr, html, anchor: row.querySelector('.ao') || row } : null;
    }
  }
  return null;
}

const hover = createHover({
  roots: [els.ccode, els.asmcode],
  card: els.card,
  delay: () => state.prefs.hoverDelay,
  resolve: resolveHover,
});

/** Select a target in every pane; `from` is the pane that asked (not scrolled). */
function selectTarget(target, from, tokEl = null) {
  state.sel = target;
  const sets = sync.select(target, { from });
  setCursor(tokEl);
  rail.markVars(sets ? sets.syms : new Set());
  const active = target?.addr ? 'a-' + target.addr : Number.isInteger(target?.line) ? 'c-L' + target.line : null;
  if (target?.line || target?.sym) {
    const line = target.line ?? (sets && sets.lines.size ? Math.min(...sets.lines) : null);
    if (line) els.ccode.setAttribute('aria-activedescendant', 'c-L' + line);
  }
  if (target?.addr) els.asmcode.setAttribute('aria-activedescendant', active);
  if (target?.sym) {
    setHint(`${escapeHtml(target.sym)} — ${editKeysHint(target.sym)}`);
  } else if (target?.addr) {
    const insn = state.current.index.addrToInsn.get(target.addr);
    setHint(insn ? `${escapeHtml(asmText(insn))} — press <kbd>;</kbd> to add a note` : null);
  } else if (Number.isInteger(target?.line)) {
    setHint(`Line ${target.line} — see its instructions in the Explain panel`);
  } else {
    setHint(null);
  }
  rail.setSelected(selectedCard(target));
  return sets;
}

function setCursor(tokEl) {
  state.cursor?.classList.remove('cur');
  state.cursor = tokEl || null;
  tokEl?.classList.add('cur');
}

/** Show the Assembly pane (switching tabs unless split already shows it). */
function ensureShown(tab) {
  if (!isShown(tab)) setTab(tab);
}

/** Jump to an address: an instruction here, or the function that holds it. */
function gotoAddr(hex) {
  if (!state.current) return;
  if (state.current.index.addrToInsn.has(hex)) {
    ensureShown('asm');
    selectTarget({ addr: hex }, null);
    return;
  }
  const fn = state.byAddr.get(hex) || containingFunction(hex);
  if (fn) openFunction(fn, { focusAddr: fn.address_hex === hex ? null : hex });
  else showDataAt(hex);
}

function containingFunction(hex) {
  const a = BigInt(hex);
  for (const fn of state.byAddr.values()) {
    const start = BigInt(fn.address_hex);
    if (a >= start && a < start + BigInt(fn.size || 0)) return fn;
  }
  return null;
}

function openCallee(tok) {
  const fn = state.byAddr.get(tok.dataset.callee) || state.byName.get(tok.textContent);
  if (fn && fn.address_hex !== state.current.data.address_hex) openFunction(fn);
}

els.ccode.addEventListener('click', (e) => {
  if (!state.current) return;
  const row = e.target.closest('.d2-cl');
  if (!row) return;
  const n = Number(row.dataset.line);
  if (e.target.closest('.la')?.textContent) {
    ensureShown('asm');
    selectTarget({ line: n }, 'c');
    return;
  }
  const tok = e.target.closest('.t');
  if (tok?.dataset.sym) selectTarget({ sym: tok.dataset.sym }, 'c', tok);
  else selectTarget({ line: n }, 'c', tok);
});

els.ccode.addEventListener('dblclick', (e) => {
  const tok = e.target.closest('.t');
  if (!tok || !state.current) return;
  const target = tokenTarget(tok);
  if (target?.kind === 'callee') {
    e.preventDefault();
    openCallee(tok);
  } else if (target) {
    e.preventDefault();
    window.getSelection()?.removeAllRanges();
    if (tok.dataset.sym) selectTarget({ sym: tok.dataset.sym }, 'c', tok);
    renameTarget(target);
  }
});

els.asmcode.addEventListener('click', (e) => {
  if (!state.current) return;
  const xt = e.target.closest('a.xt');
  if (xt) {
    e.preventDefault();
    gotoAddr(xt.dataset.goto);
    return;
  }
  const row = e.target.closest('.d2-ar');
  const so = e.target.closest('.so[data-slot]');
  if (so && row) {
    const sym = state.current.index.slotToSym?.get(Number(so.dataset.slot));
    selectTarget(sym ? { sym, addr: row.dataset.addr } : { slot: Number(so.dataset.slot), addr: row.dataset.addr }, 'asm');
    return;
  }
  if (row) selectTarget({ addr: row.dataset.addr }, 'asm');
});

for (const [pane, selector, targetOf] of [
  [els.ccode, '.d2-cl', (el) => ({ line: Number(el.dataset.line) })],
  [els.asmcode, '.d2-ar', (el) => ({ addr: el.dataset.addr })],
]) {
  let last = null;
  pane.addEventListener('pointerover', (e) => {
    const el = e.target.closest(selector);
    if (el === last) return;
    last = el;
    sync.hover(el ? targetOf(el) : null);
  });
  pane.addEventListener('pointerleave', () => {
    last = null;
    sync.hover(null);
  });
}

/** Keyboard navigation inside the C and Assembly panes. */
function paneKey(e) {
  if (!state.current) return false;
  const inC = els.ccode.contains(document.activeElement);
  const inAsm = els.asmcode.contains(document.activeElement);
  if (!inC && !inAsm) return false;
  const { index } = state.current;
  if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
    const step = e.key === 'ArrowDown' ? 1 : -1;
    if (inC) {
      const cursorLine = state.cursor?.isConnected ? Number(state.cursor.closest('.d2-cl')?.dataset.line) : null;
      const symLines = state.sel?.sym ? [...(index.symToLines.get(state.sel.sym) || [])] : [];
      const cur = state.sel?.line ?? cursorLine ?? (symLines.length ? Math.min(...symLines) : 0);
      const next = Math.min(Math.max(cur + step, 1), index.lineCount);
      selectTarget({ line: next }, null);
      $('c-L' + next)?.scrollIntoView({ block: 'nearest' });
      hover.showFor($('c-L' + next)?.querySelector('.ct'));
    } else {
      const insns = state.current.data.instructions;
      const at = state.sel?.addr ? index.insnIndex.get(state.sel.addr) : -1;
      const next = insns[Math.min(Math.max(at + step, 0), insns.length - 1)];
      if (next) {
        selectTarget({ addr: next.address_hex }, null);
        $('a-' + next.address_hex)?.scrollIntoView({ block: 'nearest' });
        hover.showFor($('a-' + next.address_hex));
      }
    }
    return true;
  }
  if (inC && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
    const toks = [...els.ccode.querySelectorAll('.t:not([data-kind=syntax]):not([data-kind=comment])')];
    if (!toks.length) return true;
    const at = state.cursor ? toks.indexOf(state.cursor) : -1;
    const next = toks[Math.min(Math.max(at + (e.key === 'ArrowRight' ? 1 : -1), 0), toks.length - 1)];
    const line = Number(next.closest('.d2-cl').dataset.line);
    if (next.dataset.sym) selectTarget({ sym: next.dataset.sym }, null, next);
    else selectTarget({ line }, null, next);
    next.scrollIntoView({ block: 'nearest', inline: 'nearest' });
    hover.showFor(next);
    return true;
  }
  if (e.key === 'Enter' && state.cursor?.dataset.kind === 'funcname' && state.cursor.dataset.callee) {
    openCallee(state.cursor);
    return true;
  }
  return false;
}

// ── views, view options ────────────────────────────────────────────────────

/** Switch the view: `c`, `split` (C beside the assembly), `asm`, `bytes`, `stack` or `src`. */
function setView(view) {
  if (view === 'src' && $('tab-src').hidden) return;
  state.view = view;
  if (view !== 'src') updatePrefs({ view });
  const shown = visibleTabs();
  for (const btn of els.tabs.querySelectorAll('[role=tab]')) {
    const on = btn.dataset.tab === view;
    btn.setAttribute('aria-selected', String(on));
    btn.tabIndex = on ? 0 : -1;
  }
  for (const t of ['c', 'asm', 'bytes', 'stack', 'src']) $(paneFor(t)).hidden = !shown.includes(t);
  els.panes.classList.toggle('split', shown.length > 1);
  els.narrow.hidden = !(view === 'split' && shown.length === 1);
  if (state.asmRendered && state.asmRendered !== (inSplit() ? 'split' : 'asm')) state.rendered.delete('asm');
  applyPaneClasses();
  renderVisible();
}
const setTab = setView;

els.tabs.addEventListener('click', (e) => {
  const btn = e.target.closest('[role=tab]');
  if (btn) setView(btn.dataset.tab);
});
els.tabs.addEventListener('keydown', (e) => {
  if (e.key !== 'ArrowRight' && e.key !== 'ArrowLeft') return;
  const tabs = [...els.tabs.querySelectorAll('[role=tab]:not([hidden])')];
  const at = tabs.findIndex((t) => t.dataset.tab === state.view);
  const next = tabs[(at + (e.key === 'ArrowRight' ? 1 : tabs.length - 1)) % tabs.length];
  setView(next.dataset.tab);
  next.focus();
});
narrowView.addEventListener('change', () => { if (state.current) setView(state.view); });

function updatePrefs(patch) {
  state.prefs = { ...state.prefs, ...patch };
  savePrefs(storage, state.prefs);
}

/** `s`: into Side by side, and back to the view it came from. */
function toggleSplit() {
  if (state.view === 'split') setView(state.lastView && state.lastView !== 'split' ? state.lastView : 'c');
  else {
    state.lastView = state.view;
    setView('split');
  }
}

const ADDR_WORDS = { abs: 'Full address', rel: 'Offset from function start', both: 'Both' };
const CMODE_WORDS = { heading: 'As headings', comment: 'As comments', off: 'Off' };
const SPELL_WORDS = { easy: 'Easy to read', exact: 'Exactly as decoded' };
const DELAY_WORDS = [[0, 'Instant'], [250, 'Short'], [450, 'Normal'], [800, 'Slow'], [-1, 'Off']];

function radios(name, words, value) {
  return Object.entries(words).map(([v, label]) =>
    `<label><input type="radio" name="${name}" value="${v}"${String(value) === v ? ' checked' : ''}>${escapeHtml(label)}</label>`).join('');
}

function check(name, label, value) {
  return `<label><input type="checkbox" name="${name}"${value ? ' checked' : ''}>${escapeHtml(label)}</label>`;
}

/** The View options popover: labelled controls, each applied as it changes. */
function renderViewMenu() {
  const p = state.prefs;
  const bytesKey = inSplit() ? 'asmBytesSplit' : 'asmBytes';
  els.viewMenu.innerHTML = '<div class="d2-vo">' +
    `<fieldset><legend>Addresses</legend><div class="opts">${radios('asmAddr', ADDR_WORDS, p.asmAddr)}</div></fieldset>` +
    `<fieldset><legend>Assembly spelling</legend><div class="opts">${radios('asmSpelling', SPELL_WORDS, p.asmSpelling)}</div></fieldset>` +
    `<fieldset><legend>Show C next to the assembly</legend><div class="opts">${radios('asmCMode', CMODE_WORDS, p.asmCMode)}</div></fieldset>` +
    `<fieldset><legend>Assembly</legend><div class="opts" style="flex-direction:column;align-items:flex-start">` +
    check(bytesKey, 'Show instruction bytes', p[bytesKey]) +
    check('asmInfer', 'Group setup instructions with their line', p.asmInfer) +
    check('asmArrows', 'Show jump arrows', p.asmArrows) + '</div></fieldset>' +
    `<fieldset><legend>C code</legend><div class="opts" style="flex-direction:column;align-items:flex-start">` +
    check('cLineNumbers', 'Show line numbers', p.cLineNumbers) +
    check('cLineAddrs', 'Show addresses next to C lines', p.cLineAddrs) + '</div></fieldset>' +
    '<div class="d2-vorow"><span>Hover delay</span><select name="hoverDelay">' +
    DELAY_WORDS.map(([v, l]) => `<option value="${v}"${p.hoverDelay === v ? ' selected' : ''}>${l}</option>`).join('') + '</select></div>' +
    '<button class="d2-link reset" data-act="reset">Reset to defaults</button></div>';
}

function toggleViewMenu(open = els.viewMenu.hidden) {
  els.viewMenu.hidden = !open;
  els.viewBtn.setAttribute('aria-expanded', String(open));
  if (open) {
    renderViewMenu();
    els.viewMenu.querySelector('input')?.focus();
  }
}

els.viewBtn.addEventListener('click', () => toggleViewMenu());
els.viewMenu.addEventListener('change', (e) => {
  const input = e.target;
  const key = input.name;
  if (!key) return;
  let value;
  if (input.type === 'checkbox') value = input.checked;
  else if (key === 'hoverDelay') value = Number(input.value);
  else value = input.value;
  updatePrefs({ [key]: value });
  applyPaneClasses();
  if (key === 'asmInfer' && state.current) showFunction(state.current.fn, state.current.data, { keep: true });
  else if (['asmCMode', 'asmArrows', 'cLineAddrs'].includes(key)) rerender('asm', 'c');
  else if (key === 'asmSpelling') {
    rerender('asm');
    rail.setSelected(selectedCard());
  }
});
els.viewMenu.addEventListener('click', (e) => {
  if (!e.target.closest('[data-act=reset]')) return;
  updatePrefs({ ...DEFAULT_PREFS, view: state.view, theme: state.prefs.theme, tipSeen: state.prefs.tipSeen, rail: state.prefs.rail });
  applyPaneClasses();
  if (state.current) showFunction(state.current.fn, state.current.data, { keep: true });
  renderViewMenu();
});

// ── menus, theme, the Explain panel and the narrow-window drawers ──────────

function closeMenus(except = null) {
  if (except !== els.viewMenu && !els.viewMenu.hidden) toggleViewMenu(false);
  if (except !== els.moreMenu && !els.moreMenu.hidden) {
    els.moreMenu.hidden = true;
    els.more.setAttribute('aria-expanded', 'false');
  }
}

els.more.addEventListener('click', () => {
  const open = els.moreMenu.hidden;
  closeMenus(els.moreMenu);
  els.moreMenu.hidden = !open;
  els.more.setAttribute('aria-expanded', String(open));
  if (open) els.moreMenu.querySelector('.d2-mi:not(:disabled), select')?.focus();
});
els.moreMenu.addEventListener('click', (e) => {
  if (e.target.closest('button.d2-mi, a.d2-mi')) closeMenus();
});
els.keys.addEventListener('click', () => openHelp({ allKeys: true }));
document.addEventListener('mousedown', (e) => {
  if (!e.target.closest('.d2-menuwrap')) closeMenus();
});


function applyTheme() {
  const dark = state.prefs.theme !== 'light';
  document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  els.theme.setAttribute('aria-pressed', String(dark));
  els.theme.title = dark ? 'Switch to the light theme' : 'Switch to the dark theme';
}
els.theme.addEventListener('click', () => {
  updatePrefs({ theme: state.prefs.theme === 'light' ? 'dark' : 'light' });
  applyTheme();
});
applyTheme();

const mediumView = window.matchMedia('(max-width: 1279px)');

/**
 * The Explain panel is a column on a wide window (hidden when collapsed) and
 * a drawer below 1280px; below 900px the function list is a drawer too.
 */
function syncLayout() {
  const drawer = mediumView.matches;
  const open = drawer ? els.work.classList.contains('drawer') : state.prefs.rail;
  els.work.classList.toggle('norail', !drawer && !open);
  els.rail.hidden = !state.inventory || (!drawer && !open);
  els.explain.hidden = !state.inventory || (!drawer && open);
  els.explain.setAttribute('aria-expanded', String(open));
  els.railBtn.setAttribute('aria-expanded', String(open));
  els.fnsBtn.hidden = !state.inventory || !narrowView.matches;
}

function setRail(open) {
  if (mediumView.matches) els.work.classList.toggle('drawer', open);
  else updatePrefs({ rail: open });
  syncLayout();
}
els.railBtn.addEventListener('click', () => setRail(false));
els.explain.addEventListener('click', () => setRail(true));
els.fnsBtn.addEventListener('click', () => {
  const open = !els.work.classList.contains('fns');
  els.work.classList.toggle('fns', open);
  els.fnsBtn.setAttribute('aria-expanded', String(open));
});
els.list.addEventListener('click', (e) => {
  if (e.target.closest('.fn') && narrowView.matches) {
    els.work.classList.remove('fns');
    els.fnsBtn.setAttribute('aria-expanded', 'false');
  }
});
mediumView.addEventListener('change', () => {
  els.work.classList.remove('drawer');
  syncLayout();
});
narrowView.addEventListener('change', () => {
  els.work.classList.remove('fns');
  syncLayout();
});
syncLayout();

// ── the keyboard ───────────────────────────────────────────────────────────

const typing = (el) => el && (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA' ||
  el.tagName === 'SELECT' || el.isContentEditable);

document.addEventListener('keydown', (e) => {
  if (e.defaultPrevented) return;
  if (e.altKey && !e.ctrlKey && !e.metaKey && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
    e.preventDefault();
    if (e.key === 'ArrowLeft' && !els.back.disabled) history.back();
    if (e.key === 'ArrowRight' && !els.fwd.disabled) history.forward();
    return;
  }
  if ((e.ctrlKey || e.metaKey) && !e.altKey && (e.key === 'z' || e.key === 'Z' || e.key === 'y') &&
      !typing(document.activeElement) && state.current) {
    e.preventDefault();
    if (e.key === 'y' || e.shiftKey) redo(); else undo();
    return;
  }
  if (e.ctrlKey || e.metaKey || e.altKey || typing(document.activeElement)) return;
  if (e.key === 'Escape') {
    if (hover.hide()) return;
    if (dialogs.close()) return;
    if (bytesState.edit !== null) {
      bytesState.edit = null;
      flushBytes();
      if (state.current?.data.hasInstructions && state.rendered.has('bytes')) renderBytes();
      return;
    }
    if (!els.viewMenu.hidden) { toggleViewMenu(false); els.viewBtn.focus(); return; }
    if (!els.moreMenu.hidden) { closeMenus(); els.more.focus(); return; }
    if (els.work.classList.contains('drawer')) { setRail(false); return; }
    if (els.work.classList.contains('fns')) { els.fnsBtn.click(); return; }
    if (state.sel) selectTarget(null, null);
    return;
  }
  if (e.key === '?') {
    e.preventDefault();
    openHelp();
    return;
  }
  if (!state.current && e.key !== '/') return;
  if (hexKey(e)) {
    e.preventDefault();
    return;
  }
  if (paneKey(e)) {
    e.preventDefault();
    return;
  }
  switch (e.key) {
    case '/':
      if (els.filter.disabled) return;
      e.preventDefault();
      els.filter.focus();
      els.filter.select();
      break;
    case ' ':
      e.preventDefault();
      setView(state.view === 'asm' ? 'c' : 'asm');
      break;
    case '1': case '2': case '3': case '4':
      setView(['c', 'asm', 'bytes', 'stack'][Number(e.key) - 1]);
      break;
    case 's':
      toggleSplit();
      break;
    case 'o':
      updatePrefs(cycle(state.prefs, 'asmAddr'));
      applyPaneClasses();
      setHint(`Addresses: ${ADDR_WORDS[state.prefs.asmAddr].toLowerCase()}`);
      break;
    case 'b': {
      const key = inSplit() ? 'asmBytesSplit' : 'asmBytes';
      updatePrefs({ [key]: !state.prefs[key] });
      applyPaneClasses();
      setHint(`Instruction bytes ${state.prefs[key] ? 'shown' : 'hidden'}${inSplit() ? ' side by side' : ''}`);
      break;
    }
    case 'n':
      e.preventDefault();
      renameSelected();
      break;
    case 'y':
      e.preventDefault();
      retypeSelected();
      break;
    case ';':
      e.preventDefault();
      commentSelected();
      break;
    case 'g':
      e.preventDefault();
      goToDialog();
      break;
    case 'u':
      undo();
      break;
    case 'x':
      e.preventDefault();
      openRefs();
      break;
    default:
      return;
  }
});

setView(state.prefs.view);
applyPaneClasses();
syncButtons();
window.addEventListener('beforeunload', () => state.kuna?.close());

// ── editing: dialogs, applyEdit, undo, the rail ────────────────────────────

/** What a C token stands for, as an edit target. */
function tokenTarget(tok) {
  if (!tok || !state.current) return null;
  const self = state.current.data.address_hex;
  if (tok.dataset.kind === 'funcname') {
    const callee = tok.dataset.callee || state.byName.get(tok.textContent)?.address_hex || null;
    if (!callee || callee === self || tok.dataset.decl === 'function') return { kind: 'self', addr: self, el: tok };
    return { kind: 'callee', addr: callee, el: tok };
  }
  if (tok.dataset.kind === 'variable') {
    if (tok.dataset.gaddr) return { kind: 'global', addr: tok.dataset.gaddr, name: tok.textContent, el: tok };
    return { kind: 'var', name: tok.textContent, el: tok };
  }
  return null;
}

/** The edit target from the cursor or the selection. */
function selectedTarget() {
  if (state.cursor?.isConnected) {
    const t = tokenTarget(state.cursor);
    if (t) return t;
  }
  if (state.sel?.sym) {
    const el = els.ccode.querySelector(`.t[data-sym="${CSS.escape(state.sel.sym)}"]`);
    return el ? tokenTarget(el) : { kind: 'var', name: state.sel.sym, el: null };
  }
  return null;
}

/** A function's C signature: the student's prototype, else the engine's (null in the Rust view). */
function currentSignature(addr = state.current.data.address_hex) {
  const own = addr === state.current.data.address_hex;
  const cached = own ? state.current.data : cachedBody(addr);
  if (session.proto(addr)) return parseSignature(session.proto(addr));
  if ((own ? state.current.rust : /rust/i.test(cached?.language || '')) || !cached?.proto) return null;
  return parseSignature(cached.proto);
}

/** The signature the open function shows, for display (Rust or C). */
function shownSignature() {
  const { data, rust } = state.current;
  return rust ? parseRustSignature(data.proto || '') : parseSignature(data.proto || '');
}

function paramIndex(name) {
  const sig = currentSignature();
  const i = sig ? sig.params.findIndex((p) => p.name === name) : -1;
  return i >= 0 ? { sig, index: i } : null;
}

function isParam(name) {
  return !!shownSignature()?.params.some((p) => p.name === name);
}

/**
 * Edits written as C declarations (retypes, prototypes, parameters, globals)
 * are refused in the Rust view: its signature is not C, and a declaration
 * built from it would be wrong. Local and function renames, comments and
 * patches do not depend on the language.
 */
function needsC(what) {
  if (!state.current?.rust) return false;
  toast(`${what} needs the C view.`, {
    kind: 'warn', detail: 'Types and signatures are written in C. Choose C under ⋯ → Show code as.',
  });
  return true;
}

function editKeysHint(name) {
  if (!state.current?.rust) return 'press <kbd>N</kbd> to rename, <kbd>Y</kbd> to change type';
  return isParam(name) ? 'inputs can be changed in the C view' : 'press <kbd>N</kbd> to rename';
}

function typeOptions() {
  const data = state.current?.data;
  return knownTypes({ types: data?.types, known: state.inventory?.known_types, typedefs: session.typedefTags() });
}

/** How wide pointers and `long` are on this target (LLP64 on Windows). */
function sizeModel() {
  const target = state.current?.data.target || state.inventory?.target;
  return {
    bits: target?.bits || 64,
    llp64: /:windows\b/i.test(target?.archid || '') || /PE/.test(state.binary?.format || ''),
  };
}

/** The storage size behind a displayed local, when the engine or the declaration says. */
function currentSize(name) {
  const v = state.current.data.variables.find((x) => x.name === name && x.size);
  if (v) return v.size;
  const decl = state.current.decls.find((d) => d.name === name);
  return decl ? typeSize(decl.type, sizeModel()) : null;
}

const noHash = (v) => (/\s#/.test(v) || /[\r\n]/.test(v) ? 'a directive cannot hold a newline or " #" (the .kuna file reads that as a comment)' : null);

async function renameSelected() {
  const target = selectedTarget();
  if (!target) {
    if (state.current) return renameTarget({ kind: 'self', addr: state.current.data.address_hex, el: els.vname });
    return;
  }
  renameTarget(target);
}

async function renameTarget(target) {
  const addr = state.current.data.address_hex;
  if (target.kind === 'self' || target.kind === 'callee') {
    const fn = state.byAddr.get(target.addr) || { address_hex: target.addr, name: target.el?.textContent || target.addr };
    const res = await dialogs.openPopover({
      anchorEl: target.el, title: `Rename ${displayName(fn)}`,
      note: 'Give the function a name that says what it does. The new name is used everywhere it is called.',
      fields: [{ name: 'name', label: 'New name', value: displayName(fn), validate: validateIdent }],
      submitLabel: 'Rename',
    });
    if (!res || res.name === displayName(fn)) return;
    const old = displayName(fn);
    return applyEdit(() => session.setFunctionName(target.addr, res.name === fn.name ? null : res.name),
      { label: 'rename', done: `Renamed ${old} to ${res.name}` });
  }
  if (target.kind === 'global') return dataDialog(target);
  if (isParam(target.name) && needsC('Renaming a parameter')) return;
  const param = paramIndex(target.name);
  const res = await dialogs.openPopover({
    anchorEl: target.el,
    title: `Rename ${target.name}`,
    note: param ? 'This is an input of the function; renaming it updates the signature.' : varSummary(target.name),
    fields: [{ name: 'name', label: 'New name', value: target.name, validate: validateIdent }],
    submitLabel: 'Rename',
  });
  if (!res || res.name === target.name) return;
  const done = `Renamed ${target.name} to ${res.name}`;
  if (param) {
    param.sig.params[param.index].name = res.name;
    return applyEdit(() => session.setProto(addr, buildPrototype(param.sig)), { label: 'rename', done, reselect: { sym: res.name } });
  }
  const sym = session.symbolOf(addr, target.name);
  return applyEdit(() => session.setVar(addr, sym, { name: res.name }), { label: 'rename', done, reselect: { sym: res.name } });
}

async function retypeSelected() {
  if (!state.current || needsC('Retyping')) return;
  const target = selectedTarget();
  if (!target) return protoDialog(state.current.data.address_hex, els.vname);
  if (target.kind === 'self' || target.kind === 'callee') return protoDialog(target.addr, target.el);
  if (target.kind === 'global') return dataDialog(target);
  const addr = state.current.data.address_hex;
  const param = paramIndex(target.name);
  const current = param ? param.sig.params[param.index].type : describeVar(target.name).type;
  const size = param ? null : currentSize(target.name);
  const res = await dialogs.openPopover({
    anchorEl: target.el,
    title: `Change the type of ${target.name}`,
    note: param ? 'This is an input of the function; changing it updates the signature.' : varSummary(target.name),
    fields: [{ name: 'type', label: 'Type (in C, like int or char *)', value: current, list: typeOptions(), validate: validateCType }],
    submitLabel: 'Change type',
    warn: (v) => {
      const next = typeSize(v.type, sizeModel());
      return size && next && next !== size
        ? `${target.name} takes ${size} bytes and ${normalizeType(v.type)} takes ${next}, so this change will not apply. Pick a type of the same size.`
        : '';
    },
  });
  if (!res || normalizeType(res.type) === normalizeType(current)) return;
  const type = normalizeType(res.type);
  const done = `Changed ${target.name} to ${type}`;
  if (param) {
    param.sig.params[param.index].type = type;
    return applyEdit(() => session.setProto(addr, buildPrototype(param.sig)), { label: 'retype', done, reselect: { sym: target.name } });
  }
  const sym = session.symbolOf(addr, target.name);
  const rec = session.varRecord(addr, target.name);
  return applyEdit(() => session.setVar(addr, sym, { type, name: rec?.name ?? undefined }), { label: 'retype', done, reselect: { sym: target.name } });
}

async function protoDialog(addr, anchorEl) {
  if (needsC('Editing a prototype')) return;
  const fn = state.byAddr.get(addr) || { address_hex: addr, name: addr };
  const sig = currentSignature(addr);
  const value = session.proto(addr) || (sig ? buildPrototype(sig) : `void ${displayName(fn)}(void)`);
  const res = await dialogs.openPopover({
    anchorEl, title: `Signature of ${displayName(fn)}`,
    note: 'What the function takes and returns, written in C. Input names and types are used; the function keeps its name.',
    fields: [{
      name: 'decl', label: 'Signature', value, textarea: true,
      validate: (v) => (parseSignature(v) ? noHash(v) : 'Write it like: long sum_to(int count)'),
    }],
    submitLabel: 'Save signature',
  });
  if (!res) return;
  const decl = buildPrototype(parseSignature(res.decl));
  if (decl === value) return;
  return applyEdit(() => session.setProto(addr, decl), { label: 'signature', done: `Changed the signature of ${displayName(fn)}` });
}

async function dataDialog(target) {
  if (needsC('Typing a global')) return;
  const g = state.current.data.globals.find((x) => x.address_hex === target.addr || x.name === target.name);
  const rec = session.records.get(`data:${target.addr}`);
  const declType = g?.declaration ? g.declaration.replace(new RegExp(`\\b${target.name}\\b`), '').replace(/;\s*$/, '').trim() : '';
  const res = await dialogs.openPopover({
    anchorEl: target.el, title: `The global variable at ${bare(target.addr)}`,
    note: 'Name it and say what type it has; every function that uses it will show the change.',
    fields: [
      { name: 'name', label: 'Name', value: rec?.name || target.name, validate: validateIdent },
      { name: 'type', label: 'Type (in C)', value: rec?.type || declType || 'undefined4', list: typeOptions(), validate: validateCType },
    ],
    submitLabel: 'Save',
  });
  if (!res) return;
  return applyEdit(() => session.setData(target.addr, normalizeType(res.type), res.name),
    { label: 'global', done: `Named the global at ${bare(target.addr)} ${res.name}` });
}

async function commentSelected() {
  if (!state.current) return;
  const { index, data } = state.current;
  let addr = state.sel?.addr || null;
  if (!addr && Number.isInteger(state.sel?.line)) addr = index.lineToInsns.get(state.sel.line)?.[0] || null;
  if (!addr && state.cursor?.isConnected) addr = index.lineToInsns.get(Number(state.cursor.closest('.d2-cl').dataset.line))?.[0] || null;
  if (!addr) {
    toast('Select an instruction, or a line of C that has one, to add a note.', { kind: 'warn' });
    return;
  }
  const anchor = $('a-' + addr) || els.ccode.querySelector(`.d2-cl[data-addrs~="${addr}"] .ct`);
  const res = await dialogs.openPopover({
    anchorEl: anchor, title: `Note at ${bare(addr)}`,
    note: 'The note appears in the C code above this instruction. Leave it empty to remove it.',
    fields: [{ name: 'text', label: 'Note', value: session.comment(data.address_hex, addr) || '', validate: noHash }],
    submitLabel: 'Save note',
  });
  if (!res) return;
  return applyEdit(() => session.setComment(data.address_hex, addr, res.text.trim() || null),
    { label: 'note', done: res.text.trim() ? 'Added a note' : 'Removed the note' });
}

async function goToDialog() {
  if (!state.inventory) return;
  const res = await dialogs.openPopover({
    title: 'Go to', submitLabel: 'Go', fields: [{
      name: 'where', label: 'A function name or an address', value: '',
      list: state.rows.map((r) => displayName(r.fn)),
      validate: (v) => (v.trim() ? null : 'a name or an address'),
    }],
  });
  if (!res) return;
  const q = res.where.trim();
  const byShown = state.rows.find((r) => displayName(r.fn) === q)?.fn || state.byName.get(q);
  if (byShown) return openFunction(byShown);
  if (/^(0x)?[0-9a-f]+$/i.test(q)) return gotoAddr('0x' + BigInt('0x' + q.replace(/^0x/i, '')).toString(16));
  toast(`There is no function called ${q}.`, { kind: 'warn' });
}

/**
 * After any change to the session: persist, refresh names and the rail. The
 * caches need no flush: their keys carry the directives a body depends on.
 */
function sessionChanged() {
  persist();
  bytesState.version++;
  syncPatchButton();
  refreshRowNames();
  if (state.current) els.vname.textContent = displayName(state.current.fn);
  renderRail();
}

function captureScroll() {
  return [els.ccode, els.asmcode, els.hexdump, els.stackframe].map((el) => [el, el.scrollTop, el.scrollLeft]);
}

function restoreScroll(saved) {
  for (const [el, top, left] of saved) { el.scrollTop = top; el.scrollLeft = left; }
}

function flash(lines) {
  for (const n of lines) {
    const el = $('c-L' + n);
    if (!el) continue;
    el.classList.remove('d2-flash');
    void el.offsetWidth;
    el.classList.add('d2-flash');
  }
}

/**
 * Apply one edit: snapshot, `mutate` the session, save, drop caches, and
 * re-inspect the open function while the old render stays up. A failed or
 * cancelled request restores the snapshot; a rejected directive stays in the
 * session, marked, with the engine's reason in a toast.
 */
async function applyEdit(mutate, { label = 'edit', reselect = null, done = '' } = {}) {
  const addr = state.current?.data.address_hex ?? null;
  const before = new Set(addr ? session.assertionsFor(addr) : []);
  const snap = session.snapshot();
  try {
    mutate();
  } catch (e) {
    toast(e.message, { kind: 'err' });
    return false;
  }
  const fresh = addr ? session.assertionsFor(addr).filter((d) => !before.has(d)) : [];
  sessionChanged();
  if (!state.current || !state.caps.assert) {
    session.pushUndo(snap);
    renderRail();
    if (state.current && session.size) toast('Change kept but not applied: this version of the decompiler cannot apply changes.', { kind: 'warn' });
    return true;
  }
  return reinspect({ snap, fresh, label, reselect, done });
}

async function reinspect({ snap = null, fresh = [], label = 'edit', reselect = null, done = '' } = {}) {
  const fn = state.current.fn;
  const oldCode = state.current.data.code;
  const scroll = captureScroll();
  const keepSel = reselect || state.sel;
  const op = beginOperation('edit');
  op.onCancel = () => {
    if (!snap) return;
    session.restore(snap);
    sessionChanged();
    toast('The change was stopped and undone.', { kind: 'warn' });
  };
  setStatus('Updating the code…');
  const t0 = performance.now();
  try {
    const doc = await fetchFunction(fn);
    if (!isCurrent(op)) return false;
    const data = normalizeInspect(doc);
    const wasApplied = new Set([...session.outcomes].filter(([, o]) => o.status === 'applied').map(([k]) => k));
    session.recordOutcomes(data.assertions);
    if (snap) session.pushUndo(snap);
    persist();
    cacheSet(fn.address_hex, data);
    showFunction(fn, data, { keep: true });
    restoreScroll(scroll);
    if (keepSel) selectTarget(keepSel, null);
    flash(changedLines(oldCode, data.code));
    if (!state.caps.assert) {
      setStatus('Change kept but not applied', 'err', 'This version of the decompiler cannot apply changes');
      renderRail();
      return false;
    }
    const mine = data.assertions.filter((r) => fresh.includes(r.directive));
    for (const row of mine.filter((r) => r.status === 'rejected')) {
      toast('That change could not be applied.', { kind: 'err', detail: `${row.detail || 'The decompiler refused it.'} (${row.directive})` });
    }
    const broke = data.assertions.filter((r) => r.status === 'rejected' && !fresh.includes(r.directive) &&
      wasApplied.has(session.sent.get(r.directive)));
    for (const row of broke) {
      toast('An earlier change no longer applies.', {
        kind: 'warn',
        detail: `${row.directive}: ${row.detail || 'the code changed under it'}. Edit or remove it under Your changes.`,
      });
    }
    if (data.assertions.some((r) => r.fatal)) {
      toast('The code shown was made without one of your changes.', { kind: 'err', detail: data.assertions.find((r) => r.fatal).detail || '' });
    }
    const dt = Math.round(performance.now() - t0);
    const bad = mine.some((r) => r.status === 'rejected');
    setStatus(bad ? 'That change was not applied' : (done || 'Updated the code'), bad ? 'err' : 'ok', `${label} · ${dt} ms`);
    renderRail();
    return !bad;
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return false;
    if (snap) {
      session.restore(snap);
      sessionChanged();
    }
    toast('The change could not be applied, so it was undone.', { kind: 'err', detail: errorLine(e) });
    setStatus('The change could not be applied', 'err', e.message);
    return false;
  } finally {
    finishOperation(op);
  }
}

function undo() {
  if (active?.kind === 'edit' || !session.undo()) return;
  sessionChanged();
  if (state.current && state.caps.assert) reinspect({ label: 'undo' });
}

function redo() {
  if (active?.kind === 'edit' || !session.redo()) return;
  sessionChanged();
  if (state.current && state.caps.assert) reinspect({ label: 'redo' });
}

/**
 * The panel's variables: the inputs, then the locals the C declares, then
 * (separately) what only the debug info knows and the C never uses.
 */
function railVars() {
  const { data, decls, index } = state.current;
  const sig = shownSignature();
  const vars = [];
  const debugVars = [];
  const shown = new Set();
  const home = (name) => data.variables.find((v) => v.name === name && v.kind !== 'arg' && Number.isInteger(v.stack_offset));
  (sig?.params || []).forEach((p, i) => {
    const h = home(p.name);
    vars.push({ name: p.name || `(input ${i + 1})`, type: p.type, where: `input ${i + 1}${h ? ', saved on the stack' : ''}`,
      title: h ? `${-h.stack_offset} bytes below the return address (${entryOffset(h.stack_offset)})` : '' });
    shown.add(p.name);
  });
  for (const d of decls) {
    vars.push({ name: d.name, type: d.type, where: d.storage ? whereWords(d.storage) : 'a temporary value', title: d.storage || '' });
    shown.add(d.name);
  }
  for (const v of data.variables) {
    if (shown.has(v.name) || v.kind === 'arg') continue;
    shown.add(v.name);
    const row = { name: v.name, type: v.type, where: Number.isInteger(v.stack_offset) ? whereWords('', v.stack_offset) : '' };
    if (index.symToLines.has(v.name)) vars.push(row);
    else debugVars.push(row);
  }
  return { vars, debugVars };
}

/** "Takes 2 inputs · returns int" from the signature the view shows. */
function fnSummary() {
  const sig = shownSignature();
  if (!sig) return [];
  const n = sig.params.length;
  const ret = sig.ret && sig.ret !== 'void' && sig.ret !== '()' ? `returns <code>${escapeHtml(sig.ret)}</code>` : 'returns nothing';
  return [`${n ? `Takes ${n} input${n === 1 ? '' : 's'}` : 'Takes no inputs'} · ${ret}`];
}

/** A change in plain words; the directive itself is the row's tooltip. */
function changeLabel(entry) {
  const rec = session.records.get(entry.key);
  const where = (func) => (func && func !== state.current?.data.address_hex ? ` in ${nameOfAddr(func)}` : '');
  if (entry.kind === 'bytes') {
    const m = /^bytes\s+(0x[0-9a-f]+)\s+([0-9a-f]+)/i.exec(entry.text);
    const n = m ? m[2].length / 2 : 0;
    return m ? `Patched ${n} byte${n === 1 ? '' : 's'} at ${bare(m[1])}` : entry.text;
  }
  if (!rec) return entry.text;
  switch (rec.kind) {
    case 'var':
      if (rec.name && rec.type) return `Renamed ${rec.sym} → ${rec.name}, type ${rec.type}${where(rec.func)}`;
      if (rec.name) return `Renamed ${rec.sym} → ${rec.name}${where(rec.func)}`;
      return `Changed ${rec.sym} to ${rec.type}${where(rec.func)}`;
    case 'fn': return `Renamed function ${state.byAddr.get(rec.addr)?.name || bare(rec.addr)} → ${rec.name}`;
    case 'proto': return `Signature of ${nameOfAddr(rec.addr)}: ${rec.decl}`;
    case 'data': return `Global at ${bare(rec.addr)}: ${rec.type} ${rec.name}`;
    case 'typedef': return `Defined the type ${rec.tag}`;
    case 'comment': return `Note at ${bare(rec.addr)}${where(rec.func)}: ${rec.text}`;
    default: return entry.text;
  }
}

function renderRail() {
  const edits = session.entries(nameOfAddr).map((e) => ({ ...e, label: changeLabel(e) }));
  const base = {
    edits,
    canUndo: session.canUndo,
    canRedo: session.canRedo,
    restored: state.restored?.count || 0,
    assertSupported: state.caps.assert,
    selectedHtml: selectedCard(),
  };
  if (!state.current) {
    rail.render({ ...base, fn: null });
    return;
  }
  const { data } = state.current;
  const { vars, debugVars } = railVars();
  rail.render({
    ...base,
    fn: { summary: fnSummary(), vars, debugVars, types: data.types, refsHtml: refsHtmlFor(data.address_hex) },
  });
  if (state.sel?.sym) rail.markVars(new Set([state.sel.sym]));
}

const importEl = document.createElement('input');
importEl.type = 'file';
importEl.accept = '.kuna,.txt,text/plain';
importEl.hidden = true;
document.body.appendChild(importEl);
/** A qualifier in a directive (a function's current or engine name) → its entry. */
function resolveFunc(name) {
  return (state.rows.find((r) => displayName(r.fn) === name)?.fn || state.byName.get(name))?.address_hex || null;
}

importEl.addEventListener('change', async () => {
  const file = importEl.files[0];
  if (!file || !state.binary) { importEl.value = ''; return; }
  let text;
  try { text = await file.text(); } finally { importEl.value = ''; }
  let counts = null;
  const bindTo = state.current?.data.address_hex ?? null;
  const ok = await applyEdit(() => {
    counts = session.importText(text, { resolveFunc, bindTo });
  }, { label: 'import' });
  if (counts) {
    toast(`Imported ${counts.added} change${counts.added === 1 ? '' : 's'} from ${file.name}.`, {
      kind: ok ? 'ok' : 'warn',
      detail: counts.raw ? `${counts.raw} of them are kept as written.` : '',
    });
  }
});

function exportSession() {
  if (!state.binary) return;
  const target = state.current ? displayName(state.current.fn) : 'main';
  const text = session.toFileText({ binary: state.binary.name, hash: state.binary.hash, target, nameOf: nameOfAddr });
  download(new Blob([text], { type: 'text/plain' }), `${state.binary.name}.kuna`);
}

/** Edit one session record from the rail, in place: its kind and its function stay. */
async function editEntry(key) {
  const rec = session.records.get(key);
  if (key.startsWith('bytes:')) {
    if (state.current) setTab('bytes');
    return;
  }
  if (!rec) return;
  const anchorEl = els.railBody.querySelector(`li[data-key="${CSS.escape(key)}"]`);
  const where = rec.func ? ` in ${nameOfAddr(rec.func)}` : '';
  if (rec.kind === 'var') {
    const fields = [{ name: 'name', label: 'Name', value: rec.name || rec.sym, validate: validateIdent }];
    if (!state.current?.rust) {
      fields.push({ name: 'type', label: 'Type (leave empty to keep the decompiler\'s)', value: rec.type || '', list: typeOptions(), validate: (v) => (v.trim() ? validateCType(v) : null) });
    }
    const res = await dialogs.openPopover({ anchorEl, title: `Change ${rec.sym}${where}`, note: `The decompiler first called it ${rec.sym}.`, fields, submitLabel: 'Save' });
    if (!res) return;
    const patch = { name: res.name };
    if ('type' in res) patch.type = res.type.trim() ? normalizeType(res.type) : null;
    return applyEdit(() => session.setVar(rec.func, rec.sym, patch), { label: 'edit' });
  }
  if (rec.kind === 'comment') {
    const res = await dialogs.openPopover({
      anchorEl, title: `Note at ${bare(rec.addr)}${where}`, note: 'Leave it empty to remove it.',
      fields: [{ name: 'text', label: 'Note', value: rec.text, validate: noHash }], submitLabel: 'Save note',
    });
    if (!res) return;
    return applyEdit(() => session.setComment(rec.func, rec.addr, res.text.trim() || null), { label: 'note', done: 'Saved the note' });
  }
  if (rec.kind === 'fn') {
    const fn = state.byAddr.get(rec.addr) || { address_hex: rec.addr, name: rec.addr };
    const res = await dialogs.openPopover({
      anchorEl, title: `Rename the function at ${bare(rec.addr)}`, note: `The decompiler first called it ${fn.name}.`,
      fields: [{ name: 'name', label: 'New name', value: rec.name, validate: validateIdent }], submitLabel: 'Rename',
    });
    if (!res) return;
    return applyEdit(() => session.setFunctionName(rec.addr, res.name === fn.name ? null : res.name), { label: 'rename' });
  }
  if (rec.kind === 'proto') return protoDialog(rec.addr, anchorEl);
  if (rec.kind === 'data') return dataDialog({ addr: rec.addr, name: rec.name, el: anchorEl });
  const text = session.entries(nameOfAddr).find((x) => x.key === key)?.text || '';
  const res = await dialogs.openPopover({
    anchorEl, title: 'Edit this change', note: 'Written the way the command line takes it (one --assert line).',
    fields: [{ name: 'text', label: 'directive', value: text, validate: (v) => (v.trim() ? noHash(v) : 'empty') }],
  });
  if (!res) return;
  const bindTo = rec.func ?? state.current?.data.address_hex ?? null;
  return applyEdit(() => session.replaceWith(key, res.text, { resolveFunc, bindTo }), { label: 'edit' });
}

const rail = createRail({
  root: els.railBody,
  on: async (act, data) => {
    switch (act) {
      case 'var-select': {
        const tok = els.ccode.querySelector(`.t[data-sym="${CSS.escape(data.sym)}"]`);
        selectTarget({ sym: data.sym }, null, tok);
        break;
      }
      case 'var-rename': renameTarget({ kind: 'var', name: data.sym, el: els.railBody.querySelector(`[data-sym="${CSS.escape(data.sym)}"]`) }); break;
      case 'sel-rename': renameSelected(); break;
      case 'sel-retype': retypeSelected(); break;
      case 'sel-comment': commentSelected(); break;
      case 'sel-line': {
        const n = Number(data.el.dataset.line);
        selectTarget({ line: n }, null);
        ensureShown('c');
        $('c-L' + n)?.scrollIntoView({ block: 'nearest' });
        break;
      }
      case 'sel-insn': gotoAddr(data.el.dataset.addr); break;
      case 'refs-load': state.refsOpen = true; loadRefs(); break;
      case 'edit-remove': applyEdit(() => session.remove(data.key), { label: 'removal', done: 'Removed the change' }); break;
      case 'edit-edit': editEntry(data.key); break;
      case 'undo': undo(); break;
      case 'redo': redo(); break;
      case 'export': exportSession(); break;
      case 'import': importEl.click(); break;
      case 'copy-cli': {
        const cmd = cliCommand(state.binary.name, state.current ? displayName(state.current.fn) : 'main');
        try {
          await navigator.clipboard.writeText(cmd);
          toast('Copied the command that applies these changes on the command line.', { detail: cmd });
        } catch (_) {
          toast('Copy this command:', { kind: 'warn', detail: cmd, ms: 12000 });
        }
        break;
      }
      case 'clear':
        if (await dialogs.confirmBox(`Remove all ${session.size} of your changes to ${state.binary.name}?`, { confirmLabel: 'Clear all' })) {
          applyEdit(() => session.clear(), { label: 'clear', done: 'Cleared all your changes' });
        }
        break;
      case 'discard-restored': {
        const mark = state.restored?.mark;
        state.restored = null;
        if (mark) applyEdit(() => session.discardMarked(mark), { label: 'discard', done: 'Discarded the restored changes' });
        break;
      }
      default: break;
    }
  },
});
renderRail();

els.fnRename.addEventListener('click', () => state.current && renameTarget({ kind: 'self', addr: state.current.data.address_hex, el: els.fnRename }));
els.proto.addEventListener('click', () => state.current && protoDialog(state.current.data.address_hex, els.proto));

// ── bytes: the hex dump, typed patches, Patch/NOP/Revert, the patched file ─

const bytesState = { edit: null, digits: '', burst: null, timer: 0, version: 0, memo: null };

function sections() {
  return state.inventory?.sections || [];
}

/** The byte the file (or, lacking sections, the engine's first answer) holds at `a`. */
function origByte(a) {
  const fromFile = originalByteAt(a, { fileBytes: state.binary?.bytes, sections: sections() });
  if (fromFile !== null) return fromFile;
  for (const insn of state.current?.data.instructions || []) {
    const base = BigInt(insn.address_hex);
    if (a >= base && a < base + BigInt(insn.size)) {
      const k = Number(a - base);
      return parseInt(insn.bytes.slice(k * 2, k * 2 + 2), 16);
    }
  }
  return null;
}

/** The hex dump's cells, rebuilt only when the function, the data view or the patches change. */
function bytesCells() {
  const key = { data: state.current?.data, view: state.dataView, version: bytesState.version };
  const memo = bytesState.memo;
  if (memo && memo.data === key.data && memo.view === key.view && memo.version === key.version) return memo.cells;
  let cells;
  if (state.dataView) {
    cells = dataCells();
  } else {
    const { data } = state.current;
    cells = functionCells(data.instructions, {
      entry: data.address_hex, size: data.size, fileBytes: state.binary?.bytes, sections: sections(), patches: session.bytes,
    });
  }
  bytesState.memo = { ...key, cells };
  return cells;
}

const BYTES_INFO = 'Click a byte and type two hex digits to change it. Changes are applied to the decompiled code right away.';

function renderBytes() {
  if (state.dataView) return renderData();
  const { data, index } = state.current;
  const cells = bytesCells();
  const selected = new Set();
  if (state.sel?.addr) selected.add(index.insnIndex.get(state.sel.addr));
  const off = fileOffsetFor(data.address_hex, sections());
  const patched = cells.filter((c) => c.patched).length;
  const where = off === null
    ? (sections().length ? 'These bytes are not stored in the file, so a patched copy cannot include them.' : '')
    : `${cells.length} bytes, starting ${off.toLocaleString()} bytes into the file.`;
  els.bytesbar.innerHTML = `<span class="d2-info">${escapeHtml(BYTES_INFO)}</span>` +
    `<span title="file offset 0x${off === null ? '' : off.toString(16)}">${escapeHtml(where)}${patched ? ` <b>${patched} changed</b>` : ''}</span>`;
  els.hexdump.innerHTML = renderHex(hexRows(cells), { sections: sections(), selected, editAddr: bytesState.edit });
  sync.refresh('bytes');
}

const DATA_SPAN = 256;

/** Cells for the data view: the page's file copy (or the engine's read) plus patches. */
function dataCells() {
  const { start, bytes } = state.dataView;
  return [...bytes].map((b, i) => {
    const a = start + BigInt(i);
    const patched = session.bytes.has(a);
    return { addr: a, hex: '0x' + a.toString(16), value: patched ? session.bytes.get(a) : b, orig: b, patched, insn: null, first: false, gap: false };
  });
}

function renderData() {
  const { focus, source } = state.dataView;
  const cells = bytesCells();
  els.bytesbar.innerHTML = `<span class="d2-info">Data at <b>${escapeHtml(bare('0x' + focus.toString(16)))}</b>: ${cells.length} bytes read from ${escapeHtml(source)}.</span>` +
    `<button class="d2-btn small" data-act="data-back">Back to ${escapeHtml(state.current ? displayName(state.current.fn) : 'the function')}</button>`;
  els.hexdump.innerHTML = renderHex(hexRows(cells), { sections: sections(), editAddr: bytesState.edit ?? focus });
}

/**
 * Show the bytes at a non-code address: from the file when it backs them, else
 * `read` without directives — the cells overlay the session's patches, and
 * Backspace restores the byte this read reports.
 */
async function showDataAt(hex) {
  const focus = BigInt(hex);
  const start = focus - (focus % 16n);
  const off = fileOffsetFor(start, sections());
  if (off !== null && state.binary) {
    state.dataView = { start, focus, bytes: state.binary.bytes.subarray(off, off + DATA_SPAN), source: 'the file' };
  } else if (!state.caps.inspect) {
    toast(`${bare(hex)} is not inside a function, and this version of the decompiler cannot show raw bytes.`, { kind: 'warn' });
    return;
  } else {
    const op = beginOperation('read');
    try {
      const res = await state.kuna.read('0x' + start.toString(16), DATA_SPAN, { assertions: [] });
      if (!isCurrent(op)) return;
      state.dataView = { start, focus, bytes: parseHex(res.bytes) || new Uint8Array(), source: 'the engine' };
    } catch (e) {
      if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
      toast(`Could not read the bytes at ${bare(hex)}.`, { kind: 'warn', detail: errorLine(e) });
      return;
    } finally {
      finishOperation(op);
    }
  }
  bytesState.edit = null;
  setTab('bytes');
  state.rendered.delete('bytes');
  renderVisible();
}

document.addEventListener('click', (e) => {
  if (!e.target.closest('[data-act=data-back]')) return;
  state.dataView = null;
  bytesState.edit = null;
  if (state.current) {
    state.rendered.delete('bytes');
    renderVisible();
  }
});

function patchButtons(insn) {
  if (!insn || !state.current?.data.hasInstructions) return '';
  const nop = nopFill(state.current.arch, insn.size);
  const base = BigInt(insn.address_hex);
  let any = false;
  for (let k = 0; k < insn.size; k++) if (session.bytes.has(base + BigInt(k))) any = true;
  return `<button class="d2-btn small" data-patch="nop" data-addr="${insn.address_hex}"${nop ? ' title="Make this instruction do nothing"' : ' disabled title="No do-nothing instruction fits this size"'}>Replace with NOP</button>` +
    `<button class="d2-btn small" data-patch="patch" data-addr="${insn.address_hex}" title="Type new bytes for this instruction">Edit bytes</button>` +
    (any ? `<button class="d2-btn small" data-patch="revert" data-addr="${insn.address_hex}" title="Put the original bytes back">Undo patch</button>` : '');
}

/** Write `values` at `addr` as one edit (a byte equal to the file's is no patch). */
function patchBytes(addr, values, label) {
  const base = BigInt(addr);
  return applyEdit(() => {
    values.forEach((v, i) => session.setByte(base + BigInt(i), v, origByte(base + BigInt(i))));
  }, { label, reselect: state.sel, done: `Patched ${values.length} byte${values.length === 1 ? '' : 's'} at ${bare(addr)}` });
}

async function patchAction(kind, addrHex) {
  const insn = state.current?.index.addrToInsn.get(addrHex);
  if (!insn) return;
  if (kind === 'nop') {
    const fill = nopFill(state.current.arch, insn.size);
    if (fill) patchBytes(addrHex, [...parseHex(fill)], 'NOP');
    return;
  }
  if (kind === 'revert') {
    const base = BigInt(addrHex);
    applyEdit(() => {
      for (let k = 0; k < insn.size; k++) session.bytes.delete(base + BigInt(k));
    }, { label: 'revert', reselect: state.sel, done: `Put back the original bytes at ${bare(addrHex)}` });
    return;
  }
  const current = [...Array(insn.size).keys()].map((k) => {
    const a = BigInt(addrHex) + BigInt(k);
    return (session.bytes.get(a) ?? parseInt(insn.bytes.slice(k * 2, k * 2 + 2), 16)).toString(16).padStart(2, '0');
  }).join(' ');
  const res = await dialogs.openPopover({
    anchorEl: $('a-' + addrHex),
    title: `Edit the bytes of ${asmText(insn)}`,
    note: `This instruction is ${insn.size} byte${insn.size === 1 ? '' : 's'}: ${current}`,
    fields: [{ name: 'hex', label: 'New bytes (hex)', value: current, validate: (v) => (parseHex(v) ? null : 'Type hex digits in pairs, like 90 90') }],
    warn: (v) => {
      const n = parseHex(v.hex)?.length || 0;
      if (!n || n === insn.size) return '';
      return n < insn.size
        ? `Only ${n} of ${insn.size} bytes: the rest of the old instruction stays and will be read as something else.`
        : `${n} bytes run past this ${insn.size}-byte instruction into the next one.`;
    },
    submitLabel: 'Patch',
  });
  if (res) patchBytes(addrHex, [...parseHex(res.hex)], 'patch');
}

document.addEventListener('click', (e) => {
  const btn = e.target.closest('[data-patch]');
  if (!btn || btn.disabled) return;
  e.preventDefault();
  patchAction(btn.dataset.patch, btn.dataset.addr);
});

els.hexdump.addEventListener('click', (e) => {
  const cell = e.target.closest('.hb[data-a]');
  if (!cell || !state.current) return;
  bytesState.edit = BigInt(cell.dataset.a);
  bytesState.digits = '';
  if (cell.dataset.i !== undefined) {
    const insn = state.current.data.instructions[Number(cell.dataset.i)];
    if (insn) selectTarget({ addr: insn.address_hex }, 'bytes');
  }
  renderBytes();
  els.hexdump.focus({ preventScroll: true });
});

/** Typing in the hex dump: two hex digits write a byte and move on. */
function hexKey(e) {
  if (bytesState.edit === null || document.activeElement !== els.hexdump) return false;
  const cells = bytesCells();
  const at = cells.findIndex((c) => c.addr === bytesState.edit);
  if (e.key === 'ArrowRight' || e.key === 'ArrowLeft') {
    const next = cells[at + (e.key === 'ArrowRight' ? 1 : -1)];
    if (next) { bytesState.edit = next.addr; bytesState.digits = ''; renderBytes(); }
    return true;
  }
  if (e.key === 'Backspace') {
    const cell = cells[at];
    if (cell?.patched) { startBurst(); session.setByte(cell.addr, cell.orig, cell.orig); byteWritten(); }
    return true;
  }
  if (!/^[0-9a-f]$/i.test(e.key)) return false;
  bytesState.digits += e.key.toLowerCase();
  if (bytesState.digits.length < 2) {
    els.hexdump.querySelector('.hb.ed')?.replaceChildren(document.createTextNode(bytesState.digits + '_'));
    return true;
  }
  const value = parseInt(bytesState.digits, 16);
  bytesState.digits = '';
  startBurst();
  session.setByte(bytesState.edit, value, origByte(bytesState.edit));
  const next = cells[at + 1];
  if (next) bytesState.edit = next.addr;
  byteWritten();
  return true;
}

function startBurst() {
  if (!bytesState.burst) bytesState.burst = { snap: session.snapshot(), before: new Set(session.globalAssertions()) };
}

/** One burst of typed bytes becomes one edit, sent 700 ms after the last key. */
function byteWritten() {
  persist();
  bytesState.version++;
  renderBytes();
  renderRail();
  syncPatchButton();
  clearTimeout(bytesState.timer);
  bytesState.timer = setTimeout(flushBytes, 700);
}

/**
 * Close a burst of typed bytes: one undo step now, and a re-inspect once no
 * other request is running (a flush never cancels what the student started).
 */
function flushBytes({ send = true } = {}) {
  clearTimeout(bytesState.timer);
  const burst = bytesState.burst;
  bytesState.burst = null;
  if (!burst || !state.current) return;
  const fresh = session.globalAssertions().filter((d) => !burst.before.has(d));
  session.pushUndo(burst.snap);
  sessionChanged();
  if (!send || !state.caps.assert) return;
  const fn = state.current.fn;
  whenIdle(() => {
    if (state.current?.fn === fn) reinspect({ fresh, label: 'patch', reselect: state.sel });
  });
}

function canDownloadPatched() {
  return !!(state.binary && session.byteRuns().length && sections().length);
}

/** The ⋯ menu's patched-program item, with the reason it is off. */
function syncPatchButton() {
  const ok = canDownloadPatched();
  els.patch.disabled = !ok;
  els.patchWhy.hidden = ok;
  els.patchWhy.textContent = !state.binary ? 'Open a program first'
    : !session.byteRuns().length ? 'Patch some bytes first' : 'This program has no file layout to patch';
}

els.hexdump.addEventListener('pointerover', (e) => {
  const cell = e.target.closest('.hb[data-a]');
  if (!cell || !state.current || state.dataView) return;
  const insn = cell.dataset.i !== undefined ? state.current.data.instructions[Number(cell.dataset.i)] : null;
  setHint(insn ? `Byte at ${escapeHtml(bare(cell.dataset.a))} belongs to <b>${escapeHtml(asmText(insn))}</b>` : `Byte at ${escapeHtml(bare(cell.dataset.a))}`);
  sync.hover(insn ? { addr: insn.address_hex } : null);
});
els.hexdump.addEventListener('pointerleave', () => {
  sync.hover(null);
  setHint(null);
});

els.patch.addEventListener('click', () => {
  if (!canDownloadPatched()) return;
  const { bytes, unmapped } = applyPatches(state.binary.bytes, sections(), session.byteRuns());
  if (unmapped.length) {
    toast('Some changed bytes are not stored in the file, so a patched copy cannot include them.', {
      kind: 'err', detail: unmapped.map((u) => `${u.count} byte${u.count === 1 ? '' : 's'} at ${u.addr}`).join(', '),
    });
    return;
  }
  const name = patchedName(state.binary.name);
  download(new Blob([bytes], { type: 'application/octet-stream' }), name);
  const fmt = state.binary.format || '';
  let detail = `${session.byteRuns().reduce((n, r) => n + r.values.length, 0)} bytes changed; nothing else in the file moved.`;
  if (/Mach-O/.test(fmt)) detail += ` macOS will not run a changed signed program until you sign it again: codesign -f -s - ${name}`;
  else if (/PE/.test(fmt)) detail += ' The file\'s checksum no longer matches; Windows only checks it for drivers.';
  else if (/ELF/.test(fmt)) detail += ` Run it with: chmod +x ${name} && ./${name}`;
  toast(`Downloaded ${name}.`, { detail, ms: 10000 });
});

// ── references and help ────────────────────────────────────────────────────

state.xrefs = new Map();

/** References depend on the program-wide directives (renames, patches), so they key the cache. */
function refsKey(addrHex) {
  return `${addrHex}\n${state.caps.assert ? session.globalAssertions().join('\n') : ''}`;
}

const refsNameOf = (a, n) => (state.byAddr.has(a) ? displayName(state.byAddr.get(a)) : n || a);

/** "Calls … / Called by …" for the open function: the engine's answer once asked for, else its own calls. */
function refsHtmlFor(addrHex) {
  const cached = state.xrefs.get(refsKey(addrHex));
  if (cached) return cached;
  const note = state.caps.inspect ? '' : 'Finding callers needs a newer version of the decompiler.';
  return renderLocalCalls(localCallees(state.current.data, state.byAddr), { nameOf: refsNameOf, note });
}

/** Ask the engine who calls the open function (once per session state), when nothing else is running. */
function loadRefs() {
  if (!state.current || !state.caps.inspect) return;
  const addr = state.current.data.address_hex;
  const key = refsKey(addr);
  if (state.xrefs.has(key)) {
    rail.setRefs(state.xrefs.get(key));
    return;
  }
  rail.setRefs(refsHtmlFor(addr).replace(/<button class="d2-link" data-act="refs-load">[^<]*<\/button>/, '<span class="d2muted">Finding who calls it…</span>'));
  whenIdle(async () => {
    if (state.current?.data.address_hex !== addr) return;
    const op = beginOperation('xrefs');
    let html;
    try {
      const res = await withDirectives(
        (list) => state.kuna.xrefs(addr, { assertions: list }),
        state.caps.assert ? session.globalAssertions() : [],
      );
      if (!isCurrent(op)) return;
      html = renderXrefs(res, { nameOf: refsNameOf });
    } catch (e) {
      if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
      html = renderLocalCalls(localCallees(state.current.data, state.byAddr), {
        nameOf: refsNameOf, note: `Could not find the callers: ${errorLine(e)}`,
      });
    } finally {
      finishOperation(op);
    }
    state.xrefs.set(key, html);
    if (state.current?.data.address_hex === addr) rail.setRefs(html);
  });
}

function openRefs() {
  if (!state.current) return;
  setRail(true);
  state.refsOpen = true;
  loadRefs();
  $('railrefsbody')?.scrollIntoView({ block: 'nearest' });
}

els.railBody.addEventListener('click', (e) => {
  const a = e.target.closest('a.xt[data-goto]');
  if (!a) return;
  e.preventDefault();
  gotoAddr(a.dataset.goto);
});

function openHelp({ allKeys = false } = {}) {
  if (els.helpDialog.open) {
    els.helpDialog.close();
    return;
  }
  hover.hide();
  closeMenus();
  els.helpDialog.innerHTML = helpHtml();
  if (allKeys) $('helpall').open = true;
  els.helpDialog.showModal();
  if (allKeys) $('helpall').scrollIntoView({ block: 'start' });
}
els.help.addEventListener('click', () => openHelp());
els.helpDialog.addEventListener('click', (e) => {
  if (e.target === els.helpDialog || e.target.closest('[data-act=help-close]')) els.helpDialog.close();
});

// ── project export ─────────────────────────────────────────────────────────

els.dl.addEventListener('click', async () => {
  if (!state.inventory || !state.kuna) return;
  const op = beginOperation('project');
  setStatus('Preparing the C code…');
  try {
    const project = await withDirectives(
      (list) => state.kuna.project(state.binary.name, { assertions: list }),
      state.caps.assert ? session.allAssertions(nameOfAddr) : [],
    );
    if (!isCurrent(op)) return;
    download(new Blob([project.bytes], { type: 'application/zip' }), project.downloadName);
    setStatus(`Downloaded ${project.downloadName}`, 'ok', `${project.bytes.length.toLocaleString()} bytes`);
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    setStatus('Could not prepare the C code', 'err', e.message);
    toast('Could not prepare the C code download.', { kind: 'err', detail: errorLine(e) });
    console.error(e);
  } finally {
    finishOperation(op);
  }
});

function download(blob, name) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = name;
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

