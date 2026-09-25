// arch.js — which instruction set a function is in, and how to fill a span of
// it with no-ops. The engine's language id decides when it is known; register
// names in the operands are the fallback. DOM-free.

/**
 * `{family, endian, bits, thumb}` for a target: family is x86 | aarch64 | arm
 * | mips | ppc | riscv, or null when unknown.
 */
export function archFrom(target, instructions = []) {
  const id = (target?.archid || target?.processor || '').toString();
  const [proc = '', end = '', bits = ''] = id.split(':');
  const endian = /^BE$/i.test(end) || target?.endian === 'big' ? 'big' : 'little';
  const width = Number(bits) || target?.bits || null;
  const p = proc.toLowerCase();
  const sizes = new Set(instructions.map((i) => i.size));
  let family = null;
  if (/^x86/.test(p)) family = 'x86';
  else if (/aarch64|arm64/.test(p)) family = 'aarch64';
  else if (/^arm/.test(p)) family = 'arm';
  else if (/^mips/.test(p)) family = 'mips';
  else if (/powerpc|ppc/.test(p)) family = 'ppc';
  else if (/risc-?v/.test(p)) family = 'riscv';
  if (!family) {
    const ops = instructions.map((i) => `${i.mnemonic} ${i.operands}`).join('\n');
    if (/\b(R|E)(AX|BX|CX|DX|SP|BP|SI|DI)\b/i.test(ops)) family = 'x86';
    else if (/\b[XW]([12]?\d|30)\b/i.test(ops) && [...sizes].every((s) => s === 4)) family = 'aarch64';
    else if (/\bR(1[0-5]|\d)\b/i.test(ops)) family = 'arm';
  }
  const thumb = family === 'arm' && (/thumb|t$/i.test(id.split(':')[3] || '') || sizes.has(2));
  return { family, endian, bits: width, thumb };
}

const NOPS = {
  x86: ['90', 1],
  aarch64: ['1f2003d5', 4],
  arm: ['0000a0e1', 4],
  thumb: ['00bf', 2],
  mips: ['00000000', 4],
  ppcBE: ['60000000', 4],
  ppcLE: ['00000060', 4],
  riscv: ['13000000', 4],
  riscvC: ['0100', 2],
};

/** Hex bytes that no-op `size` bytes on `arch`, or null when there is no exact fill. */
export function nopFill(arch, size) {
  if (!arch?.family || !(size > 0)) return null;
  let nop;
  switch (arch.family) {
    case 'x86': nop = NOPS.x86; break;
    case 'aarch64': nop = NOPS.aarch64; break;
    case 'arm': nop = arch.thumb ? NOPS.thumb : NOPS.arm; break;
    case 'mips': nop = NOPS.mips; break;
    case 'ppc': nop = arch.endian === 'big' ? NOPS.ppcBE : NOPS.ppcLE; break;
    case 'riscv': nop = size % 4 === 0 ? NOPS.riscv : NOPS.riscvC; break;
    default: return null;
  }
  const [hex, width] = nop;
  if (size % width !== 0) return null;
  return hex.repeat(size / width);
}
