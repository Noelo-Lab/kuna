// rail.js — the Explain panel beside the code. Three parts: what is selected
// (a card app.js writes), this function in plain words (what it takes and
// returns, what it calls and who calls it, its variables and types), and the
// student's changes with undo, redo and a small menu for the rest. The panel
// renders; app.js acts on the `data-act` it reports.
import { escapeHtml } from '../assets/js/highlight-c.js';

const MARK = {
  applied: ['ok', '✓', 'applied'],
  rejected: ['rej', '✗', 'rejected'],
  refused: ['rej', '✗', 'the engine could not read it; it is not sent until you edit it'],
  pending: ['pend', '•', 'not yet sent'],
};

function varsList(vars, id) {
  return `<ul class="x-list"${id ? ` id="${id}"` : ''}>` + vars.map((v) =>
    `<li data-sym="${escapeHtml(v.name)}" data-act="var-select" title="${escapeHtml(v.title || 'Select · double-click to rename')}">` +
    `<span class="xn">${escapeHtml(v.name)}</span> <span class="xt2">${escapeHtml(v.type || '')}</span>` +
    `${v.where ? ` <span class="xw">${escapeHtml(v.where)}</span>` : ''}</li>`).join('') + '</ul>';
}

function functionPart(fn) {
  const summary = fn.summary.map((s) => `<p class="x-sum">${s}</p>`).join('');
  const vars = fn.vars.length ? varsList(fn.vars, 'railvars') : '<ul class="x-list" id="railvars"></ul><p class="x-empty">No variables.</p>';
  const debug = fn.debugVars.length
    ? `<details><summary>Also in the debug info (${fn.debugVars.length})</summary>${varsList(fn.debugVars)}</details>` : '';
  const types = fn.types.length
    ? `<details class="x-types"><summary>Types (${fn.types.length})</summary>` +
      fn.types.map((t) => `<pre>${escapeHtml(t.definition || t.name)}</pre>`).join('') + '</details>' : '';
  return '<section class="x-sec" id="railfn"><h3>This function</h3>' + summary +
    `<h4>Calls and callers</h4><div id="railrefsbody">${fn.refsHtml || ''}</div>` +
    `<h4>Variables</h4>${vars}${debug}${types}</section>`;
}

function changesPart(model) {
  const items = model.edits.map((e) => {
    const [cls, sym, title] = MARK[e.status] || MARK.pending;
    const why = (e.status === 'rejected' || e.status === 'refused') && e.detail ? `<small>${escapeHtml(e.detail)}</small>` : '';
    const fatal = e.fatal ? '<small>the code shown was made without this change</small>' : '';
    return `<li data-key="${escapeHtml(e.key)}"><span class="mk ${cls}" title="${title}">${sym}</span>` +
      `<span class="tx" title="${escapeHtml(e.text)}">${escapeHtml(e.label)}${why}${fatal}</span>` +
      '<span class="acts"><button data-act="edit-edit">edit</button><button data-act="edit-remove">remove</button></span></li>';
  }).join('');
  const banner = model.restored
    ? `<div class="x-banner d2banner">Restored ${model.restored} change${model.restored === 1 ? '' : 's'} from last time. ` +
      '<button class="d2-link" data-act="discard-restored">Discard them</button></div>' : '';
  const unsupported = model.assertSupported === false
    ? '<p class="x-empty">This version of the decompiler cannot apply changes; they are kept and can be exported.</p>' : '';
  const empty = items ? '' : '<p class="x-empty">Nothing yet. Double-click a name to rename it.</p>';
  const n = model.edits.length;
  return `<section class="x-sec" id="railchanges"><h3>Your changes${n ? ` <span class="x-count">(${n})</span>` : ''}</h3>` +
    banner + unsupported + `<ul class="d2edits" id="sesslist">${items}</ul>` + empty +
    '<div class="x-editacts">' +
    `<button class="d2-btn small" data-act="undo"${model.canUndo ? '' : ' disabled'} title="Undo (U)">Undo</button>` +
    `<button class="d2-btn small" data-act="redo"${model.canRedo ? '' : ' disabled'} title="Redo (Ctrl+Shift+Z)">Redo</button>` +
    '<div class="d2-menuwrap"><button class="d2-iconbtn small" data-act="changes-menu" aria-haspopup="menu" aria-expanded="false" title="More">' +
    '<svg width="14" height="14" viewBox="0 0 16 16" aria-hidden="true"><circle cx="3" cy="8" r="1.5"/><circle cx="8" cy="8" r="1.5"/><circle cx="13" cy="8" r="1.5"/></svg>' +
    '<span class="d2-sr">More</span></button>' +
    '<div class="d2-menu d2-changesmenu" data-menu="changes" role="menu" hidden>' +
    `<button class="d2-mi" role="menuitem" data-act="export"${n ? '' : ' disabled'}>Export changes<small>a .kuna file the command line can replay</small></button>` +
    '<button class="d2-mi" role="menuitem" data-act="import">Import changes</button>' +
    `<button class="d2-mi" role="menuitem" data-act="copy-cli"${n ? '' : ' disabled'}>Copy command line</button>` +
    `<button class="d2-mi" role="menuitem" data-act="clear"${n ? '' : ' disabled'}>Clear all</button>` +
    '</div></div></div></section>';
}

