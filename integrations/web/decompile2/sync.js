// sync.js — keep the study view's panes pointing at the same thing. A target
// (`{line?, addr?, sym?, slot?}`) expands through the shared index into the
// C lines, instruction addresses, symbols and stack slots it touches; every
// registered view marks those, and a selection scrolls the other views to it.
// Hover marks but never scrolls.

/** Expand a target into `{lines, addrs, syms, slots}` sets through `index`. */
export function expand(target, index) {
  const lines = new Set();
  const addrs = new Set();
  const syms = new Set();
  const slots = new Set();
  if (!target || !index) return { lines, addrs, syms, slots };
  if (Number.isInteger(target.line)) {
    lines.add(target.line);
    for (const a of index.lineToInsns.get(target.line) || []) addrs.add(a);
    for (const a of index.lineToInferred?.get(target.line) || []) addrs.add(a);
  }
  if (target.addr) {
    addrs.add(target.addr);
    for (const l of index.insnToLines.get(target.addr) || []) lines.add(l);
    const inferred = index.inferredLine?.get(target.addr);
    if (inferred) lines.add(inferred);
  }
  if (target.sym) {
    syms.add(target.sym);
    for (const l of index.symToLines.get(target.sym) || []) lines.add(l);
    for (const a of index.symToAddrs.get(target.sym) || []) addrs.add(a);
    const slot = index.symToSlot?.get(target.sym);
    if (slot !== undefined) slots.add(slot);
  }
  if (target.slot !== undefined && target.slot !== null) {
    slots.add(target.slot);
    const sym = index.slotToSym?.get(target.slot);
    if (sym) {
      syms.add(sym);
      for (const l of index.symToLines.get(sym) || []) lines.add(l);
    }
  }
  return { lines, addrs, syms, slots };
}

export function createSync() {
  const views = new Map();
  let index = null;
  let selected = null;
  let hovered = null;

  const markAll = (sets, cls, target) => {
    for (const view of views.values()) view.mark(sets, cls, target);
  };

  return {
    setIndex(next) {
      index = next;
      selected = null;
      hovered = null;
    },
    register(name, view) {
      views.set(name, view);
    },
    /** Re-apply the current marks to one view (after it re-renders). */
    refresh(name) {
      const view = views.get(name);
      if (!view) return;
      view.mark(selected ? expand(selected, index) : null, 'hl-sel', selected);
      view.mark(hovered ? expand(hovered, index) : null, 'hl-hover', hovered);
    },
    hover(target) {
      hovered = target;
      markAll(target ? expand(target, index) : null, 'hl-hover', target);
    },
    select(target, { from = null, reveal = true } = {}) {
      selected = target;
      const sets = target ? expand(target, index) : null;
      markAll(sets, 'hl-sel', target);
      if (target && reveal) {
        for (const [name, view] of views) if (name !== from) view.reveal?.(target, sets);
      }
      return sets;
    },
    get selected() {
      return selected;
    },
  };
}
