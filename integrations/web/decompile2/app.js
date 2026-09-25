// app.js — the /decompile2 study view: loads a binary into the decompiler
// Worker, lists its functions, and shows one function as linked views (C,
// assembly, bytes, stack frame). Every engine string is escaped by the pure
// renderers this module mounts; this file owns the DOM, the operation model
// and the keyboard.
import {
  KunaWorkerCancelledError,
  KunaWorkerClient,
} from '../kuna-worker-client.js';
import { escapeHtml, highlightC } from '../assets/js/highlight-c.js';
import {
  headline,
  searchKey,
  stubDividerLabel,
  summarize,
} from '../assets/js/fnfilter.js';
import {
  normalizeInspect,
  lineSegments,
  buildIndex,
  renderC,
  localDecls,
  storageLabel,
} from './render-c.js';
import { loadPrefs, savePrefs, cycle, DEFAULT_PREFS } from './prefs.js';
import { renderAsm, renderInsnRows, formatAddr, spacedBytes } from './asm-view.js';
import { createHover } from './hover.js';
import { createSync } from './sync.js';

const $ = (id) => document.getElementById(id);
const els = {
  status: $('status'), cancel: $('cancelbtn'), dl: $('dlbtn'), patch: $('patchbtn'),
  mode: $('mode'), lang: $('lang'), example: $('examplebtn'), pick: $('pick'), file: $('file'),
  help: $('helpbtn'), progress: $('progress'), work: $('work'),
  list: $('fnlist'), head: $('fnhead'), filter: $('fnfilter'), none: $('fnnone'),
  empty: $('empty'), vhead: $('vhead'), vname: $('vname'), vmeta: $('vmeta'),
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
  binary: null,          // {name, bytes, format, example}
  inventory: null,       // the `list` document
  rows: [],              // sidebar rows [{row, fn, key, stub}]
  stubDiv: null,
  byAddr: new Map(),
  byName: new Map(),
  current: null,         // {fn, data, segs, index}
  cache: new Map(),      // address_hex -> normalized function (LRU)
  caps: { inspect: null, assert: null },
  exampleSource: null,
  hist: { index: 0, max: 0 },
  tab: 'c',
  rendered: new Set(),
};
const CACHE_MAX = 32;

// ── status, progress, toasts ────────────────────────────────────────────────

function setStatus(msg, cls = 'busy') {
  els.status.className = 'status ' + cls;
  els.status.innerHTML = '<span class="dot"></span>' + escapeHtml(msg);
}

function toast(message, { kind = 'ok', detail = '', ms = 6000 } = {}) {
  const el = document.createElement('div');
  el.className = 'd2-toast ' + kind;
  el.setAttribute('role', kind === 'err' ? 'alert' : 'status');
  el.innerHTML = escapeHtml(message) + (detail ? `<span class="dt">${escapeHtml(detail)}</span>` : '');
  els.toasts.appendChild(el);
  while (els.toasts.children.length > 4) els.toasts.firstChild.remove();
  setTimeout(() => el.remove(), ms);
  return el;
}

// ── the operation model: one engine request at a time ──────────────────────

let active = null;
let opSeq = 0;
const idleQueue = [];

function syncButtons() {
  els.cancel.disabled = !active;
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
  setStatus(
    state.inventory ? `cancelled — ${state.binary.name} remains loaded` : 'cancelled — load another binary',
    state.inventory ? 'ok' : '',
  );
});

// ── engine calls with graceful degradation ─────────────────────────────────

const isUnknownCommand = (e) => /unknown command/.test(e?.message || '');
const isUnknownAssert = (e) => /unknown option --assert/.test(e?.message || '');

/** Directives for one function; empty when the engine cannot take them. */
function directivesFor(addrHex) {
  if (state.caps.assert === false) return [];
  return session.assertionsFor(addrHex);
}

/**
 * Inspect one function, falling back to `decompile` on an engine without
 * `inspect`, and to no directives on one without `--assert`.
 */
