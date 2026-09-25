// help.js — "How to use Kuna": the keys worth knowing first, every shortcut
// behind a disclosure, and the names a decompiler invents, each in one short
// line. DOM-free: returns the dialog's HTML for app.js to mount.

const TOP_KEYS = [
  ['/', 'Search functions'],
  ['Space', 'Switch between C code and assembly'],
  ['1 – 4', 'C code · Assembly · Bytes · Stack'],
  ['s', 'Side by side'],
  ['n', 'Rename what is selected'],
  ['y', 'Change its type'],
  ['u', 'Undo'],
  ['Esc', 'Close or clear the selection'],
];

const KEYS = [
  ['/', 'search functions'],
  ['Space', 'switch between C code and assembly'],
  ['1 2 3 4', 'C code · Assembly · Bytes · Stack'],
  ['s', 'side by side (C next to the assembly)'],
  ['o', 'addresses: full · offset from the function start · both'],
  ['b', 'show or hide instruction bytes'],
  ['↑ ↓', 'move through lines or instructions'],
  ['← →', 'move between names on a C line'],
  ['Enter', 'open the function under the cursor'],
  ['n', 'rename the selected variable or function'],
  ['y', 'change its type (on a function name: its signature)'],
  [';', 'add a note to the selected instruction'],
  ['g', 'go to a function or an address'],
  ['x', 'find who calls this function'],
  ['u · Ctrl+Z', 'undo'],
  ['Ctrl+Shift+Z', 'redo'],
  ['Alt+← Alt+→', 'back and forward between functions'],
  ['?', 'this help'],
  ['Esc', 'close the card, then a dialog, then clear the selection'],
];

const GLOSSARY = [
  ['v1, v2 …', 'a local variable the decompiler named; rename it to what it means'],
  ['a0, a1 … / param_1', 'an input (parameter) without a name, in order'],
  ['local_28', 'a stack slot, named by its position'],
  ['dat_4010', 'a global variable at that address'],
  ['sub_401000 / FUN_00401000', 'a function without a name, at that address'],
  ['LAB_00401234', 'a place the code jumps to'],
  ['undefined4 · int4 · uint8', 'a value of that many bytes: type unknown · signed · unsigned'],
  ['// rax', 'this variable lives in the register RAX'],
  ['// stack - 0x14', 'it lives on the stack, 0x14 bytes below the return address'],
  ['CONCAT44(a,b)', 'two 4-byte values joined into 8 bytes'],
  ['SEXT48(x) · ZEXT48(x)', 'x widened from 4 to 8 bytes, keeping its sign · with zeros'],
  ['x._4_8_', 'the 8 bytes of x that start at byte 4'],
  ['(int)x', 'a cast: x read as an int'],
];

const LEGEND = [
  ['var(--accent-bg)', 'a line and the instructions it turns into'],
  ['var(--select-bg)', 'the name you selected, everywhere it is used'],
  ['var(--patch-bg)', 'a byte you changed'],
  ['repeating-linear-gradient(var(--band-0) 0 3px, transparent 3px 5px)', 'side by side: a dashed bar marks an instruction the page grouped with the line it sets up (turn off in View options)'],
];

const kbd = (k) => k.split(/ · | – /).map((x) => `<kbd>${x}</kbd>`).join(k.includes('–') ? ' – ' : ' ');

export function helpHtml() {
  const rows = (list) => list.map(([k, v]) => `<tr><td>${kbd(k)}</td><td>${v}</td></tr>`).join('');
  return '<div class="hh"><h2 id="helptitle">How to use Kuna</h2>' +
    '<button class="d2-iconbtn small" data-act="help-close" aria-label="Close" title="Close">×</button></div>' +
    '<div class="hb2">' +
    '<p>Pick a function on the left. Hover over a line of C to see the assembly it becomes; click a name to learn about it, and double-click to rename it. Your changes are kept in this browser.</p>' +
    `<h3>Keyboard</h3><table>${rows(TOP_KEYS)}</table>` +
    `<details id="helpall"><summary>All shortcuts</summary><table>${rows(KEYS)}</table></details>` +
    '<h3>Words you\'ll see</h3><dl>' + GLOSSARY.map(([k, v]) => `<dt>${k}</dt><dd>${v}</dd>`).join('') + '</dl>' +
    '<details><summary>What the colours mean</summary><dl>' +
    LEGEND.map(([c, v]) => `<dt><span class="d2sw" style="background:${c}"></span></dt><dd>${v}</dd>`).join('') + '</dl></details>' +
    '</div>';
}

export const HELP_KEYS = KEYS;
export const HELP_GLOSSARY = GLOSSARY;
