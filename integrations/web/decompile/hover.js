// hover.js — the study view's hover card: a delayed, non-interactive overlay
// that explains whatever the pointer rests on (a C line, a call, an
// instruction, a stack slot). `placeOverlay` is pure; `createHover` owns the
// pointer/keyboard plumbing and asks `resolve(el)` what to show.

/**
 * Where to put a `size` box next to `anchor` inside `viewport`: below the
 * anchor when it fits, else above, else wherever there is more room; always
 * clamped `margin` px inside the viewport.
 */
export function placeOverlay(anchor, size, viewport, margin = 8) {
  const gap = 4;
  const maxLeft = Math.max(margin, viewport.width - size.width - margin);
  const left = Math.min(Math.max(anchor.left, margin), maxLeft);
  const below = anchor.bottom + gap;
  const above = anchor.top - gap - size.height;
  const fitsBelow = below + size.height + margin <= viewport.height;
  const fitsAbove = above >= margin;
  let placement = 'below';
  let top = below;
  if (!fitsBelow && fitsAbove) {
    placement = 'above';
    top = above;
  } else if (!fitsBelow) {
    const roomBelow = viewport.height - anchor.bottom;
    placement = roomBelow >= anchor.top ? 'below' : 'above';
    top = placement === 'below' ? below : above;
    top = Math.min(Math.max(top, margin), Math.max(margin, viewport.height - size.height - margin));
  }
  return { left: Math.round(left), top: Math.round(top), placement };
}

/**
 * Wire a hover card. `resolve(el)` → `{key, html, anchor?}` or null for the
 * element under the pointer; `delay()` → ms (negative disables). The card
 * shows after the delay, switches instantly while warm, and hides on
 * pointer-out (120 ms grace), scroll, wheel, blur, mousedown and Escape.
 * A touch long-press (500 ms) shows it too.
 */
export function createHover({ roots, card, delay, resolve }) {
  let timer = 0;
  let graceTimer = 0;
  let shownKey = null;
  let pendingKey = null;

  const clear = () => {
    clearTimeout(timer);
    clearTimeout(graceTimer);
    timer = 0;
    graceTimer = 0;
  };

  function hide() {
    clear();
    pendingKey = null;
    if (card.hidden) return false;
    card.hidden = true;
    shownKey = null;
    return true;
  }

  function show(hit, fallbackEl) {
    card.innerHTML = hit.html;
    card.style.visibility = 'hidden';
    card.hidden = false;
    const box = card.getBoundingClientRect();
    const anchorEl = hit.anchor || fallbackEl;
    const a = anchorEl.getBoundingClientRect();
    const pos = placeOverlay(
      { left: a.left, top: a.top, right: a.right, bottom: a.bottom },
      { width: box.width, height: box.height },
      { width: window.innerWidth, height: window.innerHeight },
    );
    card.style.left = pos.left + 'px';
    card.style.top = pos.top + 'px';
    card.dataset.placement = pos.placement;
    card.style.visibility = '';
    shownKey = hit.key;
  }

  function consider(el) {
    const hit = el ? resolve(el) : null;
    if (!hit) {
      pendingKey = null;
      clearTimeout(timer);
      if (!card.hidden) {
        clearTimeout(graceTimer);
        graceTimer = setTimeout(hide, 120);
      }
      return;
    }
    clearTimeout(graceTimer);
    if (hit.key === shownKey && !card.hidden) return;
    if (hit.key === pendingKey) return;
    const ms = delay();
    if (ms < 0) return;
    pendingKey = hit.key;
    clearTimeout(timer);
    if (!card.hidden) {
      show(hit, el);
      pendingKey = null;
      return;
    }
    timer = setTimeout(() => {
      pendingKey = null;
      const again = resolve(el);
      if (again && el.isConnected) show(again, el);
    }, ms);
  }

  let touchTimer = 0;
  for (const root of roots) {
    root.addEventListener('pointerover', (e) => {
      if (e.pointerType === 'touch') return;
      consider(e.target);
    });
    root.addEventListener('pointerout', (e) => {
      if (e.pointerType === 'touch') return;
      if (!e.relatedTarget || !root.contains(e.relatedTarget)) consider(null);
    });
    root.addEventListener('pointerdown', (e) => {
      if (e.pointerType !== 'touch') return;
      clearTimeout(touchTimer);
      const el = e.target;
      touchTimer = setTimeout(() => {
        const hit = resolve(el);
        if (hit) show(hit, el);
      }, 500);
    });
    for (const type of ['pointerup', 'pointercancel', 'pointermove']) {
      root.addEventListener(type, (e) => {
        if (e.pointerType === 'touch' && type !== 'pointermove') clearTimeout(touchTimer);
      });
    }
    root.addEventListener('scroll', hide, { passive: true });
  }
  window.addEventListener('wheel', hide, { passive: true, capture: true });
  window.addEventListener('scroll', hide, { passive: true, capture: true });
  window.addEventListener('blur', hide);
  document.addEventListener('mousedown', hide, { capture: true });

  return {
    hide,
    isOpen: () => !card.hidden,
    /** Keyboard: show the card for `el` now (active-descendant navigation). */
    showFor(el) {
      clear();
      const hit = el ? resolve(el) : null;
      if (hit) show(hit, el);
      else hide();
    },
  };
}
