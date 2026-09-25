// mnemonics.js — plain-language notes for the instructions a student meets
// most, and the idioms compilers build from them (prologues, canaries,
// zeroing, sign extension). SLEIGH prints x86 mnemonics in upper case and ARM
// ones in lower case; lookups ignore case. DOM-free.

const X86 = {
  MOV: 'copy the right operand into the left one',
  MOVZX: 'copy a smaller value, filling the upper bits with zeros (unsigned widening)',
  MOVSX: 'copy a smaller value, copying its sign bit upward (signed widening)',
  MOVSXD: 'copy a 32-bit value into a 64-bit register, sign-extended',
  MOVABS: 'load a full 64-bit constant',
  LEA: 'compute an address (no memory is read) — also used for cheap arithmetic',
  PUSH: 'decrement the stack pointer and store the operand at the new top of stack',
  POP: 'load the top of stack into the operand and increment the stack pointer',
  CALL: 'push the return address and jump to the function',
  RET: 'pop the return address and jump back to the caller',
  JMP: 'jump unconditionally',
  CMP: 'subtract right from left to set the flags, discarding the result',
  TEST: 'AND the operands to set the flags, discarding the result',
  ADD: 'add the right operand to the left one',
  SUB: 'subtract the right operand from the left one',
  ADC: 'add with the carry flag (multi-word arithmetic)',
  SBB: 'subtract with borrow (multi-word arithmetic)',
  INC: 'add 1',
  DEC: 'subtract 1',
  NEG: 'negate (two\'s complement)',
  NOT: 'invert every bit',
  AND: 'bitwise AND',
  OR: 'bitwise OR',
  XOR: 'bitwise exclusive OR',
  SHL: 'shift left (multiply by a power of two)',
  SAL: 'shift left (multiply by a power of two)',
  SHR: 'shift right, filling with zeros (unsigned divide by a power of two)',
  SAR: 'shift right, copying the sign bit (signed divide by a power of two)',
  ROL: 'rotate left',
  ROR: 'rotate right',
  IMUL: 'signed multiply',
  MUL: 'unsigned multiply into RDX:RAX',
  IDIV: 'signed divide RDX:RAX; quotient in RAX, remainder in RDX',
  DIV: 'unsigned divide RDX:RAX; quotient in RAX, remainder in RDX',
  CDQ: 'sign-extend EAX into EDX:EAX (before a 32-bit IDIV)',
  CQO: 'sign-extend RAX into RDX:RAX (before a 64-bit IDIV)',
  CDQE: 'sign-extend EAX into RAX (int to long)',
  CWDE: 'sign-extend AX into EAX',
  CBW: 'sign-extend AL into AX',
  LEAVE: 'tear down the frame: RSP = RBP, then POP RBP',
  ENTER: 'build a stack frame (rare; compilers use PUSH/MOV/SUB)',
  NOP: 'do nothing (padding or alignment)',
  ENDBR64: 'CET landing pad: marks a legal target for an indirect jump or call',
  ENDBR32: 'CET landing pad: marks a legal target for an indirect jump or call',
  HLT: 'halt the processor (privileged; traps in user code)',
  INT3: 'breakpoint trap',
  INT: 'software interrupt',
  UD2: 'guaranteed invalid instruction (marks unreachable code)',
  SYSCALL: 'enter the kernel: the system call number is in RAX',
  XCHG: 'swap the two operands',
  XADD: 'exchange and add (with LOCK: an atomic fetch-and-add)',
  CMPXCHG: 'compare with RAX and swap if equal (with LOCK: an atomic compare-and-swap)',
  BT: 'copy one bit into the carry flag',
  BSF: 'index of the lowest set bit',
  BSR: 'index of the highest set bit',
  MOVSB: 'copy a byte from [RSI] to [RDI]', MOVSD: 'copy 4 bytes from [RSI] to [RDI] (or a scalar double, with XMM operands)',
  MOVSQ: 'copy 8 bytes from [RSI] to [RDI]',
  STOSB: 'store AL at [RDI]', STOSD: 'store EAX at [RDI]', STOSQ: 'store RAX at [RDI]',
  LODSB: 'load a byte from [RSI]', CMPSB: 'compare bytes at [RSI] and [RDI]', SCASB: 'compare AL with the byte at [RDI]',
  MOVAPS: 'copy 16 aligned bytes between XMM registers or memory', MOVUPS: 'copy 16 bytes (unaligned)',
  MOVAPD: 'copy 16 aligned bytes', MOVDQA: 'copy 16 aligned bytes', MOVDQU: 'copy 16 bytes (unaligned)',
  MOVSS: 'copy a single-precision float', MOVQ: 'copy 8 bytes between a general and an XMM register', MOVD: 'copy 4 bytes between a general and an XMM register',
  PXOR: 'bitwise XOR of XMM registers (XOR with itself zeroes it)', XORPS: 'bitwise XOR of XMM registers (XOR with itself zeroes it)',
  ADDSD: 'add doubles', SUBSD: 'subtract doubles', MULSD: 'multiply doubles', DIVSD: 'divide doubles',
  ADDSS: 'add floats', SUBSS: 'subtract floats', MULSS: 'multiply floats', DIVSS: 'divide floats',
  CVTSI2SD: 'convert an integer to a double', CVTTSD2SI: 'convert a double to an integer, truncating',
  CVTSI2SS: 'convert an integer to a float', CVTTSS2SI: 'convert a float to an integer, truncating',
  CVTSS2SD: 'widen a float to a double', CVTSD2SS: 'narrow a double to a float',
  UCOMISD: 'compare doubles, setting the flags', COMISD: 'compare doubles, setting the flags',
  UCOMISS: 'compare floats, setting the flags', COMISS: 'compare floats, setting the flags',
  CLD: 'clear the direction flag (string ops move forward)', STD: 'set the direction flag (string ops move backward)',
  CPUID: 'query the processor', RDTSC: 'read the time-stamp counter',
};

