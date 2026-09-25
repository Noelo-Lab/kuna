// stack-frame.js — a diagram of one function's stack frame, drawn from the
// prologue and the engine's variable list. Every offset is measured from the
// stack pointer at the function's entry, where it points at the return
// address: after `PUSH RBP; MOV RBP,RSP`, `[RBP - 0xc]` is entry − 0x14
// (entry = RBP displacement − 8). x86 only; other targets get a note. DOM-free.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { stackOperand } from './asm-view.js';

const hexOff = (n) => `${n < 0 ? '−' : '+'}0x${Math.abs(n).toString(16)}`;

/**
 * The prologue's effect: pushes (in order), where RBP points (entry offset),
 * the `SUB RSP,N` size, and where RSP sits once the prologue is done.
 */
export function prologue(instructions, bits = 64) {
  const word = bits / 8;
  let sp = 0;
  let fp = null;
  let reserve = 0;
  const pushes = [];
  for (const insn of instructions.slice(0, 16)) {
    const m = insn.mnemonic.toUpperCase();
    const ops = insn.operands.replace(/\s+/g, '');
    if (m === 'ENDBR64' || m === 'ENDBR32' || m === 'NOP') continue;
    if (m === 'PUSH' && /^[RE]?[A-Z0-9]+$/i.test(ops)) {
      sp -= word;
      pushes.push({ reg: ops.toUpperCase(), offset: sp });
      continue;
    }
    if (m === 'MOV' && /^[RE]BP,[RE]SP$/i.test(ops)) { fp = sp; continue; }
    if (m === 'SUB' && /^[RE]SP,0x[0-9a-f]+$/i.test(ops)) {
      reserve = Number(BigInt(ops.split(',')[1]));
      sp -= reserve;
      continue;
    }
    if (m === 'MOV' && /^(dword|qword|word|byte)ptr\[[RE]BP/i.test(ops)) continue;
    break;
  }
  return { pushes, fp, reserve, sp, word };
}

/**
 * The frame, high addresses first: `{family, supported, slots, size, redZone,
 * slotOf(reg, disp), note}`. A slot is `{offset, size, kind, name, type,
 * dim, array, overlap}` with kind ret | fp | saved | arg | var | pad.
 */
export function frameModel(fnData, arch = { family: 'x86', bits: 64 }) {
  if (arch.family !== 'x86') {
    return { family: arch.family, supported: false, slots: [], note: 'The frame diagram models x86 frames; this function is not x86.', slotOf: () => null };
  }
  const bits = arch.bits || 64;
  const word = bits / 8;
  const pro = prologue(fnData.instructions || [], bits);
  const slots = [{ offset: 0, size: word, kind: 'ret', name: 'return address', type: `pushed by the caller's CALL` }];
  for (const p of pro.pushes) {
    const isFp = /^[RE]BP$/.test(p.reg);
    slots.push({ offset: p.offset, size: word, kind: isFp ? 'fp' : 'saved', name: isFp ? `saved ${p.reg}` : `saved ${p.reg}`, type: 'the caller\'s value, restored before returning' });
  }
  const inCode = (name) => new RegExp(`\\b${name.replace(/[^\w]/g, '')}\\b`).test(fnData.code || '');
  const seen = new Set();
  for (const v of fnData.variables || []) {
    if (!Number.isInteger(v.stack_offset) || !v.size) continue;
    const key = `${v.stack_offset}:${v.size}:${v.name}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const off = v.stack_offset;
    const clash = slots.find((s) => (s.kind === 'fp' || s.kind === 'saved' || s.kind === 'ret') && s.offset === off && s.size === v.size);
    if (clash) {
      clash.alias = v.name;
      continue;
    }
    slots.push({
      offset: off, size: v.size, kind: off >= word ? 'arg' : 'var', name: v.name, type: v.type,
      dim: !inCode(v.name), array: /\[\d*\]/.test(v.type || ''),
    });
  }
  slots.sort((a, b) => b.offset - a.offset || b.size - a.size);
  const out = [];
  for (let i = 0; i < slots.length; i++) {
    const s = slots[i];
    const prev = out[out.length - 1];
    if (prev && prev.offset >= s.offset + s.size + 0 && prev.offset > s.offset + s.size && prev.kind !== 'pad') {
      const gapHi = prev.offset;
      const gapLo = s.offset + s.size;
      if (gapHi > gapLo && gapLo < 0) out.push({ offset: gapLo, size: gapHi - gapLo, kind: 'pad', name: 'unused', type: 'padding or a value the decompiler never named' });
    }
    if (prev && s.offset + s.size > prev.offset && prev.kind !== 'pad') {
      s.overlap = prev.name;
    }
    out.push(s);
  }
  const lowest = Math.min(0, ...out.map((s) => s.offset));
  const frameLow = pro.reserve ? pro.sp : lowest;
  if (out.length && frameLow < out[out.length - 1].offset && pro.reserve) {
    const last = out[out.length - 1];
    out.push({ offset: frameLow, size: last.offset - frameLow, kind: 'pad', name: 'unused', type: 'reserved by SUB RSP but not used as a named variable' });
  }
  const leaf = !(fnData.instructions || []).some((i) => /^CALL/i.test(i.mnemonic));
  const slotOf = (reg, disp) => {
    if (/^[RE]BP$/.test(reg) && pro.fp !== null) return pro.fp + disp;
    if (/^[RE]SP$/.test(reg)) return pro.sp + disp;
    return null;
  };
  return {
    family: 'x86',
    supported: true,
    word,
    slots: out,
    fp: pro.fp,
    reserve: pro.reserve,
    size: -Math.min(frameLow, lowest),
    redZone: leaf && !pro.reserve && lowest < pro.sp,
    slotOf,
  };
}

/** The entry offset a stack operand (`dword ptr [RBP + -0xc]`) addresses, or null. */
export function operandSlot(text, frame) {
  const so = stackOperand(text);
  return so && frame?.slotOf ? frame.slotOf(so.reg, so.disp) : null;
}

/** The slot a variable name lives in, for sync (`symToSlot`). */
export function slotIndex(frame) {
  const symToSlot = new Map();
  const slotToSym = new Map();
  for (const s of frame.slots || []) {
    if (s.kind !== 'var' && s.kind !== 'arg') continue;
    if (!symToSlot.has(s.name)) symToSlot.set(s.name, s.offset);
    if (!slotToSym.has(s.offset)) slotToSym.set(s.offset, s.name);
  }
  return { symToSlot, slotToSym };
}

/** Callouts: a stack array and what lies above it. */
export function callouts(frame) {
  const notes = [];
  for (const s of frame.slots.filter((x) => x.array)) {
    const end = s.offset + s.size;
    const above = frame.slots.filter((x) => x.offset >= end && (x.kind === 'fp' || x.kind === 'ret' || x.kind === 'saved'))
      .sort((a, b) => a.offset - b.offset);
    const dist = -end;
    notes.push(`${s.name} (${s.type}) ends ${dist} byte${dist === 1 ? '' : 's'} below the return address. ` +
      `Writing past its end runs upward into ${above.map((x) => x.name).join(', then ')} — ` +
      'the classic stack buffer overflow: overwrite the return address and RET jumps wherever the attacker chose.');
  }
  if (frame.redZone) {
    notes.push('There is no SUB RSP here: this function calls nothing, so it keeps its locals in the 128-byte "red zone" below RSP, which the x86-64 System V ABI promises signal handlers will not touch.');
  }
  return notes;
}

/** The frame as an HTML table, highest address first. */
export function renderFrame(frame, { selectedSym = null } = {}) {
  if (!frame.supported) return `<div class="d2note">${escapeHtml(frame.note)}</div>`;
  let rows = '';
  for (const s of frame.slots) {
    const cls = [s.kind === 'var' || s.kind === 'arg' ? 'var' : s.kind, s.dim ? 'dim' : '', s.array ? 'arr' : '',
      selectedSym && s.name === selectedSym ? 'hl-sym' : ''].filter(Boolean).join(' ');
    const alias = s.alias ? ` <span class="d2muted">(the engine calls it ${escapeHtml(s.alias)})</span>` : '';
    const overlap = s.overlap ? ` <span class="d2muted">overlaps ${escapeHtml(s.overlap)}</span>` : '';
    const dim = s.dim ? ' <span class="d2muted">— debug info only, not in the C</span>' : '';
    rows += `<tr class="${cls}"${s.kind === 'var' || s.kind === 'arg' ? ` data-sym="${escapeHtml(s.name)}"` : ''} data-slot="${s.offset}">` +
      `<td class="off">entry${escapeHtml(hexOff(s.offset))}</td>` +
      `<td class="slot"><b>${escapeHtml(s.name)}</b>${alias} <span class="sz">${s.size} B</span><br>` +
      `<span class="d2muted">${escapeHtml(s.type || '')}</span>${dim}${overlap}</td></tr>`;
  }
  const notes = callouts(frame).map((n) => `<div class="d2callout">${escapeHtml(n)}</div>`).join('');
  const fpNote = frame.fp !== null
    ? `RBP points at entry${hexOff(frame.fp)}, so an operand [RBP − d] is entry${hexOff(frame.fp)} − d.`
    : 'This function does not set up RBP as a frame pointer; operands are relative to RSP.';
  return '<p class="d2muted" style="margin:0 0 10px;max-width:74ch">Offsets are from the stack pointer at entry, where it points at the ' +
    `return address; the stack grows down the page. ${escapeHtml(fpNote)}${frame.reserve ? ` SUB RSP reserves ${frame.reserve} bytes.` : ''}</p>` +
    `<table class="d2frame">${rows}</table>${notes}`;
}
