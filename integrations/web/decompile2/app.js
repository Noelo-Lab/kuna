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
  els.vname.textContent = session.displayName(fn);
  els.vmeta.textContent = fnMeta(fn, data);
  const row = state.rows.find((r) => r.fn === fn)?.row;
  row?.classList.toggle('bad', !!data.error);
  renderVisible();
  if (focusAddr) revealAddr(focusAddr);
  setHint(null);
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
  },
  asm() {
    els.asmcode.innerHTML = needsInspect('The assembly view');
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

function revealAddr(addrHex) {
  const row = document.getElementById('a-' + addrHex);
  row?.scrollIntoView({ block: 'nearest' });
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
  { key: 'cLineNumbers', label: 'C line numbers' },
  { key: 'cLineAddrs', label: 'C line addresses' },
];

function renderViewMenu() {
  const p = state.prefs;
  els.viewMenu.innerHTML = VIEW_ITEMS.map((item) => {
    const value = p[item.key];
    const shown = typeof value === 'boolean' ? (value ? 'on' : 'off') : String(value);
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
  if (key === 'reset') updatePrefs({ ...DEFAULT_PREFS, tab: state.tab, split: state.prefs.split });
  else if (typeof state.prefs[key] === 'boolean') updatePrefs({ [key]: !state.prefs[key] });
  else updatePrefs(cycle(state.prefs, key));
  applyPaneClasses();
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
    if (!els.viewMenu.hidden) { toggleViewMenu(false); els.viewBtn.focus(); }
    return;
  }
  if (!state.current && e.key !== '/' && e.key !== '?') return;
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
window.kunaStudy = { state, openFunction, setTab };