const X86_CC = {
  O: 'overflow', NO: 'no overflow', B: 'below (unsigned <)', C: 'below (unsigned <)', NAE: 'below (unsigned <)',
  AE: 'above or equal (unsigned >=)', NB: 'above or equal (unsigned >=)', NC: 'above or equal (unsigned >=)',
  E: 'equal', Z: 'zero / equal', NE: 'not equal', NZ: 'not zero / not equal',
  BE: 'below or equal (unsigned <=)', NA: 'below or equal (unsigned <=)',
  A: 'above (unsigned >)', NBE: 'above (unsigned >)',
  S: 'negative (sign set)', NS: 'not negative', P: 'parity even', PE: 'parity even', NP: 'parity odd', PO: 'parity odd',
  L: 'less (signed <)', NGE: 'less (signed <)', GE: 'greater or equal (signed >=)', NL: 'greater or equal (signed >=)',
  LE: 'less or equal (signed <=)', NG: 'less or equal (signed <=)', G: 'greater (signed >)', NLE: 'greater (signed >)',
};

const A64 = {
  mov: 'copy a register or constant', movz: 'load a 16-bit constant, zeroing the rest', movk: 'insert a 16-bit constant, keeping the rest',
  movn: 'load the inverse of a 16-bit constant', mvn: 'copy the bitwise inverse',
  ldr: 'load a register from memory', ldrb: 'load a byte, zero-extended', ldrh: 'load 2 bytes, zero-extended',
  ldrsb: 'load a byte, sign-extended', ldrsh: 'load 2 bytes, sign-extended', ldrsw: 'load 4 bytes, sign-extended',
  ldur: 'load from an unscaled offset', str: 'store a register to memory', strb: 'store a byte', strh: 'store 2 bytes', stur: 'store to an unscaled offset',
  ldp: 'load a pair of registers', stp: 'store a pair of registers',
  add: 'add two values', adds: 'add and set the flags', sub: 'subtract', subs: 'subtract and set the flags',
  cmp: 'compare (subtract, keeping only the flags)', cmn: 'compare with the negated operand', tst: 'test bits (AND, keeping only the flags)',
  and: 'bitwise AND', ands: 'bitwise AND and set the flags', orr: 'bitwise OR', eor: 'bitwise exclusive OR', bic: 'AND with the inverse',
  lsl: 'shift left', lsr: 'shift right, filling with zeros', asr: 'shift right, copying the sign bit', ror: 'rotate right',
  mul: 'multiply', madd: 'multiply and add', msub: 'multiply and subtract', smull: 'signed 32x32 to 64-bit multiply', umull: 'unsigned 32x32 to 64-bit multiply',
  sdiv: 'signed divide', udiv: 'unsigned divide', neg: 'negate',
  b: 'branch unconditionally', bl: 'call: branch and put the return address in x30 (LR)', blr: 'call through a register', br: 'branch through a register',
  ret: 'return to the address in x30 (LR)', cbz: 'branch if the register is zero', cbnz: 'branch if the register is not zero',
  tbz: 'branch if the bit is zero', tbnz: 'branch if the bit is not zero',
  adr: 'compute a PC-relative address', adrp: 'compute the 4 KiB page of a PC-relative address',
  csel: 'choose one of two registers by a condition', cset: 'set to 1 if the condition holds, else 0', csinc: 'choose, or increment the second',
  sxtw: 'sign-extend a 32-bit value', uxtw: 'zero-extend a 32-bit value', sxtb: 'sign-extend a byte', uxtb: 'zero-extend a byte',
  sxth: 'sign-extend 2 bytes', uxth: 'zero-extend 2 bytes', nop: 'do nothing', svc: 'enter the kernel (system call)',
  paciasp: 'sign the return address (pointer authentication)', autiasp: 'check the signed return address', bti: 'branch-target landing pad',
};

