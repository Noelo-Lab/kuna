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
  changedLines,
} from './render-c.js';
import { loadPrefs, savePrefs, cycle, DEFAULT_PREFS } from './prefs.js';
import { renderAsm, renderInsnRows, formatAddr, spacedBytes } from './asm-view.js';
import { createHover } from './hover.js';
import { createSync } from './sync.js';
import { Session, cliCommand } from './session.js';
import {
  validateIdent, validateCType, parseSignature, buildPrototype, typeSize, knownTypes, normalizeType,
} from './ctype.js';
import { hashBytes, SessionStore } from './persist.js';
import { createDialogs } from './dialogs.js';
import { createRail } from './rail.js';
import {
  functionCells, hexRows, renderHex, applyPatches, parseHex, patchedName, fileOffsetFor, originalByteAt,
} from './bytes-view.js';
import { archFrom, nopFill } from './arch.js';
import { explain, idioms } from './mnemonics.js';
import { frameModel, renderFrame, slotIndex } from './stack-frame.js';
import { renderXrefs, localCallees } from './xrefs-view.js';
import { helpHtml } from './help.js';

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
  binary: null,
  inventory: null,
  rows: [],
  stubDiv: null,
  byAddr: new Map(),
  byName: new Map(),
  current: null,
  cache: new Map(),
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

