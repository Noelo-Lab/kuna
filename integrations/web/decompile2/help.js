// help.js — the study view's help dialog: every shortcut, a glossary of the
// names a decompiler invents, and what the colours mean. DOM-free: returns
// the dialog's HTML for app.js to mount.

const KEYS = [
  ['/', 'filter the function list'],
  ['Space', 'switch between C and assembly'],
  ['1 2 3 4', 'C · Assembly · Bytes · Stack'],
  ['s', 'split view: C beside the other pane'],
  ['o', 'instruction addresses: absolute · offset · both'],
  ['b', 'bytes column on/off'],
  ['↑ ↓', 'move through lines or instructions (the hover card follows)'],
  ['← →', 'move between names on a C line'],
  ['Enter', 'open the called function under the cursor'],
  ['n', 'rename the selected variable or function'],
  ['y', 'retype it (on a function name: edit its prototype)'],
  [';', 'comment the selected instruction'],
  ['g', 'go to a function or an address'],
  ['x', 'references: callers, callees, data'],
  ['u · Ctrl+Z', 'undo'],
  ['Ctrl+Shift+Z', 'redo'],
  ['Alt+← Alt+→', 'back and forward between functions'],
  ['?', 'this help'],
  ['Esc', 'close the card, then the dialog, then clear the selection'],
];

const GLOSSARY = [
  ['a0, a1 … / param_1', 'parameters the decompiler could not name; numbered in argument order'],
  ['v1, v2 …', 'locals the decompiler introduced; the same register or slot can hold several over a function'],
  ['local_28', 'a stack slot named by its offset (hex)'],
  ['dat_4010', 'a global at that address with no symbol'],
  ['sub_401000 / FUN_00401000', 'a function with no symbol, named by its entry address'],
  ['LAB_00401234', 'a jump target the structurer could not turn into a loop or an if'],
  ['undefined4 · int4 · uint8', 'a value of that many bytes whose type is unknown · signed · unsigned'],
  ['// rax', 'the variable lives in that register'],
  ['// stack - 0x14', 'the variable lives 0x14 bytes below the stack pointer at entry'],
  ['CONCAT44(a,b)', 'two 4-byte values glued into an 8-byte one (a is the high half)'],
  ['SEXT48(x) · ZEXT48(x)', 'sign- or zero-extend a 4-byte value to 8 bytes'],
  ['x._4_8_', 'bytes 4 to 11 of x (offset 4, length 8)'],
  ['(int)x', 'a cast: the program reinterprets or converts x'],
];

const LEGEND = [
  ['var(--band-0)', 'colour band: a C line and the instructions it came from share a colour'],
  ['repeating-linear-gradient(var(--band-0) 0 3px, transparent 3px 5px)', 'dashed band: an instruction the engine did not map, attributed to the line it sets up or finishes (inferred; view ▾ turns it off)'],
  ['var(--hl-sym)', 'the selected name, everywhere it is used'],
  ['var(--patch-bg)', 'a byte you patched (hover it for the original)'],
  ['var(--red)', 'jumps, calls and returns; a rejected edit (✗)'],
  ['var(--ok)', 'an edit the engine applied (✓)'],
];

function dl(rows) {
  return `<dl>${rows.map(([k, v]) => `<dt>${k.split(' · ').map((x) => `<kbd>${x}</kbd>`).join(' ')}</dt><dd>${v}</dd>`).join('')}</dl>`;
}

export function helpHtml() {
  return '<div class="hh"><b id="helptitle">Study view — help</b><button class="d2-ib" data-act="help-close" aria-label="Close">×</button></div>' +
    '<div class="hb2"><div><h3>Keys</h3>' + dl(KEYS) +
    '<h3>Colours</h3><dl>' + LEGEND.map(([c, v]) => `<dt><span class="d2sw" style="background:${c}"></span></dt><dd>${v}</dd>`).join('') + '</dl></div>' +
    '<div><h3>Names the decompiler invents</h3><dl>' + GLOSSARY.map(([k, v]) => `<dt>${k}</dt><dd>${v}</dd>`).join('') + '</dl>' +
    '<h3>How edits work</h3><p>Each rename, retype, comment or patch is one <code>--assert</code> directive. The page ' +
    're-runs the decompiler with all of them after every edit, keeps them for this binary in your browser, and ' +
    '<b>export</b> saves them as a <code>.kuna</code> file the command-line tool replays: ' +
    '<code>kuna decompile prog main --assert @prog.kuna</code>.</p></div></div>';
}

export const HELP_KEYS = KEYS;
export const HELP_GLOSSARY = GLOSSARY;
