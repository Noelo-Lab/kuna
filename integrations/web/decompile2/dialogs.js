// dialogs.js — the study view's small forms and notices: a popover anchored
// at the thing being edited (rename, retype, prototype, comment, go to),
// toasts for outcomes, and a confirm box. Every string is escaped here.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { placeOverlay } from './hover.js';

export function createDialogs({ pop, toasts }) {
  let current = null;

  function close(result = null) {
    if (!current) return false;
    const { resolve, restoreFocus } = current;
    current = null;
    pop.hidden = true;
    pop.innerHTML = '';
    resolve(result);
    restoreFocus?.focus?.({ preventScroll: true });
    return true;
  }

  function place(anchorEl) {
    const box = pop.getBoundingClientRect();
    const vp = { width: window.innerWidth, height: window.innerHeight };
    const a = anchorEl?.isConnected ? anchorEl.getBoundingClientRect()
      : { left: (vp.width - box.width) / 2, top: vp.height / 3, right: 0, bottom: vp.height / 3 };
    const pos = placeOverlay(a, { width: box.width, height: box.height }, vp, 12);
    pop.style.left = pos.left + 'px';
    pop.style.top = pos.top + 'px';
  }

  /**
   * Open a form. `fields`: `[{name, label, value, placeholder, list, textarea,
   * validate(value, values)}]`; `warn(values)` returns a non-blocking note.
   * Resolves with the field values, or null when cancelled.
   */
  function openPopover({ anchorEl = null, title, note = '', fields = [], submitLabel = 'Apply', warn = null, danger = false }) {
    close(null);
    return new Promise((resolve) => {
      current = { resolve, restoreFocus: document.activeElement };
      const lists = [];
      const body = fields.map((f, i) => {
        const id = `d2f-${f.name}`;
        let listAttr = '';
        if (f.list?.length) {
          listAttr = ` list="${id}-list"`;
          lists.push(`<datalist id="${id}-list">${f.list.map((v) => `<option value="${escapeHtml(v)}"></option>`).join('')}</datalist>`);
        }
        const common = `id="${id}" name="${escapeHtml(f.name)}" autocomplete="off" autocapitalize="off" spellcheck="false"` +
          `${f.placeholder ? ` placeholder="${escapeHtml(f.placeholder)}"` : ''}${i === 0 ? ' data-first="1"' : ''}`;
        const input = f.textarea
          ? `<textarea ${common} rows="3">${escapeHtml(f.value ?? '')}</textarea>`
          : `<input ${common}${listAttr} value="${escapeHtml(f.value ?? '')}">`;
        return `<label class="d2f"><span>${escapeHtml(f.label)}</span>${input}</label>`;
      }).join('');
      pop.innerHTML = `<form novalidate><h3>${escapeHtml(title)}</h3>` +
        (note ? `<div class="pre">${escapeHtml(note)}</div>` : '') + body + lists.join('') +
        '<p class="err" aria-live="polite"></p>' +
        `<div class="row"><button type="button" class="tinybtn" data-act="cancel">Cancel</button>` +
        `<button type="submit" class="tinybtn primary"${danger ? ' data-danger="1"' : ''}>${escapeHtml(submitLabel)}</button></div></form>`;
      pop.hidden = false;
      pop.setAttribute('role', 'dialog');
      pop.setAttribute('aria-label', title);
      place(anchorEl);
      const form = pop.querySelector('form');
      const errEl = pop.querySelector('.err');
      const values = () => Object.fromEntries(fields.map((f) => [f.name, form.elements[f.name].value]));
      const check = () => {
        const v = values();
        for (const f of fields) {
          form.elements[f.name].classList.remove('bad');
          const err = f.validate?.(v[f.name], v);
          if (err) {
            form.elements[f.name].classList.add('bad');
            errEl.style.color = '';
            errEl.textContent = err;
            return null;
          }
        }
        const note2 = warn?.(v);
        errEl.style.color = note2 ? 'var(--tok-num)' : '';
        errEl.textContent = note2 || '';
        return v;
      };
      form.addEventListener('input', check);
      form.addEventListener('submit', (e) => {
        e.preventDefault();
        const v = check();
        if (v) close(v);
      });
      form.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') { e.preventDefault(); e.stopPropagation(); close(null); }
        if (e.key === 'Enter' && e.target.tagName === 'TEXTAREA' && (e.ctrlKey || e.metaKey)) {
          e.preventDefault();
          form.requestSubmit();
        }
      });
      pop.querySelector('[data-act=cancel]').addEventListener('click', () => close(null));
      const first = pop.querySelector('[data-first]') || pop.querySelector('button[type=submit]');
      first?.focus();
      first?.select?.();
      check();
    });
  }

  document.addEventListener('mousedown', (e) => {
    if (current && !pop.contains(e.target)) close(null);
  });

  /** A toast in the corner: `kind` ok | warn | err, an optional `detail` line. */
  function toast(message, { kind = 'ok', detail = '', ms = 6000, action = null } = {}) {
    const el = document.createElement('div');
    el.className = 'd2-toast ' + kind;
    el.setAttribute('role', kind === 'err' ? 'alert' : 'status');
    el.innerHTML = escapeHtml(message) +
      (action ? ` <button class="d2-lb" data-act="toast">${escapeHtml(action.label)}</button>` : '') +
      (detail ? `<span class="dt">${escapeHtml(detail)}</span>` : '');
    if (action) el.querySelector('[data-act=toast]').addEventListener('click', () => { action.run(); el.remove(); });
    toasts.appendChild(el);
    while (toasts.children.length > 4) toasts.firstChild.remove();
    setTimeout(() => el.remove(), ms);
    return el;
  }

  /** Ask a yes/no question; resolves true on confirm. */
  async function confirmBox(message, { anchorEl = null, confirmLabel = 'Yes' } = {}) {
    const res = await openPopover({ anchorEl, title: 'Confirm', note: message, submitLabel: confirmLabel, danger: true });
    return res !== null;
  }

  return { openPopover, toast, confirmBox, close, isOpen: () => !!current };
}