async function fetchFunction(fn, assertions) {
  const call = async (list) => {
    if (state.caps.inspect !== false) {
      try {
        const doc = await state.kuna.inspect(fn.address_hex, { assertions: list });
        state.caps.inspect = true;
        return doc;
      } catch (e) {
        if (!isUnknownCommand(e)) throw e;
        state.caps.inspect = false;
      }
    }
    return state.kuna.decompile(fn.address_hex, { assertions: list });
  };
  try {
    const doc = await call(assertions);
    if (assertions.length) state.caps.assert = true;
    return doc;
  } catch (e) {
    if (!assertions.length || !isUnknownAssert(e)) throw e;
    state.caps.assert = false;
    toast('This engine build cannot apply edits yet.', {
      kind: 'warn',
      detail: 'Your edits are kept in the session and apply once the engine supports --assert.',
    });
    return call([]);
  }
}

// ── the session (edits). Replaced by session.js in the edit milestone. ─────

const session = {
  globalAssertions: () => [],
  assertionsFor: () => [],
  allAssertions: () => [],
  displayName: (fn) => fn.name,
};

// ── startup ────────────────────────────────────────────────────────────────

try {
  state.kuna = new KunaWorkerClient({ wasmUrl: '../kuna_wasm.wasm', specRoot: '../specs' });
  await state.kuna.ready();
  setStatus('ready — load an ELF, PE, or Mach-O binary, or try the example', 'ok');
  els.pick.removeAttribute('aria-disabled');
  els.example.disabled = false;
} catch (e) {
  setStatus('failed to load decompiler: ' + e.message, 'err');
  console.error(e);
}

// ── inventory and sidebar ──────────────────────────────────────────────────

const isStub = (fn) => fn.kind === 'plt' || fn.kind === 'thunk';

function resetList() {
  els.list.innerHTML = '';
  state.rows = [];
  state.stubDiv = null;
  els.head.textContent = 'functions';
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

function showEmpty(on) {
  els.empty.hidden = !on;
  els.vhead.hidden = on;
  els.tabbar.hidden = on;
  els.panes.hidden = on;
  els.hint.hidden = on;
}

function applyFilter() {
  const summary = summarize(state.rows, els.filter.value);
  state.rows.forEach((entry, i) => { entry.row.hidden = !summary.matches[i]; });
  els.head.textContent = headline(summary);
  if (state.stubDiv) {
    state.stubDiv.textContent = stubDividerLabel(summary);
    state.stubDiv.hidden = summary.stubs.matched === 0;
  }
  els.filter.classList.toggle('bad', !!summary.error);
  if (summary.error) els.filter.title = summary.error;
  else els.filter.removeAttribute('title');
  els.none.textContent = summary.error ? `invalid regex: ${summary.error}` : 'no matches';
  els.none.hidden = state.rows.length === 0 || summary.matches.some(Boolean);
}

const visibleRows = () => state.rows.filter((entry) => !entry.row.hidden);

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
    const shown = session.displayName(entry.fn);
    entry.row.querySelector('.nm').textContent = shown;
    entry.row.classList.toggle('renamed', shown !== entry.fn.name);
    entry.key = searchKey({ ...entry.fn, aliases: [...(entry.fn.aliases || []), shown] });
  }
}

function buildSidebar(functions) {
  const byAddr = (a, b) => (BigInt(a.address_hex) < BigInt(b.address_hex) ? -1 : 1);
  const normal = functions.filter((fn) => !isStub(fn)).sort(byAddr);
  const stubs = functions.filter(isStub).sort(byAddr);
  const frag = document.createDocumentFragment();
  const addRow = (fn) => {
    const row = document.createElement('div');
    row.className = 'fn' + (isStub(fn) ? ' stub' : '') + (fn.error ? ' bad' : '');
    row.tabIndex = 0;
    row.setAttribute('role', 'button');
    row.dataset.addr = fn.address_hex;
    row.innerHTML = `<span class="nm">${escapeHtml(fn.name)}</span><span class="ad">${escapeHtml(fn.address_hex)}</span>`;
    row.addEventListener('click', () => openFunction(fn));
    row.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openFunction(fn); return; }
      if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp') return;
      e.preventDefault();
      const shown = visibleRows();
      const at = shown.findIndex((entry) => entry.row === row);
      shown[at + (e.key === 'ArrowDown' ? 1 : -1)]?.row.focus();
    });
    frag.appendChild(row);
    state.rows.push({ row, fn, key: searchKey(fn), stub: isStub(fn) });
  };
  normal.forEach(addRow);
  if (stubs.length) {
    state.stubDiv = document.createElement('div');
    state.stubDiv.className = 'fndiv label';
    frag.appendChild(state.stubDiv);
    stubs.forEach(addRow);
  }
  els.list.appendChild(frag);
  els.filter.disabled = false;
  refreshRowNames();
  applyFilter();
}

