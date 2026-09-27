// presence.js — the other people's pointers and pings, drawn over the panes.
// A pointer travels as an anchor the page already renders (a C line, an
// instruction, a line's heading in the assembly, a byte, a stack slot) and a
// place on it — for code, the character column, so it lands on the same name
// whatever the window's width — and each page draws it on that thing in its
// own window, or hides it when that thing is not on screen. One overlay per
// pane, moved with `transform`; nothing here takes pointer events.
const ARROW = '<svg class="d2-ptr-arrow" width="16" height="22" viewBox="0 0 16 22" aria-hidden="true">' +
  '<path d="M1 1v17.5l4.6-4.4 3.3 7.2 3-1.4-3.2-7h6.4z"/></svg>';

const widths = new Map();
let canvas = null;

/** The width of one character in `el`'s monospace font. */
function charWidth(el) {
  const font = getComputedStyle(el).font;
  if (!widths.has(font)) {
    canvas ||= document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    ctx.font = font;
    widths.set(font, ctx.measureText('0000000000').width / 10 || 8);
  }
  return widths.get(font);
}

/** The anchor under `target`, with the place on it for `(x, y)`: `{anchor, el, fx, fy, col?}` or null. */
export function anchorAt(target, x, y) {
  if (!target?.closest) return null;
  let el;
  let anchor;
  let text = null;
  if ((el = target.closest('#ccode .d2-cl[data-line]'))) {
    anchor = `c:${el.dataset.line}`;
    text = el.querySelector('.ct');
  } else if ((el = target.closest('#asmcode .d2-ar[data-addr]'))) {
    anchor = `a:${el.dataset.addr}`;
    text = el.querySelector('.am');
  } else if ((el = target.closest('#asmcode .d2-as[data-line]'))) {
    anchor = `h:${el.dataset.line}`;
    text = el.querySelector('.ast');
  } else if ((el = target.closest('#hexdump .hb[data-a]'))) {
    anchor = `b:${el.dataset.a}`;
  } else if ((el = target.closest('#stackframe tr[data-slot]'))) {
    anchor = `s:${el.dataset.slot}`;
    el = el.querySelector('td.slot') || el;
  } else {
    return null;
  }
  const r = el.getBoundingClientRect();
  if (!r.width || !r.height) return null;
  const clamp = (v) => Math.min(1, Math.max(0, v));
  const out = { anchor, el, fx: clamp((x - r.left) / r.width), fy: clamp((y - r.top) / r.height) };
  if (text) {
    const col = (x - text.getBoundingClientRect().left) / charWidth(text);
    out.col = Math.round(Math.min(4096, Math.max(-64, col)) * 100) / 100;
  }
  return out;
}

/** The element an anchor names in this page, or null. */
export function findAnchor(anchor) {
  const m = /^([chabs]):(.+)$/.exec(anchor || '');
  if (!m) return null;
  const [, kind, v] = m;
  if (kind === 'c') return document.getElementById(`c-L${v}`);
  if (kind === 'a') return document.getElementById(`a-${v}`);
  if (kind === 'h') return document.querySelector(`#asmcode .d2-as[data-line="${CSS.escape(v)}"]`);
  if (kind === 'b') return document.querySelector(`#hexdump .hb[data-a="${CSS.escape(v)}"]`);
  const tr = document.querySelector(`#stackframe tr[data-slot="${CSS.escape(v)}"]`);
  return tr?.querySelector('td.slot') || tr;
}

const TEXT_OF = { c: '.ct', a: '.am', h: '.ast' };

/** Where on screen a pointer lands, or null when its anchor is not visible here. */
function placeOf(cursor) {
  const el = findAnchor(cursor.anchor);
  if (!el) return null;
  const pane = el.closest('.d2pane');
  const scroller = el.closest('.d2code');
  if (!pane || !scroller || pane.hidden || !pane.getClientRects().length) return null;
  const r = el.getBoundingClientRect();
  if (!r.width || !r.height) return null;
  let x = r.left + cursor.fx * r.width;
  const text = TEXT_OF[cursor.anchor[0]] && Number.isFinite(cursor.col) ? el.querySelector(TEXT_OF[cursor.anchor[0]]) : null;
  if (text) x = text.getBoundingClientRect().left + cursor.col * charWidth(text);
  const y = r.top + cursor.fy * r.height;
  const view = scroller.getBoundingClientRect();
  if (x < view.left || x > view.right || y < view.top || y > view.bottom) return null;
  return { pane, x, y, rect: r };
}