const A32 = {
  mov: 'copy a register or constant', mvn: 'copy the bitwise inverse', ldr: 'load a word from memory', str: 'store a word',
  ldrb: 'load a byte', strb: 'store a byte', ldrh: 'load 2 bytes', strh: 'store 2 bytes', ldm: 'load several registers', stm: 'store several registers',
  push: 'store registers on the stack', pop: 'load registers from the stack (popping pc returns)',
  add: 'add two values', sub: 'subtract', rsb: 'reverse subtract', cmp: 'compare (subtract, keeping only the flags)', cmn: 'compare with the negated operand',
  tst: 'test bits', teq: 'test equality (XOR, keeping only the flags)', and: 'bitwise AND', orr: 'bitwise OR', eor: 'bitwise exclusive OR', bic: 'AND with the inverse',
  lsl: 'shift left', lsr: 'shift right, filling with zeros', asr: 'shift right, copying the sign bit', mul: 'multiply', mla: 'multiply and add',
  b: 'branch', bl: 'call: branch and put the return address in lr', bx: 'branch to a register (lr returns)', blx: 'call through a register',
  cpy: 'copy a register', nop: 'do nothing', svc: 'enter the kernel (system call)',
};

const ARM_CC = {
  eq: 'equal', ne: 'not equal', cs: 'carry set (unsigned >=)', hs: 'unsigned >=', cc: 'carry clear (unsigned <)', lo: 'unsigned <',
  mi: 'negative', pl: 'positive or zero', vs: 'overflow', vc: 'no overflow', hi: 'unsigned >', ls: 'unsigned <=',
  ge: 'signed >=', lt: 'signed <', gt: 'signed >', le: 'signed <=', al: 'always',
};

/** The mnemonic tables, for tests and the help dialog. */
export const TABLES = { x86: X86, x86cc: X86_CC, aarch64: A64, arm: A32, armcc: ARM_CC };

function explainX86(raw) {
  let m = raw.toUpperCase().trim();
  const prefixes = [];
  for (;;) {
    const p = /^(LOCK|REP|REPE|REPZ|REPNE|REPNZ|BND|NOTRACK)\s+/.exec(m);
    if (!p) break;
    prefixes.push(p[1]);
    m = m.slice(p[0].length);
  }
  let text = X86[m];
  if (!text) {
    const cc = /^(J|SET|CMOV)(N?[A-Z]{1,3})$/.exec(m);
    if (cc && X86_CC[cc[2]]) {
      const what = X86_CC[cc[2]];
      text = cc[1] === 'J' ? `jump if ${what}` : cc[1] === 'SET' ? `set the byte to 1 if ${what}, else 0` : `copy only if ${what}`;
      text += ' — decided by the flags of the last CMP/TEST/arithmetic';
    }
  }
  if (!text) return null;
  const pre = prefixes.map((p) => (p === 'LOCK' ? 'atomically: ' : p.startsWith('REP') ? 'repeat RCX times: ' : '')).join('');
  return pre + text;
}

function explainArm(raw, table) {
  let m = raw.toLowerCase().trim().replace(/\.w$|\.n$/, '');
  let text = table[m];
  if (text) return text;
  const dot = /^b\.([a-z]{2})$/.exec(m);
  if (dot && ARM_CC[dot[1]]) return `branch if ${ARM_CC[dot[1]]} — decided by the flags of the last compare`;
  for (const cc of Object.keys(ARM_CC)) {
    if (m.endsWith(cc) && table[m.slice(0, -2)]) return `${table[m.slice(0, -2)]}, only if ${ARM_CC[cc]}`;
  }
  if (m.endsWith('s') && table[m.slice(0, -1)]) return `${table[m.slice(0, -1)]}, and set the flags`;
  return null;
}

