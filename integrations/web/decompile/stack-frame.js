// stack-frame.js — a diagram of one function's stack frame, drawn from the
// prologue and the names the code shows: each local at every stack location its
// declaration's comment gives, and each parameter the engine keeps on the stack,
// beside the return address, the registers the prologue saves and the stack
// canary. Offsets are measured from the stack pointer at the function's entry,
// where it points at the return address (after `PUSH RBP; MOV RBP,RSP`,
// `[RBP - 0xc]` is entry − 0x14), and read as the comments write them
// (`rbp - 0xc`). x86 only; other targets get a note. DOM-free.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { stackOperand } from './asm-view.js';
import { localDecls, stackLocation, stackPieces } from './render-c.js';
import { parseRustSignature, parseSignature, rustTypeSize, typeSize } from './ctype.js';

/**
 * The prologue's effect: pushes (in order), where RBP points (entry offset),
 * the bytes reserved below them (`SUB RSP,N`, `ADD RSP,-N`), and where RSP
 * sits once the prologue is done. The scan steps over instructions that leave
 * RSP alone (compilers interleave them) and over a conditional jump before the
 * first push (a shrink-wrapped early exit), and stops at other control flow, at
 * anything else that moves RSP, and at a push once locals are reserved (an
 * argument). A push below the stack pointer `inspect`'s `frame` gives is an
 * argument too, and a push of a scratch register only pads the stack.
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
    if (/^J/.test(m) && m !== 'JMP' && !pushes.length && !reserve) continue;
    if (/^(J|CALL|RET|LEAVE|LOOP|POP|ENTER)/.test(m)) break;
    if (m === 'PUSH') {
      if (reserve || !/^[RE]?[A-Z0-9]+$/i.test(ops)) break;
      sp -= word;
      pushes.push({ reg: ops.toUpperCase(), offset: sp });
      continue;
    }
    if (m === 'MOV' && /^[RE]BP,[RE]SP$/i.test(ops) && /BP$/.test(pushes.at(-1)?.reg || '') && pushes.at(-1).offset === sp) { fp = sp; continue; }
    const alloc = /^[RE]SP,(-?)0x([0-9a-f]+)$/i.exec(ops);
    if (alloc && ((m === 'SUB' && !alloc[1]) || (m === 'ADD' && alloc[1]))) {
      const n = Number(BigInt(`0x${alloc[2]}`));
      reserve += n;
      sp -= n;
      continue;
    }
    if (/^[RE]SP(,|$)/i.test(ops)) break;
  }
  return { pushes, fp, reserve, sp, word };
}

/** The entry offset the prologue stores the stack canary at (`MOV RAX,qword ptr FS:[0x28]` or `MOV EAX,GS:[0x14]`, then a stack store of it), or null. */
export function canaryOffset(instructions, slotOf) {
  const insns = (instructions || []).slice(0, 32);
  for (let i = 0; i < insns.length; i++) {
    const load = /^([A-Z0-9]+),(?:(?:qword|dword)ptr)?[FG]S:\[0x(?:28|14)\]$/i.exec(insns[i].operands.replace(/\s+/g, ''));
    if (!load || insns[i].mnemonic.toUpperCase() !== 'MOV') continue;
    for (const insn of insns.slice(i + 1, i + 4)) {
      const so = stackOperand(insn.operands);
      if (insn.mnemonic.toUpperCase() !== 'MOV' || !so || so.start !== 0) continue;
      if (insn.operands.slice(so.end).replace(/[\s,]/g, '').toUpperCase() !== load[1].toUpperCase()) continue;
      return slotOf(so.reg, so.disp);
    }
  }
  return null;
}

/** The slots the prologue makes rather than the code: a code name that lands on one is shown on it. */
const MACHINE = new Set(['ret', 'fp', 'saved', 'canary']);

/** Registers no x86 calling convention preserves: pushing one in a prologue only pads the stack. */
const SCRATCH = /^([RE]?AX|[RE]?CX|[RE]?DX|R(8|9|10|11))$/;

/**
 * The frame, high addresses first: `{family, supported, slots, size, redZone,
 * fp, slotOf(reg, disp), where(offset), note}`; `fp` and `slotOf` follow
 * `inspect`'s `frame` when the engine gives one. A slot is `{offset, size, kind, name, type,
 * array, overlap, input}` with kind ret | fp | saved | canary | arg | var | pad.
 */