const dialogs = createDialogs({ pop: els.pop, toasts: els.toasts });
const toast = dialogs.toast;

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
    const shown = displayName(entry.fn);
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
    const hash = await hashBytes(bytes);
    if (!isCurrent(op)) return;
    if (hash !== state.binary?.hash) {
      session = restoreSession(hash);
      state.restored = session.size;
    }
    state.binary = { name, bytes, example, format: null, hash };
    setStatus(`indexing ${name} (${bytes.length.toLocaleString()} bytes)…`);
    inventory = await state.kuna.load(bytes, {
      fileName: name,
      mode: els.mode.value,
      language: els.lang.value,
      assertions: listAssertions(),
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
  session.recordOutcomes(inventory.assertions);
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
  renderRail();
  if (state.restored) toast(`Restored ${state.restored} edit${state.restored === 1 ? '' : 's'} for ${name}.`);
  const dt = Math.round(performance.now() - t0);
  setStatus(`${name} (${inventory.format}) — ${inventory.functions.length} functions indexed in ${dt} ms`, 'ok');
  finishOperation(op);
  const fromHash = state.byAddr.get((location.hash || '').slice(1).toLowerCase());
  const first = (keep && state.byAddr.get(keep)) || fromHash || state.byName.get('main') ||
    state.rows.find((r) => !r.stub)?.fn || state.rows[0]?.fn;
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
  if (state.caps.assert === false) return [];
  return session.globalAssertions().filter((d) => !d.startsWith('function '));
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
    if (!active) setStatus(`${displayName(fn)} — ${fn.address_hex} from cache`, cached.error ? 'err' : 'ok');
    return;
  }
  const op = beginOperation('function');
  const label = displayName(fn);
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

function showFunction(fn, data, { focusAddr = null, keep = false } = {}) {
  const { segs } = lineSegments(data);
  const index = buildIndex(data, segs);
  const arch = archFrom(data.target || state.inventory?.target, data.instructions);
  const frame = data.hasInstructions ? frameModel(data, arch) : null;
  if (frame?.supported) Object.assign(index, slotIndex(frame));
  state.current = { fn, data, segs, index, decls: localDecls(data.code), arch, frame };
  state.rendered.clear();
  if (!keep) state.sel = null;
  state.cursor = null;
  hover.hide();
  sync.setIndex(index);
  els.vname.textContent = displayName(fn);
  els.vmeta.textContent = fnMeta(fn, data);
  const row = state.rows.find((r) => r.fn === fn)?.row;
  row?.classList.toggle('bad', !!data.error);
  renderVisible();
  setHint(null);
  renderRail();
  if (state.refsOpen && document.getElementById('railrefs')?.open) loadRefs();
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
  els.asmcode.classList.toggle('no-bytes', !p.asmBytes);
  els.asmcode.classList.toggle('no-ccom', p.asmCMode === 'off');
}

function asmContext(data) {
  return {
    prefs: state.prefs,
    codeLines: data.code.split('\n'),
    fnByAddr: state.byAddr,
    nameOf: displayName,
    patched: patchedInsns(data),
    slotOf: state.current?.frame?.supported ? (reg, disp) => state.current.frame.slotOf(reg, disp) : null,
    hints: idioms(data.instructions, state.current?.arch?.family || 'x86', { nameAt: (hex) => state.byAddr.get(hex)?.name || null }),
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
  let html = `<div class="ch">${escapeHtml(displayName(fn))} — ${escapeHtml(addrHex)}${fn.size ? ` · ${fn.size} B` : ''}${kind}</div>`;
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
  const family = state.current.arch?.family || 'x86';
  const note = explain(insn.mnemonic, family);
  if (note) html += `<div class="cx"><b>${escapeHtml(insn.mnemonic)}</b>: ${escapeHtml(note)}</div>`;
  const idiom = idioms(data.instructions, family, { nameAt: (hex) => state.byAddr.get(hex)?.name || null }).get(addrHex);
  if (idiom) html += `<div class="cx">idiom: ${escapeHtml(idiom)}</div>`;
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
    const hit = state.current.frame?.slots.find((s) => off >= s.offset && off < s.offset + s.size);
    if (hit) html += `<div class="cx"><b>${escapeHtml(hit.name)}</b> ${escapeHtml(hit.type || '')} · ${hit.size} B${hit.offset !== off ? ` (byte ${off - hit.offset} of it)` : ''}</div>`;
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
  rail.markVars(sets ? sets.syms : new Set());
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
    setHint(insn ? `${escapeHtml(insn.address_hex)} <b>${escapeHtml(insn.text)}</b> ${patchButtons(insn)} · <kbd>;</kbd> comment · <kbd>x</kbd> references` : null);
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
  if (target?.kind === 'callee' && Number(tok.closest('.d2-cl').dataset.line) !== 1) {
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
const narrowRail = window.matchMedia('(max-width: 1180px)');
setRail(state.prefs.rail && !narrowRail.matches);
narrowRail.addEventListener('change', () => setRail(state.prefs.rail && !narrowRail.matches));

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

setTab(state.prefs.tab === 'src' ? 'c' : state.prefs.tab);
applyPaneClasses();
syncButtons();
window.addEventListener('beforeunload', () => state.kuna?.close());

// ── editing: dialogs, applyEdit, undo, the rail ────────────────────────────

/** What a C token stands for, as an edit target. */
function tokenTarget(tok) {
  if (!tok || !state.current) return null;
  const self = state.current.data.address_hex;
  const line = Number(tok.closest('.d2-cl')?.dataset.line);
  if (tok.dataset.kind === 'funcname') {
    const callee = tok.dataset.callee || state.byName.get(tok.textContent)?.address_hex || null;
    if (!callee || callee === self || (line === 1 && tok.dataset.decl !== 'param')) return { kind: 'self', addr: self, el: tok };
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

function currentSignature(addr = state.current.data.address_hex) {
  const cached = addr === state.current.data.address_hex ? state.current.data : state.cache.get(addr);
  return parseSignature(session.proto(addr) || cached?.proto || '');
}

function paramIndex(name) {
  const sig = currentSignature();
  const i = sig ? sig.params.findIndex((p) => p.name === name) : -1;
  return i >= 0 ? { sig, index: i } : null;
}

function typeOptions() {
  const data = state.current?.data;
  return knownTypes({ types: data?.types, known: state.inventory?.known_types, typedefs: session.typedefTags() });
}

function targetBits() {
  return state.current?.data.target?.bits || state.inventory?.target?.bits || 64;
}

/** The storage size behind a displayed local, when the engine or the declaration says. */
function currentSize(name) {
  const v = state.current.data.variables.find((x) => x.name === name && x.size);
  if (v) return v.size;
  const decl = state.current.decls.find((d) => d.name === name);
  return decl ? typeSize(decl.type, { bits: targetBits(), llp64: /PE/.test(state.binary?.format || '') }) : null;
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
      anchorEl: target.el, title: `Rename function ${displayName(fn)}`,
      note: `function ${target.addr}=<name>`,
      fields: [{ name: 'name', label: 'new name', value: displayName(fn), validate: validateIdent }],
    });
    if (!res || res.name === displayName(fn)) return;
    return applyEdit(() => session.setFunctionName(target.addr, res.name === fn.name ? null : res.name), { label: 'rename' });
  }
  if (target.kind === 'global') return dataDialog(target);
  const param = paramIndex(target.name);
  const res = await dialogs.openPopover({
    anchorEl: target.el,
    title: `Rename ${param ? 'parameter' : 'local'} ${target.name}`,
    note: param ? 'a parameter is renamed through the function\'s prototype' : varSummary(target.name),
    fields: [{ name: 'name', label: 'new name', value: target.name, validate: validateIdent }],
  });
  if (!res || res.name === target.name) return;
  if (param) {
    param.sig.params[param.index].name = res.name;
    return applyEdit(() => session.setProto(addr, buildPrototype(param.sig)), { label: 'rename', reselect: { sym: res.name } });
  }
  const sym = session.symbolOf(addr, target.name);
  return applyEdit(() => session.setVar(addr, sym, { name: res.name }), { label: 'rename', reselect: { sym: res.name } });
}

async function retypeSelected() {
  const target = selectedTarget();
  if (!target) return state.current && protoDialog(state.current.data.address_hex, els.vname);
  if (target.kind === 'self' || target.kind === 'callee') return protoDialog(target.addr, target.el);
  if (target.kind === 'global') return dataDialog(target);
  const addr = state.current.data.address_hex;
  const param = paramIndex(target.name);
  const current = param ? param.sig.params[param.index].type : describeVar(target.name).type;
  const size = param ? null : currentSize(target.name);
  const res = await dialogs.openPopover({
    anchorEl: target.el,
    title: `Retype ${param ? 'parameter' : 'local'} ${target.name}`,
    note: param ? 'a parameter is retyped through the function\'s prototype' : varSummary(target.name),
    fields: [{ name: 'type', label: 'C type', value: current, list: typeOptions(), validate: validateCType }],
    warn: (v) => {
      const next = typeSize(v.type, { bits: targetBits() });
      return size && next && next !== size
        ? `${target.name} is ${size} bytes; ${normalizeType(v.type)} is ${next}. The engine refuses a retype that changes the storage size.`
        : '';
    },
  });
  if (!res || normalizeType(res.type) === normalizeType(current)) return;
  const type = normalizeType(res.type);
  if (param) {
    param.sig.params[param.index].type = type;
    return applyEdit(() => session.setProto(addr, buildPrototype(param.sig)), { label: 'retype', reselect: { sym: target.name } });
  }
  const sym = session.symbolOf(addr, target.name);
  const rec = session.varRecord(addr, target.name);
  return applyEdit(() => session.setVar(addr, sym, { type, name: rec?.name ?? undefined }), { label: 'retype', reselect: { sym: target.name } });
}

async function protoDialog(addr, anchorEl) {
  const fn = state.byAddr.get(addr) || { address_hex: addr, name: addr };
  const cached = addr === state.current.data.address_hex ? state.current.data : state.cache.get(addr);
  const value = session.proto(addr) || cached?.proto || `void ${displayName(fn)}(void)`;
  const res = await dialogs.openPopover({
    anchorEl, title: `Prototype of ${displayName(fn)}`,
    note: `prototype ${addr} <declaration> — the name inside is ignored; types and parameter names apply`,
    fields: [{
      name: 'decl', label: 'C declaration', value, textarea: true,
      validate: (v) => (parseSignature(v) ? noHash(v) : 'write a declaration like: long sum_to(int count)'),
    }],
  });
  if (!res) return;
  const decl = buildPrototype(parseSignature(res.decl));
  if (decl === value) return;
  return applyEdit(() => session.setProto(addr, decl), { label: 'prototype' });
}

async function dataDialog(target) {
  const g = state.current.data.globals.find((x) => x.address_hex === target.addr || x.name === target.name);
  const rec = session.records.get(`data:${target.addr}`);
  const declType = g?.declaration ? g.declaration.replace(new RegExp(`\\b${target.name}\\b`), '').replace(/;\s*$/, '').trim() : '';
  const res = await dialogs.openPopover({
    anchorEl: target.el, title: `Global at ${target.addr}`,
    note: `data ${target.addr} <type> <name>`,
    fields: [
      { name: 'name', label: 'name', value: rec?.name || target.name, validate: validateIdent },
      { name: 'type', label: 'C type', value: rec?.type || declType || 'undefined4', list: typeOptions(), validate: validateCType },
    ],
  });
  if (!res) return;
  return applyEdit(() => session.setData(target.addr, normalizeType(res.type), res.name), { label: 'global' });
}

async function commentSelected() {
  if (!state.current) return;
  const { index, data } = state.current;
  let addr = state.sel?.addr || null;
  if (!addr && Number.isInteger(state.sel?.line)) addr = index.lineToInsns.get(state.sel.line)?.[0] || null;
  if (!addr && state.cursor?.isConnected) addr = index.lineToInsns.get(Number(state.cursor.closest('.d2-cl').dataset.line))?.[0] || null;
  if (!addr) {
    toast('Select an instruction, or a C line that has one, to comment it.', { kind: 'warn' });
    return;
  }
  const anchor = document.getElementById('a-' + addr) || els.ccode.querySelector(`.d2-cl[data-addrs~="${addr}"] .ct`);
  const res = await dialogs.openPopover({
    anchorEl: anchor, title: `Comment at ${addr}`,
    note: 'rendered into the C at that instruction; leave empty to remove',
    fields: [{ name: 'text', label: 'comment', value: session.comment(data.address_hex, addr) || '', validate: noHash }],
  });
  if (!res) return;
  return applyEdit(() => session.setComment(data.address_hex, addr, res.text.trim() || null), { label: 'comment' });
}

async function goToDialog() {
  if (!state.inventory) return;
  const res = await dialogs.openPopover({
    title: 'Go to', fields: [{
      name: 'where', label: 'function name or address', value: '',
      list: state.rows.map((r) => displayName(r.fn)),
      validate: (v) => (v.trim() ? null : 'a name or an address'),
    }],
  });
  if (!res) return;
  const q = res.where.trim();
  const byShown = state.rows.find((r) => displayName(r.fn) === q)?.fn || state.byName.get(q);
  if (byShown) return openFunction(byShown);
  if (/^(0x)?[0-9a-f]+$/i.test(q)) return gotoAddr('0x' + BigInt('0x' + q.replace(/^0x/i, '')).toString(16));
  toast(`No function named ${q}.`, { kind: 'warn' });
}

/** After any change to the session: persist, drop caches, refresh names and the rail. */
function sessionChanged() {
  persist();
  els.patch.disabled = !canDownloadPatched();
  state.cache.clear();
  state.xrefs?.clear();
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
    const el = document.getElementById('c-L' + n);
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
async function applyEdit(mutate, { label = 'edit', reselect = null } = {}) {
  if (!state.current) return false;
  const addr = state.current.data.address_hex;
  const before = new Set(session.assertionsFor(addr));
  const snap = session.snapshot();
  try {
    mutate();
  } catch (e) {
    toast(e.message, { kind: 'err' });
    return false;
  }
  const fresh = session.assertionsFor(addr).filter((d) => !before.has(d));
  sessionChanged();
  if (state.caps.assert === false) {
    session.pushUndo(snap);
    renderRail();
    toast('Edit kept, not applied: this engine build has no --assert.', { kind: 'warn' });
    return true;
  }
  return reinspect({ snap, fresh, label, reselect });
}

async function reinspect({ snap = null, fresh = [], label = 'edit', reselect = null } = {}) {
  const fn = state.current.fn;
  const oldCode = state.current.data.code;
  const scroll = captureScroll();
  const keepSel = reselect || state.sel;
  const op = beginOperation('edit');
  op.onCancel = () => {
    if (!snap) return;
    session.restore(snap);
    sessionChanged();
    toast(`The ${label} was cancelled and undone.`, { kind: 'warn' });
  };
  setStatus(`applying ${label} to ${displayName(fn)}…`);
  const t0 = performance.now();
  try {
    const doc = await fetchFunction(fn, directivesFor(fn.address_hex));
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
    if (state.caps.assert === false) {
      setStatus(`${displayName(fn)} — ${label} kept; this engine build cannot apply edits`, 'err');
      renderRail();
      return false;
    }
    const mine = data.assertions.filter((r) => fresh.includes(r.directive));
    for (const row of mine.filter((r) => r.status === 'rejected')) {
      toast(`Rejected: ${row.directive}`, { kind: 'err', detail: row.detail || '' });
    }
    const broke = data.assertions.filter((r) => r.status === 'rejected' && !fresh.includes(r.directive) &&
      wasApplied.has(session.sent.get(r.directive)));
    for (const row of broke) {
      toast(`An earlier edit no longer applies: ${row.directive}`, {
        kind: 'warn',
        detail: `${row.detail || ''} — the code changed under it; edit or remove it in the rail.`.replace(/^ — /, ''),
      });
    }
    if (data.assertions.some((r) => r.fatal)) {
      toast('The C shown was produced without one of your directives.', { kind: 'err', detail: data.assertions.find((r) => r.fatal).detail || '' });
    }
    const dt = Math.round(performance.now() - t0);
    const bad = mine.some((r) => r.status === 'rejected');
    setStatus(`${displayName(fn)} — ${label} ${bad ? 'rejected' : 'applied'} in ${dt} ms`, bad ? 'err' : 'ok');
    renderRail();
    return !bad;
  } catch (e) {
    if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return false;
    if (snap) {
      session.restore(snap);
      sessionChanged();
    }
    toast(`The engine could not apply the ${label}.`, { kind: 'err', detail: e.message });
    setStatus(`${label} failed: ${e.message}`, 'err');
    return false;
  } finally {
    finishOperation(op);
  }
}

function undo() {
  if (active?.kind === 'edit' || !session.undo()) return;
  sessionChanged();
  if (state.current && state.caps.assert !== false) reinspect({ label: 'undo' });
}

function redo() {
  if (active?.kind === 'edit' || !session.redo()) return;
  sessionChanged();
  if (state.current && state.caps.assert !== false) reinspect({ label: 'redo' });
}

/** The rail's variables: parameters, printed locals, then what the engine knows but the C does not show. */
function railVars() {
  const { data, decls, index } = state.current;
  const sig = parseSignature(data.proto || '');
  const out = [];
  const shown = new Set();
  const entry = (off) => `stack entry${off < 0 ? '−' : '+'}0x${Math.abs(off).toString(16)}`;
  const home = (name) => data.variables.find((v) => v.name === name && v.kind !== 'arg' && Number.isInteger(v.stack_offset));
  (sig?.params || []).forEach((p, i) => {
    const h = home(p.name);
    out.push({ name: p.name || `(unnamed ${i})`, type: p.type, kind: 'param', where: `arg ${i}${h ? ' · ' + entry(h.stack_offset) : ''}` });
    shown.add(p.name);
  });
  for (const d of decls) {
    out.push({ name: d.name, type: d.type, kind: 'local', where: storageLabel(d.storage) });
    shown.add(d.name);
  }
  for (const v of data.variables) {
    if (shown.has(v.name) || v.kind === 'arg') continue;
    shown.add(v.name);
    out.push({
      name: v.name, type: v.type, kind: v.kind,
      where: Number.isInteger(v.stack_offset) ? entry(v.stack_offset) : v.kind,
      dim: !index.symToLines.has(v.name),
    });
  }
  return out;
}

function renderRail() {
  const edits = session.entries(nameOfAddr);
  const base = {
    edits,
    canUndo: session.canUndo,
    canRedo: session.canRedo,
    restored: state.restored || 0,
    assertSupported: state.caps.assert,
  };
  if (!state.current) {
    rail.render({ ...base, proto: null, vars: [], types: [], refsHtml: '' });
    return;
  }
  const { data } = state.current;
  rail.render({
    ...base,
    proto: data.proto,
    vars: railVars(),
    types: data.types,
    refsOpen: state.refsOpen,
    refsHtml: state.refsHtml?.(data.address_hex) || '',
  });
  if (state.sel?.sym) rail.markVars(new Set([state.sel.sym]));
}

const importEl = document.createElement('input');
importEl.type = 'file';
importEl.accept = '.kuna,.txt,text/plain';
importEl.hidden = true;
document.body.appendChild(importEl);
importEl.addEventListener('change', async () => {
  const file = importEl.files[0];
  importEl.value = '';
  if (!file || !state.current) return;
  const text = await file.text();
  const resolveFunc = (name) => (state.rows.find((r) => displayName(r.fn) === name)?.fn || state.byName.get(name))?.address_hex || null;
  let counts = null;
  const ok = await applyEdit(() => {
    counts = session.importText(text, { resolveFunc, bindTo: state.current.data.address_hex });
  }, { label: 'import' });
  if (counts) {
    toast(`Imported ${counts.added} directive${counts.added === 1 ? '' : 's'} from ${file.name}.`, {
      kind: ok ? 'ok' : 'warn',
      detail: counts.raw ? `${counts.raw} kept verbatim (the page does not model them).` : '',
    });
  }
});

function exportSession() {
  if (!state.binary) return;
  const target = state.current ? displayName(state.current.fn) : 'main';
  const text = session.toFileText({ binary: state.binary.name, hash: state.binary.hash, target, nameOf: nameOfAddr });
  download(new Blob([text], { type: 'text/plain' }), `${state.binary.name}.kuna`);
}

async function editEntry(key) {
  const rec = session.records.get(key);
  if (key.startsWith('bytes:')) {
    setTab('bytes');
    return;
  }
  if (!rec || !state.current) return;
  if (rec.kind === 'var' && rec.func === state.current.data.address_hex) return renameTarget({ kind: 'var', name: rec.name || rec.sym, el: null });
  if (rec.kind === 'fn') return renameTarget({ kind: rec.addr === state.current.data.address_hex ? 'self' : 'callee', addr: rec.addr, el: null });
  if (rec.kind === 'proto') return protoDialog(rec.addr, null);
  if (rec.kind === 'data') return dataDialog({ addr: rec.addr, name: rec.name, el: null });
  const res = await dialogs.openPopover({
    title: 'Edit directive', note: 'one --assert directive',
    fields: [{ name: 'text', label: 'directive', value: session.entries(nameOfAddr).find((x) => x.key === key)?.text || '', validate: (v) => (v.trim() ? noHash(v) : 'empty') }],
  });
  if (!res) return;
  return applyEdit(() => { session.remove(key); session.addRaw(res.text); }, { label: 'edit' });
}

const rail = createRail({
  root: els.railBody,
  on: async (act, data) => {
    switch (act) {
      case 'edit-proto': if (state.current) protoDialog(state.current.data.address_hex, els.railBody.querySelector('[data-act=edit-proto]')); break;
      case 'var-select': {
        const tok = els.ccode.querySelector(`.t[data-sym="${CSS.escape(data.sym)}"]`);
        selectTarget({ sym: data.sym }, null, tok);
        break;
      }
      case 'var-rename': renameTarget({ kind: 'var', name: data.sym, el: els.railBody.querySelector(`tr[data-sym="${CSS.escape(data.sym)}"]`) }); break;
      case 'edit-remove': applyEdit(() => session.remove(data.key), { label: 'removal' }); break;
      case 'edit-edit': editEntry(data.key); break;
      case 'undo': undo(); break;
      case 'redo': redo(); break;
      case 'export': exportSession(); break;
      case 'import': importEl.click(); break;
      case 'copy-cli': {
        const cmd = cliCommand(state.binary.name, state.current ? displayName(state.current.fn) : 'main');
        try {
          await navigator.clipboard.writeText(cmd);
          toast('Copied the replay command.', { detail: cmd });
        } catch (_) {
          toast('Copy this command:', { kind: 'warn', detail: cmd, ms: 12000 });
        }
        break;
      }
      case 'clear':
        if (await dialogs.confirmBox(`Remove all ${session.size} edits for ${state.binary.name}?`, { confirmLabel: 'Clear' })) {
          applyEdit(() => session.clear(), { label: 'clear' });
        }
        break;
      case 'discard-restored':
        state.restored = 0;
        applyEdit(() => session.clear(), { label: 'discard' });
        break;
      case 'refs-open': state.onRefsOpen?.(); break;
      default: break;
    }
  },
});
renderRail();

els.fnRename.addEventListener('click', () => state.current && renameTarget({ kind: 'self', addr: state.current.data.address_hex, el: els.fnRename }));
els.proto.addEventListener('click', () => state.current && protoDialog(state.current.data.address_hex, els.proto));

// ── bytes: the hex dump, typed patches, Patch/NOP/Revert, the patched file ─

const bytesState = { edit: null, digits: '', burst: null, timer: 0 };

function sections() {
  return state.inventory?.sections || [];
}

function currentArch() {
  const data = state.current?.data;
  return archFrom(data?.target || state.inventory?.target, data?.instructions || []);
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

function bytesCells() {
  if (state.dataView) return dataCells();
  const { data } = state.current;
  return functionCells(data.instructions, {
    entry: data.address_hex, size: data.size, fileBytes: state.binary?.bytes, sections: sections(), patches: session.bytes,
  });
}

function renderBytes() {
  if (state.dataView) return renderData();
  const { data, index } = state.current;
  const cells = bytesCells();
  const selected = new Set();
  if (state.sel?.addr) selected.add(index.insnIndex.get(state.sel.addr));
  const off = fileOffsetFor(data.address_hex, sections());
  const patched = cells.filter((c) => c.patched).length;
  const where = off === null
    ? (sections().length ? 'not backed by the file (patches cannot be downloaded)' : 'file offsets arrive with the engine\'s section table')
    : `file offset 0x${off.toString(16)}`;
  els.bytesbar.innerHTML = `<span><b>${escapeHtml(displayName(state.current.fn))}</b> ${escapeHtml(data.address_hex)} · ${cells.length} bytes · ${escapeHtml(where)}</span>` +
    `<span>${patched ? `${patched} patched` : 'click a byte and type hex to patch it'}</span>` +
    (state.sel?.addr ? `<span>${patchButtons(index.addrToInsn.get(state.sel.addr))}</span>` : '');
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
  const cells = dataCells();
  els.bytesbar.innerHTML = `<span><b>data</b> at ${escapeHtml('0x' + focus.toString(16))} · ${cells.length} bytes from ${escapeHtml(source)}</span>` +
    `<button class="d2-lb" data-act="data-back">back to ${escapeHtml(state.current ? displayName(state.current.fn) : 'the function')}</button>`;
  els.hexdump.innerHTML = renderHex(hexRows(cells), { sections: sections(), editAddr: bytesState.edit ?? focus });
}

/** Show the bytes at a non-code address: from the file when it backs them, else `read`. */
async function showDataAt(hex) {
  const focus = BigInt(hex);
  const start = focus - (focus % 16n);
  const off = fileOffsetFor(start, sections());
  if (off !== null && state.binary) {
    state.dataView = { start, focus, bytes: state.binary.bytes.subarray(off, off + DATA_SPAN), source: 'the file' };
  } else {
    const op = beginOperation('read');
    try {
      const res = await state.kuna.read('0x' + start.toString(16), DATA_SPAN, { assertions: state.caps.assert === false ? [] : session.globalAssertions() });
      if (!isCurrent(op)) return;
      state.dataView = { start, focus, bytes: parseHex(res.bytes) || new Uint8Array(), source: 'the engine' };
    } catch (e) {
      if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
      const old = isUnknownCommand(e) || /unexpected argument/.test(e.message || '');
      toast(old ? `${hex} is not in a function, and this engine build cannot read raw bytes.` : `Could not read ${hex}.`, { kind: 'warn', detail: old ? '' : e.message });
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
  const nop = nopFill(currentArch(), insn.size);
  const base = BigInt(insn.address_hex);
  let any = false;
  for (let k = 0; k < insn.size; k++) if (session.bytes.has(base + BigInt(k))) any = true;
  return `<button class="d2-lb" data-patch="patch" data-addr="${insn.address_hex}">patch</button> ` +
    `<button class="d2-lb" data-patch="nop" data-addr="${insn.address_hex}"${nop ? '' : ' disabled title="no exact no-op fill for this size"'}>nop</button> ` +
    `<button class="d2-lb" data-patch="revert" data-addr="${insn.address_hex}"${any ? '' : ' disabled'}>revert</button>`;
}

/** Write `values` at `addr` as one edit (a byte equal to the file's is no patch). */
function patchBytes(addr, values, label) {
  const base = BigInt(addr);
  return applyEdit(() => {
    values.forEach((v, i) => session.setByte(base + BigInt(i), v, origByte(base + BigInt(i))));
  }, { label, reselect: state.sel });
}

async function patchAction(kind, addrHex) {
  const insn = state.current?.index.addrToInsn.get(addrHex);
  if (!insn) return;
  if (kind === 'nop') {
    const fill = nopFill(currentArch(), insn.size);
    if (fill) patchBytes(addrHex, [...parseHex(fill)], 'NOP');
    return;
  }
  if (kind === 'revert') {
    const base = BigInt(addrHex);
    applyEdit(() => {
      for (let k = 0; k < insn.size; k++) session.bytes.delete(base + BigInt(k));
    }, { label: 'revert', reselect: state.sel });
    return;
  }
  const current = [...Array(insn.size).keys()].map((k) => {
    const a = BigInt(addrHex) + BigInt(k);
    return (session.bytes.get(a) ?? parseInt(insn.bytes.slice(k * 2, k * 2 + 2), 16)).toString(16).padStart(2, '0');
  }).join(' ');
  const res = await dialogs.openPopover({
    anchorEl: document.getElementById('a-' + addrHex),
    title: `Patch ${insn.address_hex} (${insn.text})`,
    note: `${insn.size} byte${insn.size === 1 ? '' : 's'}: ${current}`,
    fields: [{ name: 'hex', label: 'new bytes (hex)', value: current, validate: (v) => (parseHex(v) ? null : 'hex digits in pairs, like 90 90') }],
    warn: (v) => {
      const n = parseHex(v.hex)?.length || 0;
      if (!n || n === insn.size) return '';
      return n < insn.size
        ? `${n} of ${insn.size} bytes: the rest of this instruction stays, and the CPU decodes what is left as something else.`
        : `${n} bytes run past this ${insn.size}-byte instruction into the next one.`;
    },
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
  renderBytes();
  renderRail();
  els.patch.disabled = !canDownloadPatched();
  clearTimeout(bytesState.timer);
  bytesState.timer = setTimeout(flushBytes, 700);
}

function flushBytes({ send = true } = {}) {
  clearTimeout(bytesState.timer);
  const burst = bytesState.burst;
  bytesState.burst = null;
  if (!burst || !state.current) return;
  const fresh = session.globalAssertions().filter((d) => !burst.before.has(d));
  sessionChanged();
  if (!send || state.caps.assert === false) {
    session.pushUndo(burst.snap);
    renderRail();
    return;
  }
  reinspect({ snap: burst.snap, fresh, label: 'patch', reselect: state.sel });
}

function canDownloadPatched() {
  return !!(state.binary && session.byteRuns().length && sections().length);
}

els.patch.addEventListener('click', () => {
  if (!canDownloadPatched()) return;
  const { bytes, unmapped } = applyPatches(state.binary.bytes, sections(), session.byteRuns());
  if (unmapped.length) {
    toast('Some patched bytes are not in the file, so they cannot be written.', {
      kind: 'err', detail: unmapped.map((u) => `${u.count} byte${u.count === 1 ? '' : 's'} at ${u.addr}`).join(', '),
    });
    return;
  }
  const name = patchedName(state.binary.name);
  download(new Blob([bytes], { type: 'application/octet-stream' }), name);
  const fmt = state.binary.format || '';
  let detail = `${session.byteRuns().reduce((n, r) => n + r.values.length, 0)} bytes changed; nothing else in the file moved.`;
  if (/Mach-O/.test(fmt)) detail += ` macOS refuses a modified signed binary: re-sign it with  codesign -f -s - ${name}`;
  else if (/PE/.test(fmt)) detail += ' The PE checksum no longer matches; Windows checks it only for drivers.';
  else if (/ELF/.test(fmt)) detail += ` Run it with  chmod +x ${name} && ./${name}`;
  toast(`Downloaded ${name}.`, { detail, ms: 10000 });
});

// ── references and help ────────────────────────────────────────────────────

state.xrefs = new Map();

function refsHtmlFor(addrHex) {
  return state.xrefs.get(addrHex) || '';
}
state.refsHtml = refsHtmlFor;

/** Load (once per session state) and show the open function's references. */
function loadRefs() {
  if (!state.current) return;
  const addr = state.current.data.address_hex;
  if (state.xrefs.has(addr)) {
    rail.setRefs(state.xrefs.get(addr));
    return;
  }
  rail.setRefs('<p class="d2muted">loading…</p>');
  whenIdle(async () => {
    if (state.current?.data.address_hex !== addr) return;
    const nameOf = (a, n) => (state.byAddr.has(a) ? displayName(state.byAddr.get(a)) : n || a);
    const op = beginOperation('xrefs');
    let html;
    try {
      const res = await state.kuna.xrefs(addr, { assertions: state.caps.assert === false ? [] : session.globalAssertions() });
      if (!isCurrent(op)) return;
      html = renderXrefs(res, { nameOf });
    } catch (e) {
      if (!isCurrent(op) || e instanceof KunaWorkerCancelledError) return;
      const callees = localCallees(state.current.data, state.byAddr);
      html = (isUnknownCommand(e)
        ? '<p class="d2muted">Callers need the engine\'s <code>xrefs</code> command, which this build does not have. Callees below are read from this function\'s CALL instructions.</p>'
        : `<p class="d2muted">${escapeHtml(e.message)}</p>`) +
        renderXrefs({ callers: [], callees, data_refs: [] }, { nameOf });
    } finally {
      finishOperation(op);
    }
    state.xrefs.set(addr, html);
    if (state.current?.data.address_hex === addr) rail.setRefs(html);
  });
}

state.onRefsOpen = () => {
  state.refsOpen = true;
  loadRefs();
};

function openRefs() {
  if (!state.current) return;
  if (els.work.classList.contains('norail')) setRail(true);
  state.refsOpen = true;
  const det = document.getElementById('railrefs');
  if (det && !det.open) det.open = true;
  else loadRefs();
  det?.scrollIntoView({ block: 'nearest' });
}

els.railBody.addEventListener('click', (e) => {
  const a = e.target.closest('a.xt[data-goto]');
  if (!a) return;
  e.preventDefault();
  gotoAddr(a.dataset.goto);
});

function openHelp() {
  if (els.helpDialog.open) {
    els.helpDialog.close();
    return;
  }
  hover.hide();
  els.helpDialog.innerHTML = helpHtml();
  els.helpDialog.showModal();
}
els.help.addEventListener('click', openHelp);
els.helpDialog.addEventListener('click', (e) => {
  if (e.target === els.helpDialog || e.target.closest('[data-act=help-close]')) els.helpDialog.close();
});

// ── project export ─────────────────────────────────────────────────────────

els.dl.addEventListener('click', async () => {
  if (!state.inventory || !state.kuna) return;
  const op = beginOperation('project');
  setStatus('building project…');
  try {
    const assertions = state.caps.assert === false ? [] : session.allAssertions(nameOfAddr);
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