/** A one-line explanation of `mnemonic` on `family` (x86 | aarch64 | arm), or null. */
export function explain(mnemonic, family = 'x86') {
  if (!mnemonic) return null;
  if (family === 'aarch64') return explainArm(mnemonic, A64);
  if (family === 'arm') return explainArm(mnemonic, A32);
  return explainX86(mnemonic);
}

const FS28 = /FS:\[0x28\]/i;

/**
 * Idiom labels by instruction address: prologues and epilogues, stack-canary
 * loads and checks, `xor r,r`, `test r,r`, `cdqe`, `endbr64`, the variadic
 * `mov eax,0` before a call.
 */
export function idioms(instructions, family = 'x86', { nameAt = () => null } = {}) {
  const out = new Map();
  const at = (i) => instructions[i];
  const set = (i, label) => { if (at(i) && !out.has(at(i).address_hex)) out.set(at(i).address_hex, label); };
  if (family === 'aarch64') {
    instructions.forEach((insn, i) => {
      const m = insn.mnemonic.toLowerCase();
      const ops = insn.operands.toLowerCase().replace(/\s+/g, '');
      if (m === 'stp' && /^x29,x30,\[sp/.test(ops)) set(i, 'prologue: save the frame pointer and return address');
      else if (m === 'mov' && ops === 'x29,sp') set(i, 'prologue: x29 now marks this frame');
      else if (m === 'ldp' && /^x29,x30,\[sp/.test(ops)) set(i, 'epilogue: restore the frame pointer and return address');
      else if (m === 'ret') set(i, 'return to the caller');
      else if (/^(paciasp|autiasp)$/.test(m)) set(i, 'pointer authentication of the return address');
    });
    return out;
  }
  if (family !== 'x86') return out;
  instructions.forEach((insn, i) => {
    const m = insn.mnemonic.toUpperCase();
    const ops = insn.operands.replace(/\s+/g, ' ').trim();
    const [a, b] = ops.split(',').map((s) => s.trim());
    if (m === 'ENDBR64' || m === 'ENDBR32') set(i, 'landing pad (CET)');
    else if (m === 'PUSH' && /^[RE]BP$/i.test(ops) && i <= 2) set(i, 'prologue: save the caller\'s frame pointer');
    else if (m === 'MOV' && /^[RE]BP$/i.test(a) && /^[RE]SP$/i.test(b)) set(i, 'prologue: RBP now marks this frame');
    else if (m === 'SUB' && /^[RE]SP$/i.test(a) && /^0x[0-9a-f]+$/i.test(b || '') && i <= 6) set(i, `prologue: reserve ${Number(BigInt(b))} bytes for locals`);
    else if (m === 'MOV' && FS28.test(ops)) set(i, 'stack canary: load the guard value');
    else if ((m === 'SUB' || m === 'XOR' || m === 'CMP') && FS28.test(ops)) set(i, 'stack canary: check the guard before returning');
    else if (m === 'XOR' && a && a === b) set(i, `zero ${a} (shorter than MOV ${a},0)`);
    else if (m === 'TEST' && a && a === b) set(i, `compare ${a} with 0`);
    else if (m === 'CDQE') set(i, 'int to long: sign-extend EAX into RAX');
    else if (m === 'MOV' && /^EAX$/i.test(a) && /^0x0$/i.test(b || '') && at(i + 1) && /^CALL$/i.test(at(i + 1).mnemonic)) set(i, 'variadic call: AL = 0 vector registers used');
    else if (m === 'LEAVE') set(i, 'epilogue: RSP = RBP; POP RBP');
    else if (m === 'POP' && /^[RE]BP$/i.test(ops) && at(i + 1) && /^RET/i.test(at(i + 1).mnemonic)) set(i, 'epilogue: restore the caller\'s frame pointer');
    else if (m === 'RET') set(i, 'return to the caller');
    else if (m === 'CALL' && /__stack_chk_fail/.test(`${ops} ${nameAt(/0x[0-9a-f]+/i.exec(ops)?.[0]?.toLowerCase()) || ''}`)) set(i, 'stack canary mismatch: abort');
  });
  return out;
}