export function frameModel(fnData, arch = { family: 'x86', bits: 64 }) {
  if (arch.family !== 'x86') {
    return { family: arch.family, supported: false, slots: [], note: 'The frame diagram models x86 frames; this function is not x86.', slotOf: () => null };
  }
  const bits = arch.bits || 64;
  const word = bits / 8;
  const pro = prologue(fnData.instructions || [], bits);
  const engineSp = fnData.frame?.stack_pointer?.offset;
  if (Number.isInteger(engineSp)) {
    pro.pushes = pro.pushes.filter((p) => p.offset >= engineSp);
    if (engineSp < pro.sp) pro.reserve += pro.sp - engineSp;
    pro.sp = engineSp;
  }
  const engine = fnData.frame ? [fnData.frame.frame_pointer, fnData.frame.stack_pointer].filter(Boolean) : null;
  const fp = engine ? fnData.frame.frame_pointer : pro.fp === null ? null : { register: bits === 64 ? 'rbp' : 'ebp', offset: pro.fp };
  const slotOf = (reg, disp) => {
    if (engine) {
      const r = engine.find((e) => e.register === reg.toLowerCase());
      return r ? r.offset + disp : null;
    }
    if (/^[RE]BP$/.test(reg) && pro.fp !== null) return pro.fp + disp;
    if (/^[RE]SP$/.test(reg)) return pro.sp + disp;
    return null;
  };
  const slots = [{ offset: 0, size: word, kind: 'ret', name: 'return address', type: `pushed by the caller's CALL` }];
  for (const p of pro.pushes) {
    if (SCRATCH.test(p.reg)) continue;
    const isFp = /^[RE]BP$/.test(p.reg) && fp?.offset === p.offset;
    slots.push({ offset: p.offset, size: word, kind: isFp ? 'fp' : 'saved', name: `saved ${p.reg}`, type: 'the caller\'s value, restored before returning' });
  }
  const canary = canaryOffset(fnData.instructions, slotOf);
  if (Number.isInteger(canary) && canary < 0) slots.push({ offset: canary, size: word, kind: 'canary', name: 'stack canary', type: 'a random value checked before returning' });

  const seen = new Set();
  const place = (slot) => {
    const key = `${slot.offset}:${slot.size}:${slot.name}`;
    if (seen.has(key)) return;
    seen.add(key);
    const clash = slots.find((s) => MACHINE.has(s.kind) && s.offset === slot.offset && s.size === slot.size);
    if (clash && !clash.alias) Object.assign(clash, { alias: slot.name, aliasType: slot.type });
    else slots.push(slot);
  };
  const rust = /rust/i.test(fnData.language || '');
  const known = new Map((fnData.types || []).filter((t) => t?.name && t.size > 0).map((t) => [t.name, t.size]));
  const slotAt = (name, off) => fnData.variables?.find((v) => v.kind !== 'arg' && v.stack_offset === off && (name === null || v.name === name))?.size;
  const sizeOf = (d, off) => (rust ? rustTypeSize : typeSize)(d.type, { bits, known })
    || slotAt(d.name, off) || fnData.variables?.find((v) => v.name === d.name)?.size || slotAt(null, off) || 1;
  const isArray = (type) => (rust ? /^\[.+;\s*\d+\]$/ : /\[\d*\]/).test(type || '');
  for (const d of localDecls(fnData.code)) {
    const pieces = stackPieces(d.storage, fnData.frame).sort((a, b) => a - b);
    if (!pieces.length) continue;
    const home = fnData.variables?.find((v) => v.name === d.name && v.kind !== 'arg' && pieces.includes(v.stack_offset))?.stack_offset;
    const anchor = home ?? pieces[0];
    const boxes = [[anchor, sizeOf(d, anchor)]];
    for (const off of pieces) {
      if (boxes.some(([o, n]) => off >= o && off < o + n)) continue;
      const n = Math.min(sizeOf(d, off), slotAt(null, off) || word);
      boxes.push([off, off < 0 ? Math.min(n, -off) : n]);
    }
    boxes.forEach(([off, size], i) => place({ offset: off, size, kind: off >= word ? 'arg' : 'var', name: d.name, type: d.type, array: i === 0 && isArray(d.type) }));
  }
  const sig = rust ? parseRustSignature(fnData.proto || '') : parseSignature(fnData.proto || '');
  const params = sig?.params || [];
  for (const v of fnData.variables || []) {
    if (!Number.isInteger(v.stack_offset) || !v.size) continue;
    const at = v.kind === 'arg' ? v.arg_index : params.findIndex((p) => p.name === v.name);
    const p = Number.isInteger(at) && at >= 0 ? params[at] : null;
    if (!p?.name) continue;
    place({ offset: v.stack_offset, size: v.size, kind: v.stack_offset >= word ? 'arg' : 'var', name: p.name, type: p.type || v.type, input: at + 1 });
  }

  slots.sort((a, b) => b.offset - a.offset || b.size - a.size);
  const out = [];
  for (let i = 0; i < slots.length; i++) {
    const s = slots[i];
    const prev = out[out.length - 1];
    if (prev && prev.offset > s.offset + s.size && prev.kind !== 'pad') {
      const gapHi = prev.offset;
      const gapLo = s.offset + s.size;
      if (gapHi > gapLo && gapLo < 0) out.push({ offset: gapLo, size: gapHi - gapLo, kind: 'pad', name: 'unnamed space', type: 'no variable in the code lives here: padding, or a value the compiler keeps for itself' });
    }
    if (prev && s.offset + s.size > prev.offset && prev.kind !== 'pad') {
      s.overlap = prev.name;
    }
    out.push(s);
  }
  const lowest = Math.min(0, ...out.map((s) => s.offset));
  const frameLow = Math.min(pro.sp, lowest);
  if (out.length && frameLow < out[out.length - 1].offset) {
    const last = out[out.length - 1];
    out.push({ offset: frameLow, size: last.offset - frameLow, kind: 'pad', name: 'unnamed space', type: 'reserved by the prologue; no variable in the code lives here' });
  }
  const leaf = !(fnData.instructions || []).some((i) => /^CALL/i.test(i.mnemonic));
  return {
    family: 'x86',
    supported: true,
    word,
    slots: out,
    fp,
    reserve: pro.reserve,
    size: -Math.min(frameLow, lowest),
    redZone: leaf && !pro.reserve && lowest < pro.sp,
    slotOf,
    where: (offset) => stackLocation(offset, fnData.frame),
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
    const name = s.kind === 'var' || s.kind === 'arg' ? s.name : s.alias;
    if (!name) continue;
    if (!symToSlot.has(name)) symToSlot.set(name, s.offset);
    if (!slotToSym.has(s.offset)) slotToSym.set(s.offset, name);
  }
  return { symToSlot, slotToSym };
}

