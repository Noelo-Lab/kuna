// rail.js — the details rail beside the function: its signature, the
// variables (merged from the signature, the declarations kuna printed and the
// engine's variable list, since register locals appear only in the C), the
// types it uses, lazily fetched references, and the edit session with its
// per-directive outcomes and actions. The rail renders; app.js acts.
import { escapeHtml } from '../assets/js/highlight-c.js';

const MARK = { applied: ['ok', '✓', 'applied'], rejected: ['rej', '✗', 'rejected'], pending: ['pend', '·', 'not yet sent'] };

function section(label, inner, extra = '') {
  return `<section class="d2sec"><p class="label"><span>${escapeHtml(label)}</span>${extra}</p>${inner}</section>`;
}

function varsTable(vars) {
  if (!vars.length) return '<p class="d2muted">none</p>';
  const rows = vars.map((v) => `<tr class="${v.dim ? 'dim' : ''}" data-sym="${escapeHtml(v.name)}">` +
    `<td class="nm" data-act="var-select" title="${v.dim ? 'not in the C: debug info or frame bookkeeping' : 'select · double-click renames'}">${escapeHtml(v.name)}</td>` +
    `<td class="ty">${escapeHtml(v.type || '')}</td><td class="st">${escapeHtml(v.where || v.kind)}</td></tr>`).join('');
  return `<table class="d2tbl" id="railvars">${rows}</table>`;
}

function editsList(model) {
  const items = model.edits.map((e) => {
    const [cls, sym, title] = MARK[e.status] || MARK.pending;
    return `<li data-key="${escapeHtml(e.key)}"><span class="mk ${cls}" title="${title}">${sym}</span>` +
      `<span class="tx">${escapeHtml(e.text)}${e.status === 'rejected' && e.detail ? `<small>${escapeHtml(e.detail)}</small>` : ''}` +
      `${e.fatal ? '<small>the C shown was produced without this directive</small>' : ''}</span>` +
      `<button data-act="edit-edit" title="edit">edit</button><button data-act="edit-remove" title="remove" aria-label="remove">×</button></li>`;
  }).join('');
  const banner = model.restored
    ? `<div class="d2banner">Restored ${model.restored} edit${model.restored === 1 ? '' : 's'} for this binary · <button data-act="discard-restored">discard</button></div>`
    : '';
  const unsupported = model.assertSupported === false
    ? '<p class="d2muted">This engine build cannot apply edits yet; they are kept and exported.</p>' : '';
  const acts = '<div class="d2acts">' +
    `<button class="d2-lb" data-act="undo"${model.canUndo ? '' : ' disabled'} title="Undo (u, Ctrl+Z)">undo</button>` +
    `<button class="d2-lb" data-act="redo"${model.canRedo ? '' : ' disabled'} title="Redo (Ctrl+Shift+Z)">redo</button>` +
    `<button class="d2-lb" data-act="export"${model.edits.length ? '' : ' disabled'} title="Download the session as a .kuna directive file">export</button>` +
    '<button class="d2-lb" data-act="import" title="Load a .kuna directive file">import</button>' +
    `<button class="d2-lb" data-act="copy-cli"${model.edits.length ? '' : ' disabled'} title="Copy the native command that replays this session">copy CLI</button>` +
    `<button class="d2-lb" data-act="clear"${model.edits.length ? '' : ' disabled'}>clear</button></div>`;
  return banner + unsupported +
    (items ? `<ul class="d2edits" id="sesslist">${items}</ul>` : '<ul class="d2edits" id="sesslist"></ul><p class="d2muted">No edits yet. Double-click a name to rename it, <kbd>y</kbd> retypes.</p>') +
    acts;
}

/**
 * Wire the rail. `on(action, data)` receives every click: `edit-proto`,
 * `var-select`/`var-rename` (`{sym}`), `edit-edit`/`edit-remove` (`{key}`),
 * `undo`, `redo`, `export`, `import`, `copy-cli`, `clear`,
 * `discard-restored`, `refs-open`.
 */
export function createRail({ root, on }) {
  root.addEventListener('click', (e) => {
    const el = e.target.closest('[data-act]');
    if (!el || el.disabled) return;
    const act = el.dataset.act;
    if (act === 'var-select') on('var-select', { sym: el.closest('tr').dataset.sym });
    else if (act === 'edit-edit' || act === 'edit-remove') on(act, { key: el.closest('li').dataset.key });
    else on(act, {});
  });
  root.addEventListener('dblclick', (e) => {
    const row = e.target.closest('#railvars tr');
    if (row && !row.classList.contains('dim')) on('var-rename', { sym: row.dataset.sym });
  });
  root.addEventListener('toggle', (e) => {
    if (e.target.id === 'railrefs' && e.target.open) on('refs-open', {});
  }, true);

  return {
    /** Render the whole rail from `model` (see the file header). */
    render(model) {
      if (!model) {
        root.innerHTML = section('session edits', editsList({ edits: [], canUndo: false, canRedo: false }));
        return;
      }
      const sig = model.proto
        ? `<div class="d2sig">${escapeHtml(model.proto)}</div>`
        : '<p class="d2muted">no signature</p>';
      const types = model.types.length
        ? model.types.map((t) => `<details><summary>${escapeHtml(t.name)}${t.size ? ` <span class="d2muted">${t.size} B</span>` : ''}</summary>` +
          `<pre class="d2sig">${escapeHtml(t.definition || '')}</pre></details>`).join('')
        : '<p class="d2muted">only built-in types</p>';
      const refs = `<details id="railrefs"${model.refsOpen ? ' open' : ''}><summary>callers, callees, data</summary>` +
        `<div id="railrefsbody">${model.refsHtml || '<p class="d2muted">open to load</p>'}</div></details>`;
      root.innerHTML =
        section('signature', sig, '<button class="d2-lb" data-act="edit-proto" title="Edit the prototype (y on the name)">edit</button>') +
        section('variables', varsTable(model.vars)) +
        section('types', types) +
        section('references', refs, '<kbd title="x">x</kbd>') +
        section(`session edits (${model.edits.length})`, editsList(model));
    },
    /** Mark the variable rows for `syms` (the current selection). */
    markVars(syms) {
      for (const tr of root.querySelectorAll('#railvars tr')) tr.classList.toggle('hl-sym', syms.has(tr.dataset.sym));
    },
    setRefs(html) {
      const body = root.querySelector('#railrefsbody');
      if (body) body.innerHTML = html;
    },
  };
}