/**
 * Wire the panel. `on(action, data)` receives every click: `var-select` /
 * `var-rename` (`{sym}`), `edit-edit` / `edit-remove` (`{key}`), `undo`,
 * `redo`, `export`, `import`, `copy-cli`, `clear`, `discard-restored`,
 * `refs-load`, and the selected card's `sel-*` actions (`{el}`).
 */
export function createRail({ root, on }) {
  const closeMenus = () => {
    for (const m of root.querySelectorAll('[data-menu]')) m.hidden = true;
    for (const b of root.querySelectorAll('[data-act=changes-menu]')) b.setAttribute('aria-expanded', 'false');
  };
  root.addEventListener('click', (e) => {
    const el = e.target.closest('[data-act]');
    if (!el || el.disabled) return;
    const act = el.dataset.act;
    if (act === 'changes-menu') {
      const menu = el.parentElement.querySelector('[data-menu]');
      const open = menu.hidden;
      closeMenus();
      menu.hidden = !open;
      el.setAttribute('aria-expanded', String(open));
      return;
    }
    closeMenus();
    if (act === 'var-select') on('var-select', { sym: el.closest('[data-sym]').dataset.sym });
    else if (act === 'edit-edit' || act === 'edit-remove') on(act, { key: el.closest('li').dataset.key });
    else on(act, { el });
  });
  root.addEventListener('dblclick', (e) => {
    const row = e.target.closest('.x-list [data-sym]');
    if (row) on('var-rename', { sym: row.dataset.sym });
  });
  document.addEventListener('mousedown', (e) => {
    if (!e.target.closest('.x-editacts')) closeMenus();
  });

  return {
    /** Render the whole panel from `model` (see the file header). */
    render(model) {
      root.innerHTML = `<section class="x-sec" id="railsel"${model.selectedHtml ? '' : ' hidden'}>${model.selectedHtml || ''}</section>` +
        (model.fn ? functionPart(model.fn) : '') + changesPart(model);
    },
    /** Replace just the selected card. */
    setSelected(html) {
      const sel = root.querySelector('#railsel');
      if (!sel) return;
      sel.innerHTML = html || '';
      sel.hidden = !html;
    },
    /** Mark the variable rows for `syms` (the current selection). */
    markVars(syms) {
      for (const li of root.querySelectorAll('.x-list [data-sym]')) li.classList.toggle('hl-sym', syms.has(li.dataset.sym));
    },
    setRefs(html) {
      const body = root.querySelector('#railrefsbody');
      if (body) body.innerHTML = html;
    },
  };
}