function markSelectedRow(addrHex) {
  for (const entry of state.rows) {
    const on = entry.fn.address_hex === addrHex;
    entry.row.classList.toggle('sel', on);
    if (on) entry.row.scrollIntoView({ block: 'nearest' });
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
  setStatus(`reading ${name}…`);
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
    if (!isCurrent(op)) return;
    state.binary = { name, bytes, example, format: null };
    setStatus(`indexing ${name} (${bytes.length.toLocaleString()} bytes)…`);
    inventory = await state.kuna.load(bytes, {
      fileName: name,
      mode: els.mode.value,
      language: els.lang.value,
      assertions: state.caps.assert === false ? [] : session.globalAssertions(),
    });
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    setStatus('function inventory failed: ' + e.message, 'err');
    console.error(e);
    finishOperation(op);
    return;
  }
  if (!isCurrent(op)) return;
  state.binary.format = inventory.format;
  state.inventory = inventory;
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
  const dt = Math.round(performance.now() - t0);
  setStatus(`${name} (${inventory.format}) — ${inventory.functions.length} functions indexed in ${dt} ms`, 'ok');
  finishOperation(op);
  const fromHash = state.byAddr.get((location.hash || '').slice(1).toLowerCase());
  const first = (keep && state.byAddr.get(keep)) || fromHash || state.byName.get('main') ||
    state.rows.find((r) => !r.stub)?.fn || state.rows[0]?.fn;
  if (first) openFunction(first, { replace: true });
}

els.file.addEventListener('change', () => {
  const f = els.file.files[0];
  if (f) indexBinary(f);
  els.file.value = '';
});

const reindex = () => {
  if (state.binary) indexBinary(state.binary, { example: state.binary.example, keep: state.current?.fn.address_hex });
};
els.mode.addEventListener('change', reindex);
els.lang.addEventListener('change', reindex);

els.example.addEventListener('click', async () => {
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
});

// ── opening a function ─────────────────────────────────────────────────────

function cacheGet(addrHex) {
  const hit = state.cache.get(addrHex);
  if (hit) { state.cache.delete(addrHex); state.cache.set(addrHex, hit); }
  return hit;
}

function cacheSet(addrHex, data) {
  state.cache.delete(addrHex);
  state.cache.set(addrHex, data);
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
    if (!active) setStatus(`${session.displayName(fn)} — ${fn.address_hex} from cache`, cached.error ? 'err' : 'ok');
    return;
  }
  const op = beginOperation('function');
  const label = session.displayName(fn);
  setStatus(`decompiling ${label} @ ${fn.address_hex}…`);
  if (!keepView) {
    state.current = null;
    els.vname.textContent = label;
    els.vmeta.textContent = fn.address_hex;
    els.ccode.innerHTML = `<div class="d2note">decompiling ${escapeHtml(label)} @ ${escapeHtml(fn.address_hex)}…</div>`;
    for (const pane of [els.asmcode, els.hexdump, els.stackframe]) pane.innerHTML = '';
  }
  const t0 = performance.now();
  try {
    const doc = await fetchFunction(fn, directivesFor(fn.address_hex));
    if (!isCurrent(op)) return;
    const data = normalizeInspect(doc);
    cacheSet(fn.address_hex, data);
    showFunction(fn, data, { focusAddr });
    const dt = Math.round(performance.now() - t0);
    const verb = data.source === 'inspect' ? 'inspected' : 'decompiled';
    setStatus(`${label} — ${fn.address_hex} ${verb} in ${dt} ms`, data.error ? 'err' : 'ok');
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    els.ccode.innerHTML = `<div class="d2note">${escapeHtml(`${label} @ ${fn.address_hex} — decompile failed:\n${e.message}`)}</div>`;
    setStatus('decompile failed: ' + e.message, 'err');
    console.error(e);
  } finally {
    finishOperation(op);
  }
}