/** Callouts: a stack array and what lies above it. */
export function callouts(frame) {
  const notes = [];
  for (const s of frame.slots.filter((x) => x.array && x.offset + x.size <= 0)) {
    const end = s.offset + s.size;
    const above = frame.slots.filter((x) => x.offset >= end && (x.kind === 'fp' || x.kind === 'ret' || x.kind === 'saved' || x.kind === 'canary'))
      .sort((a, b) => a.offset - b.offset);
    const dist = -end;
    const guarded = above.some((x) => x.kind === 'canary');
    notes.push(`${s.name} (${s.type}) ends ${dist} byte${dist === 1 ? '' : 's'} below the return address. ` +
      `Writing past its end runs upward into ${above.map((x) => x.name).join(', then ')} — ` +
      'the classic stack buffer overflow: overwrite the return address and RET jumps wherever the attacker chose.' +
      (guarded ? ' The stack canary is checked before RET, so a write that changes it stops the program first.' : ''));
  }
  if (frame.redZone) {
    notes.push('There is no SUB RSP here: this function calls nothing, so it keeps its locals in the 128-byte "red zone" below RSP, which the x86-64 System V ABI promises signal handlers will not touch.');
  }
  return notes;
}

/** Bytes below (or above) the return address, in words. */
export function belowText(offset) {
  if (offset < 0) return `${-offset} byte${offset === -1 ? '' : 's'} below the return address`;
  return offset === 0 ? 'the return address' : `${offset} byte${offset === 1 ? '' : 's'} above the return address`;
}

const SLOT_NOTE = {
  ret: 'put there by the CALL that ran this function',
  fp: 'the caller\'s frame pointer, put back before returning',
  saved: 'a register this function gives back unchanged',
  canary: 'a random value checked before returning',
};

/** The frame as an HTML table, highest address first: one line per slot, each at the location the code's comments use. */
export function renderFrame(frame, { selectedSym = null } = {}) {
  if (!frame.supported) return `<div class="d2note">${escapeHtml(frame.note)}</div>`;
  let rows = '';
  for (const s of frame.slots) {
    const sym = s.kind === 'var' || s.kind === 'arg' ? s.name : s.alias || null;
    const cls = [sym && !s.alias ? 'var' : s.kind, s.array ? 'arr' : '',
      selectedSym && sym === selectedSym ? 'hl-sym' : ''].filter(Boolean).join(' ');
    const teach = !!SLOT_NOTE[s.kind] && !s.alias;
    const note = s.alias ? `the ${s.name}: ${SLOT_NOTE[s.kind]}` : SLOT_NOTE[s.kind] || (s.input ? `input ${s.input}, kept on the stack` : '');
    const type = s.alias ? s.aliasType : sym && s.type;
    rows += `<tr class="${cls}"${sym ? ` data-sym="${escapeHtml(sym)}"` : ''} data-slot="${s.offset}">` +
      `<td class="off" title="${escapeHtml(belowText(s.offset))}">${escapeHtml(frame.where(s.offset))}</td>` +
      `<td class="slot"><div class="sl"><span class="sn">${escapeHtml(sym || s.name)}</span>` +
      `${type ? `<span class="st">${escapeHtml(type)}</span>` : ''}` +
      `${note ? `<span class="sa${teach ? ' d2-teach' : ''}" title="${escapeHtml(note)}">${escapeHtml(note)}</span>` : ''}` +
      `<span class="sz">${s.size} byte${s.size === 1 ? '' : 's'}</span></div></td></tr>`;
  }
  const notes = callouts(frame).map((n) => `<div class="d2callout">${escapeHtml(n)}</div>`).join('');
  return '<p class="d2-stacklede">Higher addresses are at the top. Each box is one thing the function keeps on the stack, at the location the code\'s comments give.</p>' +
    '<table class="d2frame"><colgroup><col class="off"><col></colgroup>' +
    '<thead><tr><th class="off">Location</th><th>What is stored there</th></tr></thead>' +
    `<tbody>${rows}</tbody></table>${notes}`;
}