/**
 * The overlay. `show(peer, cursor, who)` places a person's pointer
 * (`who`: {name, color}); `pulse(anchor, color)` rings a thing for a moment.
 */
export function createPresence({ panes }) {
  const cursors = new Map();
  const nodes = new Map();
  let frame = 0;
  let late = 0;

  const overlay = (pane) => {
    let layer = pane.querySelector(':scope > .d2-presence');
    if (!layer) {
      layer = document.createElement('div');
      layer.className = 'd2-presence';
      pane.appendChild(layer);
    }
    return layer;
  };

  function draw() {
    cancelAnimationFrame(frame);
    clearTimeout(late);
    frame = 0;
    late = 0;
    for (const [peer, c] of cursors) {
      let node = nodes.get(peer);
      const place = c.off || !c.visible ? null : placeOf(c);
      if (!place) {
        if (node) node.hidden = true;
        continue;
      }
      const layer = overlay(place.pane);
      if (!node) {
        node = document.createElement('div');
        node.className = 'd2-ptr';
        node.dataset.peer = peer;
        node.innerHTML = `${ARROW}<span class="d2-ptr-tag"></span>`;
        nodes.set(peer, node);
      }
      if (node.parentElement !== layer) layer.appendChild(node);
      node.style.setProperty('--who', c.color);
      const tag = node.querySelector('.d2-ptr-tag');
      if (tag.textContent !== c.name) tag.textContent = c.name;
      const box = place.pane.getBoundingClientRect();
      node.style.transform = `translate(${Math.round(place.x - box.left)}px, ${Math.round(place.y - box.top)}px)`;
      node.dataset.anchor = c.anchor;
      node.hidden = false;
    }
  }

  const schedule = () => {
    if (frame || late) return;
    if (!document.hidden) frame = requestAnimationFrame(draw);
    late = setTimeout(draw, 100);
  };

  panes.addEventListener('scroll', schedule, { capture: true, passive: true });
  window.addEventListener('resize', schedule);
  const watch = new MutationObserver(schedule);
  for (const code of panes.querySelectorAll('.d2code')) watch.observe(code, { childList: true });
  for (const pane of panes.querySelectorAll('.d2pane')) watch.observe(pane, { attributeFilter: ['hidden'] });

  return {
    /** Place (or move) a person's pointer; `visible` false hides it (another function). */
    show(peer, cursor, who, visible = true) {
      cursors.set(peer, { ...cursor, name: who.name, color: who.color, visible });
      schedule();
    },
    hide(peer) {
      const c = cursors.get(peer);
      if (c) c.off = true;
      schedule();
    },
    forget(peer) {
      cursors.delete(peer);
      nodes.get(peer)?.remove();
      nodes.delete(peer);
    },
    redraw: schedule,
    /** Is this anchor on screen now? */
    visible(anchor) {
      return !!placeOf({ anchor, fx: 0.5, fy: 0.5 });
    },
    /** Ring the thing an anchor names, in `color`, if it is on screen. */
    pulse(anchor, color) {
      const place = placeOf({ anchor, fx: 0.5, fy: 0.5 });
      if (!place) return false;
      const box = place.pane.getBoundingClientRect();
      const ring = document.createElement('div');
      ring.className = 'd2-ping';
      ring.style.setProperty('--who', color);
      ring.style.transform = `translate(${Math.round(place.rect.left - box.left)}px, ${Math.round(place.rect.top - box.top)}px)`;
      ring.style.width = `${Math.round(place.rect.width)}px`;
      ring.style.height = `${Math.round(place.rect.height)}px`;
      ring.dataset.anchor = anchor;
      overlay(place.pane).appendChild(ring);
      setTimeout(() => ring.remove(), 2600);
      return true;
    },
    clear() {
      for (const peer of [...cursors.keys()]) this.forget(peer);
      for (const ring of panes.querySelectorAll('.d2-ping')) ring.remove();
    },
  };
}