// ── rendering ──────────────────────────────────────────────────────────────

function fnMeta(fn, data) {
  const tags = [fn.address_hex];
  if (data.size) tags.push(`${data.size} B`);
  if (data.hasInstructions) tags.push(`${data.instructions.length} insns`);
  if (isStub(fn)) tags.push(fn.kind);
  if (data.error) tags.push('decompile error');
  return tags.join(' · ');
}

function showFunction(fn, data, { focusAddr = null } = {}) {
  const { segs } = lineSegments(data);
  const index = buildIndex(data, segs);
  state.current = { fn, data, segs, index, decls: localDecls(data.code) };
  state.rendered.clear();
  state.sel = null;
  state.cursor = null;
  hover.hide();
  sync.setIndex(index);
  els.vname.textContent = session.displayName(fn);
  els.vmeta.textContent = fnMeta(fn, data);
  const row = state.rows.find((r) => r.fn === fn)?.row;
  row?.classList.toggle('bad', !!data.error);
  renderVisible();
  setHint(null);
  if (focusAddr) selectTarget({ addr: focusAddr }, null);
}

const DEFAULT_HINT = 'click a name to select it · double-click to rename · <kbd>Space</kbd> C ⇄ assembly · <kbd>?</kbd> shortcuts';

/** The status line under the panes: what is selected and what keys do. */
function setHint(html) {
  els.hint.innerHTML = html || DEFAULT_HINT;
}

function paneFor(tab) {
  return { c: 'pane-c', asm: 'pane-asm', bytes: 'pane-bytes', stack: 'pane-stack', src: 'pane-src' }[tab];
}

/** The tabs on screen: the active one, plus C beside it in split view. */
function visibleTabs() {
  if (!state.prefs.split || window.matchMedia('(max-width: 820px)').matches) return [state.tab];
  if (state.tab === 'c') return ['c', 'asm'];
  return ['c', state.tab];
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
  return `<div class="d2note">${escapeHtml(what)} needs the engine's <code>inspect</code> surface, ` +
    'which this build of the decompiler does not have yet. The C view works; this view ' +
    'fills in once the engine answers <code>inspect</code>.</div>';
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
  },
  bytes() {
    els.bytesbar.innerHTML = '';
    els.hexdump.innerHTML = needsInspect('The bytes view');
  },
  stack() {
    els.stackframe.innerHTML = needsInspect('The stack-frame diagram');
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
  els.asmcode.classList.toggle('no-bytes', !p.asmBytes);
  els.asmcode.classList.toggle('no-ccom', p.asmCMode === 'off');
}

function asmContext(data) {
  return {
    prefs: state.prefs,
    codeLines: data.code.split('\n'),
    fnByAddr: state.byAddr,
    nameOf: (fn) => session.displayName(fn),
  };
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

const byId = (id) => document.getElementById(id);

sync.register('c', markerView((sets, cls, target) => {
  const out = [];
  if (!target?.sym || cls === 'hl-hover') {
    for (const line of sets.lines) { const el = byId('c-L' + line); if (el) out.push(el); }
  }
  if (cls === 'hl-sel') {
    for (const sym of sets.syms) out.push(...els.ccode.querySelectorAll(`.t[data-sym="${CSS.escape(sym)}"]`));
  }
  return out;
}, (target, sets) => {
  if (!isShown('c') || !sets.lines.size) return;
  byId('c-L' + Math.min(...sets.lines))?.scrollIntoView({ block: 'nearest' });
}));

sync.register('asm', markerView((sets) => {
  const out = [];
  for (const a of sets.addrs) { const el = byId('a-' + a); if (el) out.push(el); }
  return out;
}, (target, sets) => {
  if (!isShown('asm') || !sets.addrs.size) return;
  const index = state.current?.index;
  const first = [...sets.addrs].sort((a, b) => (index?.insnIndex.get(a) ?? 0) - (index?.insnIndex.get(b) ?? 0))[0];
  byId('a-' + first)?.scrollIntoView({ block: 'nearest' });
}));

/** Describe a variable from every source the engine gives. */
function describeVar(name) {
  const { data, decls } = state.current;
  const decl = decls.find((d) => d.name === name);
  const vars = data.variables.filter((v) => v.name === name);
  const param = vars.find((v) => v.kind === 'arg');
  const stack = vars.find((v) => Number.isInteger(v.stack_offset));
  const kind = param ? 'parameter' : decl || stack ? 'local' : /^dat_|^g_/.test(name) ? 'global' : 'name';
  const type = decl?.type || param?.type || stack?.type || '';
  const where = [];
  if (param && Number.isInteger(param.arg_index)) where.push(`argument ${param.arg_index}`);
  if (decl) where.push(storageLabel(decl.storage));
  else if (stack) where.push(`stack entry${stack.stack_offset < 0 ? '−' : '+'}0x${Math.abs(stack.stack_offset).toString(16)}`);
  return { name, kind, type, where: where.join(' · ') };
}

function varSummary(name) {
  const v = describeVar(name);
  return [v.name, v.kind, v.type, v.where].filter(Boolean).join(' · ');
}

function lineCard(n, varTok) {
  const { data, index, decls } = state.current;
  const addrs = index.lineToInsns.get(n) || [];
  const text = data.code.split('\n')[n - 1] || '';
  let html;
  if (addrs.length) {
    const insns = addrs.map((a) => index.addrToInsn.get(a)).filter(Boolean);
    html = `<div class="ch">L${n} → ${addrs.length} instruction${addrs.length === 1 ? '' : 's'}</div>`;
    if (insns.length) {
      html += renderInsnRows(insns, { startHex: data.address_hex, prefs: state.prefs, max: 12 });
      if (insns.length > 12) html += `<div class="cm">… ${insns.length - 12} more, click to open in Assembly</div>`;
    } else {
      html += `<div class="cm">at ${escapeHtml(addrs.join(', '))} — the instruction listing needs the engine's inspect surface</div>`;
    }
  } else {
    const decl = decls.find((d) => d.line === n);
    let why;
    if (decl) why = `declares ${decl.name} (${decl.type}, ${storageLabel(decl.storage)}). A declaration emits no instructions: the name is the decompiler's, the storage is the program's.`;
    else if (n === 1) why = `is the signature: the decompiler's reading of how ${data.name} is called.`;
    else if (/^\s*[{}]?\s*$/.test(text)) why = 'is structure the decompiler prints; no instruction belongs to it.';
    else if (!data.hasInstructions && !data.line_mappings.length) why = 'has no instruction map from this engine build (it needs the inspect surface).';
    else why = 'has no instruction of its own — its work was folded into a neighbouring line\'s instructions.';
    html = `<div class="ch">L${n}</div><div class="cx">This line ${escapeHtml(why)}</div>`;
  }
  if (varTok) html += `<div class="cx"><b>${escapeHtml(varSummary(varTok.textContent))}</b> — <kbd>n</kbd> rename · <kbd>y</kbd> retype</div>`;
  return html;
}

function calleeCard(addrHex) {
  const fn = state.byAddr.get(addrHex);
  if (!fn) return `<div class="ch">${escapeHtml(addrHex)}</div><div class="cm">not a function in this binary's inventory</div>`;
  const cached = state.cache.get(addrHex);
  const kind = isStub(fn) ? ` · ${fn.kind}` : '';
  let html = `<div class="ch">${escapeHtml(session.displayName(fn))} — ${escapeHtml(addrHex)}${fn.size ? ` · ${fn.size} B` : ''}${kind}</div>`;
  if (cached?.proto) html += `<pre>${escapeHtml(cached.proto)}</pre>`;
  html += '<div class="cm">double-click or <kbd>Enter</kbd> opens it</div>';
  return html;
}

function insnCard(addrHex) {
  const { data, index } = state.current;
  const insn = index.addrToInsn.get(addrHex);
  if (!insn) return null;
  const lines = index.insnToLines.get(addrHex) || [];
  const code = data.code.split('\n');
  let html = `<div class="ch">${escapeHtml(formatAddr(addrHex, data.address_hex, 'both'))} · ${escapeHtml(insn.mnemonic)} · ${insn.size} byte${insn.size === 1 ? '' : 's'}</div>`;
  html += `<div class="cm">${escapeHtml(spacedBytes(insn.bytes))}${insn.file_offset !== null ? ` · file offset 0x${insn.file_offset.toString(16)}` : ''}</div>`;
  if (lines.length) {
    html += `<pre>${lines.map((l) => `L${l}  ${escapeHtml((code[l - 1] || '').trim())}`).join('\n')}</pre>`;
  } else {
    html += '<div class="cx">Not mapped to a C line: frame setup, a value the decompiler folded into another statement, or code it proved dead.</div>';
  }
  return html;
}

function targetCard(hex) {
  const { data, index } = state.current;
  if (index.addrToInsn.has(hex)) {
    return `<div class="ch">${escapeHtml(hex)} · inside ${escapeHtml(data.name)} at ${escapeHtml(formatAddr(hex, data.address_hex, 'rel'))}</div><div class="cm">click to jump there</div>`;
  }
  return calleeCard(hex);
}

function slotCard(so) {
  const slot = so.dataset.slot;
  let html = `<div class="ch">stack slot ${escapeHtml(so.textContent)}</div>`;
  if (slot !== undefined) {
    const off = Number(slot);
    html += `<div class="cm">entry${off < 0 ? '−' : '+'}0x${Math.abs(off).toString(16)} — measured from the stack pointer at the function's entry</div>`;
  }
  return html;
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
  const active = target?.addr ? 'a-' + target.addr : Number.isInteger(target?.line) ? 'c-L' + target.line : null;
  if (target?.line || target?.sym) {
    const line = target.line ?? (sets && sets.lines.size ? Math.min(...sets.lines) : null);
    if (line) els.ccode.setAttribute('aria-activedescendant', 'c-L' + line);
  }
  if (target?.addr) els.asmcode.setAttribute('aria-activedescendant', active);
  if (target?.sym) {
    setHint(`<b>${escapeHtml(varSummary(target.sym))}</b> — <kbd>n</kbd> rename · <kbd>y</kbd> retype · <kbd>Esc</kbd> clears`);
  } else if (target?.addr) {
    const insn = state.current.index.addrToInsn.get(target.addr);
    setHint(insn ? `${escapeHtml(insn.address_hex)} <b>${escapeHtml(insn.text)}</b> — <kbd>;</kbd> comment · <kbd>x</kbd> references` : null);
  } else if (Number.isInteger(target?.line)) {
    const n = (state.current.index.lineToInsns.get(target.line) || []).length;
    setHint(`L${target.line} — ${n} instruction${n === 1 ? '' : 's'} · click the address gutter to open them in Assembly`);
  } else {
    setHint(null);
  }
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
  else toast(`${hex} is not inside a known function.`, { kind: 'warn' });
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
  if (tok.dataset.kind === 'funcname' && tok.dataset.callee && tok.dataset.callee !== state.current.data.address_hex) {
    e.preventDefault();
    openCallee(tok);
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
      const cur = state.sel?.line ?? (state.sel?.sym ? null : 0) ?? 0;
      const next = Math.min(Math.max((cur || 0) + step, 1), index.lineCount);
      selectTarget({ line: next }, null);
      byId('c-L' + next)?.scrollIntoView({ block: 'nearest' });
      hover.showFor(byId('c-L' + next)?.querySelector('.ct'));
    } else {
      const insns = state.current.data.instructions;
      const at = state.sel?.addr ? index.insnIndex.get(state.sel.addr) : -1;
      const next = insns[Math.min(Math.max(at + step, 0), insns.length - 1)];
      if (next) {
        selectTarget({ addr: next.address_hex }, null);
        byId('a-' + next.address_hex)?.scrollIntoView({ block: 'nearest' });
        hover.showFor(byId('a-' + next.address_hex));
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

// ── tabs, split, view menu ─────────────────────────────────────────────────

function setTab(tab) {
  if (tab === 'src' && $('tab-src').hidden) return;
  state.tab = tab;
  if (tab !== 'src') updatePrefs({ tab });
  const shown = visibleTabs();
  for (const btn of els.tabs.querySelectorAll('[role=tab]')) {
    const t = btn.dataset.tab;
    btn.setAttribute('aria-selected', String(t === tab));
    btn.classList.toggle('d2-side', t !== tab && shown.includes(t));
    btn.tabIndex = t === tab ? 0 : -1;
  }
  for (const t of ['c', 'asm', 'bytes', 'stack', 'src']) $(paneFor(t)).hidden = !shown.includes(t);
  els.panes.classList.toggle('split', shown.length > 1);
  els.split.setAttribute('aria-pressed', String(state.prefs.split));
  renderVisible();
}

els.tabs.addEventListener('click', (e) => {
  const btn = e.target.closest('[role=tab]');
  if (btn) setTab(btn.dataset.tab);
});
els.tabs.addEventListener('keydown', (e) => {
  if (e.key !== 'ArrowRight' && e.key !== 'ArrowLeft') return;
  const tabs = [...els.tabs.querySelectorAll('[role=tab]:not([hidden])')];
  const at = tabs.findIndex((t) => t.dataset.tab === state.tab);
  const next = tabs[(at + (e.key === 'ArrowRight' ? 1 : tabs.length - 1)) % tabs.length];
  setTab(next.dataset.tab);
  next.focus();
});

function updatePrefs(patch) {
  state.prefs = { ...state.prefs, ...patch };
  savePrefs(storage, state.prefs);
}

els.split.addEventListener('click', () => toggleSplit());
function toggleSplit() {
  updatePrefs({ split: !state.prefs.split });
  setTab(state.tab);
}

const VIEW_ITEMS = [
  { key: 'asmAddr', label: 'Instruction addresses', kbd: 'o' },
  { key: 'asmBytes', label: 'Bytes column', kbd: 'b' },
  { key: 'asmCMode', label: 'C in the assembly' },
  { key: 'asmArrows', label: 'Branch arrows' },
  { key: 'asmHints', label: 'Idiom hints' },
  { key: 'cLineNumbers', label: 'C line numbers' },
  { key: 'cLineAddrs', label: 'C line addresses' },
  { key: 'hoverDelay', label: 'Hover card' },
  { key: 'split', label: 'Split view', kbd: 's' },
];

const VALUE_LABEL = {
  asmAddr: { abs: 'absolute', rel: 'offset', both: 'both' },
  asmCMode: { comment: 'comments', interleave: 'interleaved', off: 'off' },
  hoverDelay: { 0: 'instant', 250: '250 ms', 450: '450 ms', 800: '800 ms', '-1': 'off' },
};

function renderViewMenu() {
  const p = state.prefs;
  els.viewMenu.innerHTML = VIEW_ITEMS.map((item) => {
    const value = p[item.key];
    const shown = VALUE_LABEL[item.key]?.[value] ?? (typeof value === 'boolean' ? (value ? 'on' : 'off') : String(value));
    return `<button class="d2-mi" role="menuitem" data-pref="${item.key}">${escapeHtml(item.label)}` +
      `<span class="v${value === true ? ' on' : ''}">${escapeHtml(shown)}${item.kbd ? ` <kbd>${item.kbd}</kbd>` : ''}</span></button>`;
  }).join('') + '<hr><button class="d2-mi" role="menuitem" data-pref="reset">Reset view settings</button>';
}

function toggleViewMenu(open = els.viewMenu.hidden) {
  els.viewMenu.hidden = !open;
  els.viewBtn.setAttribute('aria-expanded', String(open));
  if (open) {
    renderViewMenu();
    els.viewMenu.querySelector('.d2-mi')?.focus();
  }
}

els.viewBtn.addEventListener('click', () => toggleViewMenu());
els.viewMenu.addEventListener('click', (e) => {
  const item = e.target.closest('[data-pref]');
  if (!item) return;
  const key = item.dataset.pref;
  if (key === 'split') toggleSplit();
  else if (key === 'reset') updatePrefs({ ...DEFAULT_PREFS, tab: state.tab, split: state.prefs.split });
  else if (typeof state.prefs[key] === 'boolean') updatePrefs({ [key]: !state.prefs[key] });
  else updatePrefs(cycle(state.prefs, key));
  applyPaneClasses();
  if (['asmCMode', 'asmArrows', 'asmHints', 'reset'].includes(key)) rerender('asm');
  renderViewMenu();
  els.viewMenu.querySelector(`[data-pref="${key}"]`)?.focus();
});
document.addEventListener('mousedown', (e) => {
  if (!els.viewMenu.hidden && !e.target.closest('.d2-menuwrap')) toggleViewMenu(false);
});

// ── the rail ───────────────────────────────────────────────────────────────

function setRail(open) {
  els.work.classList.toggle('norail', !open);
  els.railBtn.setAttribute('aria-expanded', String(open));
  els.railBtn.innerHTML = open ? '&rsaquo;' : '&lsaquo;';
  els.railBtn.title = open ? 'Collapse details' : 'Show details';
}
els.railBtn.addEventListener('click', () => {
  const open = els.work.classList.contains('norail');
  setRail(open);
  if (!window.matchMedia('(max-width: 1180px)').matches) updatePrefs({ rail: open });
});
setRail(state.prefs.rail && !window.matchMedia('(max-width: 1180px)').matches);

// ── the keyboard ───────────────────────────────────────────────────────────

const typing = (el) => el && (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA' ||
  el.tagName === 'SELECT' || el.isContentEditable);

document.addEventListener('keydown', (e) => {
  if (e.altKey && !e.ctrlKey && !e.metaKey && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
    e.preventDefault();
    if (e.key === 'ArrowLeft' && !els.back.disabled) history.back();
    if (e.key === 'ArrowRight' && !els.fwd.disabled) history.forward();
    return;
  }
  if (e.ctrlKey || e.metaKey || e.altKey || typing(document.activeElement)) return;
  if (e.key === 'Escape') {
    if (hover.hide()) return;
    if (!els.viewMenu.hidden) { toggleViewMenu(false); els.viewBtn.focus(); return; }
    if (state.sel) selectTarget(null, null);
    return;
  }
  if (!state.current && e.key !== '/' && e.key !== '?') return;
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
      setTab(state.tab === 'asm' ? 'c' : 'asm');
      break;
    case '1': case '2': case '3': case '4':
      setTab(['c', 'asm', 'bytes', 'stack'][Number(e.key) - 1]);
      break;
    case 's':
      toggleSplit();
      break;
    case 'o':
      updatePrefs(cycle(state.prefs, 'asmAddr'));
      applyPaneClasses();
      setHint(`instruction addresses: ${VALUE_LABEL.asmAddr[state.prefs.asmAddr]}`);
      break;
    case 'b':
      updatePrefs({ asmBytes: !state.prefs.asmBytes });
      applyPaneClasses();
      setHint(`bytes column ${state.prefs.asmBytes ? 'on' : 'off'}`);
      break;
    default:
      return;
  }
});

setTab(state.prefs.tab === 'src' ? 'c' : state.prefs.tab);
applyPaneClasses();
syncButtons();
window.addEventListener('beforeunload', () => state.kuna?.close());

// ── project export ─────────────────────────────────────────────────────────

els.dl.addEventListener('click', async () => {
  if (!state.inventory || !state.kuna) return;
  const op = beginOperation('project');
  setStatus('building project…');
  try {
    const nameOf = (addrHex) => session.displayName(state.byAddr.get(addrHex) || { name: addrHex });
    const assertions = state.caps.assert === false ? [] : session.allAssertions(nameOf);
    const project = await state.kuna.project(state.binary.name, { assertions });
    if (!isCurrent(op)) return;
    download(new Blob([project.bytes], { type: 'application/zip' }), project.downloadName);
    setStatus(`${project.downloadName} downloaded (${project.bytes.length.toLocaleString()} bytes)`, 'ok');
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
    setStatus('project export failed: ' + e.message, 'err');
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

// Exposed for the browser smoke test and for debugging from the console.
window.kunaStudy = { state, openFunction, setTab, showFunction, normalizeInspect };
