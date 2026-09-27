// decompile2-collab.mjs — live sessions in the study view, from the source
// tree with no build and no browser: the replicated registers converge under
// any delivery order and refuse ops that would make the engine read a file;
// a session reads back as registers and the registers as the same directives,
// in one canonical order; Undo leaves what someone else changed since; the
// invite and reply codes round-trip and refuse what is not theirs; the SDP is
// cut down and rebuilt without letting a peer add a line; and whole groups of
// pages, joined by in-memory links, introduce newcomers to everyone, send
// them the program, converge, stop at 8 people, drop malformed messages, and
// refuse another build.
import assert from 'node:assert/strict';
import { createHash, randomBytes } from 'node:crypto';
import { Session } from '../decompile2/session.js';
import {
  Replica, validOp, registersOf, applyRegisters, adoptRawKeys, describeRegister, recordKeyOf, History, MODES,
} from '../decompile2/collab/replica.js';
import {
  PROTOCOL, MAX_PEERS, COLORS, validMessage, readMessage, maxBytes, encodeCode, decodeCode, codeFrom, limiter,
  randomId, initials, validAnchor,
} from '../decompile2/collab/wire.js';
import { compactSdp, expandSdp, validSdp, passiveAnswer } from '../decompile2/collab/sdp.js';
import { sha256Js, sha256Hex } from '../decompile2/collab/sha256.js';
import { Group, resolveColors } from '../decompile2/collab/group.js';

const checks = [];
const sleep = (ms) => new Promise((done) => setTimeout(done, ms));

// ── SHA-256 without WebCrypto ──────────────────────────────────────────────
for (const n of [0, 1, 55, 56, 63, 64, 65, 1000, 70001]) {
  const bytes = new Uint8Array(randomBytes(n));
  const want = createHash('sha256').update(bytes).digest('hex');
  assert.equal(sha256Js(bytes), want, `sha256 of ${n} bytes`);
  assert.equal(await sha256Hex(bytes, null), want, 'the fallback is used without crypto.subtle');
  assert.equal(await sha256Hex(bytes), want, 'and WebCrypto agrees');
}
checks.push('SHA-256 with and without WebCrypto');

// ── registers: convergence ─────────────────────────────────────────────────
let seed = 0x2545f491;
const rand = (n) => { seed ^= seed << 13; seed ^= seed >>> 17; seed ^= seed << 5; return (seed >>> 0) % n; };
const pick = (xs) => xs[rand(xs.length)];
const KEYS = [
  'var:0x1198:v1:name', 'var:0x1198:v1:type', 'var:0x1161:a0:name', 'fn:0x1149', 'fn:0x1161', 'proto:0x1161',
  'data:0x4010:type', 'data:0x4010:name', 'typedef:pair', 'comment:0x1198:0x11b5', 'byte:0x11af', 'byte:0x11b0',
  'raw:ana00000:1', 'rawf:0x1198:ben00000:2', 'setting:mode',
];
const VALUES = {
  name: ['total', 'count', 'sum', null], type: ['unsigned long', 'long', 'int', null], fn: ['adder', 'add_two', null],
  proto: ['long sum_to(int count)', 'int sum_to(int n)', null], typedef: ['typedef struct pair { int a; int b; } pair;', null],
  comment: ['calls add', 'hot path', null], byte: ['05', '90', null], raw: ['readonly 0x2000+8', null],
  rawf: ['flow 0x11b5 return', null], setting: MODES,
};
const valueFor = (key) => {
  const [kind, , , field] = key.split(':');
  if (kind === 'var') return pick(VALUES[field]);
  if (kind === 'data') return pick(key.endsWith(':name') ? VALUES.name : VALUES.type);
  return pick(VALUES[kind]);
};
function round(peers, n, lateJoiner) {
  const reps = peers.map((p) => new Replica(p));
  const log = [];
  for (let i = 0; i < n; i++) {
    const r = pick(reps);
    const key = pick(KEYS);
    const op = r.set(key, valueFor(key));
    assert.ok(validOp(op), `a page's own op passes the check (${JSON.stringify(op)})`);
    log.push(op);
    if (rand(4) === 0) for (const o of log.slice(-3)) pick(reps).apply(o);
  }
  for (const r of reps) {
    const order = [...log, ...log.slice(0, rand(log.length))];
    for (let i = order.length - 1; i > 0; i--) { const j = rand(i + 1); [order[i], order[j]] = [order[j], order[i]]; }
    for (const op of order) assert.notEqual(r.receive(JSON.parse(JSON.stringify(op))), 'invalid');
  }
  if (lateJoiner) {
    const late = new Replica('zed00000');
    for (const op of pick(reps).snapshot()) late.receive(op);
    reps.push(late);
  }
  const regs = JSON.stringify([...reps[0].regs].sort());
  for (const r of reps) assert.equal(JSON.stringify([...r.regs].sort()), regs, 'every page ends with the same registers');
  const sessions = reps.map((r) => {
    const s = new Session();
    applyRegisters(s, r, [...r.regs.keys()].reverse());
    return s;
  });
  const want = JSON.stringify(sessions[0].allAssertions((a) => a));
  for (const s of sessions) assert.equal(JSON.stringify(s.allAssertions((a) => a)), want, 'and sends the engine the same directives');
  return reps[0].value('setting:mode');
}
const modes = new Set();
for (let i = 0; i < 2000; i++) modes.add(round(['ana00000', 'ben00000', 'cy000000'].slice(0, 2 + rand(2)), 5 + rand(40), rand(2) === 0));
assert.ok(modes.size >= 3, 'the decompiler effort is one of the registers that converge');
checks.push('2000 random rounds over the full key set (incl. the mode) converge to one register map and one directive list');

// ── registers: what may not come from another page ─────────────────────────
{
  const c = (n = 7) => [n, 'eve00000'];
  const HOSTILE = [
    [{ k: 'raw:eve00000:1', v: '@/work/input.bin', c: c() }, 'a raw directive that is @FILE'],
    [{ k: 'raw:eve00000:2', v: '   @/work/input.bin', c: c() }, 'the same behind spaces'],
    [{ k: 'raw:eve00000:3', v: 'bytes 0x11af @/work/input.bin', c: c() }, 'a raw bytes directive whose payload is @FILE'],
    [{ k: 'rawf:0x1198:eve00000:4', v: 'bytes  0x11af\t@x', c: c() }, 'the same with odd spacing, bound to a function'],
    [{ k: 'comment:0x1198:0x11b5', v: 'fine\nbytes 0x401000 @/home/u/.ssh/id_rsa', c: c() }, 'a newline that would start a directive in an exported .kuna file'],
    [{ k: 'comment:0x1198:0x11b5', v: 'fine\u2028bytes 0x401000 @x', c: c() }, 'a line separator'],
    [{ k: 'comment:0x1198:0x11b5', v: 'fine #bytes 0x401000 @x', c: c() }, 'a " #" the .kuna reader would cut at'],
    [{ k: 'var:0x1198:v1:type', v: 'long @x', c: c() }, 'an @ in a type'],
    [{ k: 'proto:0x1161', v: 'int f(char *@x)', c: c() }, 'an @ in a signature'],
    [{ k: 'typedef:pair', v: 'typedef int @x;', c: c() }, 'an @ in a type definition'],
    [{ k: 'byte:0x11af', v: '9g', c: c() }, 'a byte that is not two hex digits'],
    [{ k: 'byte:0x11af', v: '@x', c: c() }, 'a byte that is a file'],
    [{ k: 'var:0x1198:v1:name', v: 'x y', c: c() }, 'a name that is not an identifier'],
    [{ k: 'var:0x1198:v1:name', v: 'a'.repeat(201), c: c() }, 'an overlong name'],
    [{ k: 'comment:0x1198:0x11b5', v: 'x'.repeat(4097), c: c() }, 'an overlong note'],
    [{ k: 'comment:0x1198:0x11b5', v: '', c: c() }, 'an empty value (a deletion is null)'],
    [{ k: 'flow:0x1198', v: 'return', c: c() }, 'a key of a kind the session does not have'],
    [{ k: 'var:0x1198:v1:name\n', v: 'x', c: c() }, 'a key with a newline'],
    [{ k: 'setting:mode', v: 'turbo', c: c() }, 'a mode that does not exist'],
    [{ k: 'setting:lang', v: 'rust', c: c() }, 'the output language (each person\'s own)'],
    [{ k: 'fn:0x1149', v: 'ok', c: [-1, 'eve00000'] }, 'a negative clock'],
    [{ k: 'fn:0x1149', v: 'ok', c: [2 ** 60, 'eve00000'] }, 'a clock past the safe integers'],
    [{ k: 'fn:0x1149', v: 'ok', c: [1, 'Eve!'] }, 'a malformed page id'],
    [{ k: 'fn:0x1149', v: 'ok', c: c(), extra: 1 }, 'an extra field'],
    [{ k: 'fn:0x1149', v: 7, c: c() }, 'a value that is not a string'],
    [['fn:0x1149', 'ok', c()], 'an array'],
    [null, 'null'],
  ];
  const victim = new Replica('ana00000');
  for (const [op, why] of HOSTILE) assert.equal(victim.receive(op), 'invalid', `rejected: ${why}`);
  assert.equal(victim.regs.size, 0, 'and nothing was applied');
  assert.equal(victim.receive({ k: 'comment:0x1198:0x11b5', v: 'see @notes', c: c() }), true, 'an @ inside a note is text, and allowed');
  assert.equal(victim.receive({ k: 'raw:eve00000:5', v: 'flow 0x11b5 return', c: c() }), true, 'a raw directive without a file form is allowed');
  const s = new Session();
  applyRegisters(s, victim, [...victim.regs.keys()]);
  assert.ok(s.allAssertions((a) => a).every((d) => !/(^|\s)@/.test(d.replace('see @notes', ''))), 'no directive the page sends can read a file');
  checks.push(`${HOSTILE.length} hostile ops rejected`);
}

// ── a session as registers and back, and the canonical order ───────────────
{
  const MAIN = '0x1198';
  const SUM = '0x1161';
  const build = (order) => {
    const s = new Session();
    const edits = {
      a: () => s.setVar(MAIN, 'v1', { name: 'total', type: 'unsigned long' }),
      b: () => s.setVar(SUM, 'v1', { name: 'i' }),
      c: () => s.setComment(MAIN, '0x11b5', 'calls add first'),
      d: () => s.setProto(SUM, 'long sum_to(int count)'),
      e: () => s.setData('0x4010', 'int', 'counter'),
      f: () => s.setTypedef('pair', 'typedef struct pair { int a; int b; } pair;'),
      g: () => s.setByte(0x11e1n, 0x90, 0xe8),
      h: () => s.setFunctionName(SUM, 'summation'),
      i: () => s.setRaw('raw:ana00000:1', 'readonly 0x2000+8'),
      j: () => s.setRaw('raw:ben00000:1', 'flow 0x11b5 return', MAIN),
      k: () => s.setByte(0x11e2n, 0x90, 0xe8),
    };
    for (const k of order) edits[k]();
    return s;
  };
  const one = build('abcdefghijk');
  const two = build('kjihgfedcba');
  const nameOf = (a) => one.functionName(a) || a;
  assert.deepEqual(two.allAssertions(nameOf), one.allAssertions(nameOf), 'the same edits in another order: the same directives, in the same order');
  assert.deepEqual(two.assertionsFor(MAIN), one.assertionsFor(MAIN));
  const regs = registersOf(one, 'ana00000');
  assert.equal(regs.get('var:0x1198:v1:type'), 'unsigned long');
  assert.equal(regs.get('byte:0x11e1'), '90');
  assert.equal(regs.get('rawf:0x1198:ben00000:1'), 'flow 0x11b5 return', 'a raw directive bound to a function keeps its function');
  const r = new Replica('ana00000');
  for (const [k, v] of regs) assert.ok(validOp(r.set(k, v)), `${k} is a register another page accepts`);
  const back = new Session();
  applyRegisters(back, r, [...r.regs.keys()]);
  assert.deepEqual(back.allAssertions(nameOf), one.allAssertions(nameOf), 'registers back into a session give the same directives');
  assert.deepEqual([...registersOf(back, 'ana00000')].sort(), [...regs].sort());
  assert.equal(recordKeyOf('var:0x1198:v1:name'), 'var:0x1198:v1');
  assert.equal(recordKeyOf('rawf:0x1198:ben00000:1'), 'raw:ben00000:1');

  const own = new Session();
  own.addRaw('readonly 0x2000+8');
  own.addRaw('flow f::0x10 return');
  assert.equal(adoptRawKeys(own, 'ana00000'), 2);
  assert.deepEqual([...own.records.keys()], ['raw:ana00000:1', 'raw:ana00000:2'], 'this page\'s raw directives take their shared names once');
  assert.equal(adoptRawKeys(own, 'ana00000'), 0);
  assert.equal(describeRegister('var:0x1198:v1:name', 'count', 'total'), 'renamed total to count');
  assert.equal(describeRegister('fn:0x1161', 'summation', null, () => 'sum_to'), 'renamed sum_to to summation');
  checks.push('session ⇄ registers round trip; canonical directive order; raw keys adopted');
}

// ── Undo in a live session ─────────────────────────────────────────────────
{
  const a = new Replica('ana00000');
  const b = new Replica('ben00000');
  const hist = new History();
  const write = (rep, key, value) => {
    const prev = rep.value(key);
    const prevClock = rep.clock(key);
    const op = rep.set(key, value);
    return { key, prev, prevClock, op };
  };
  const mine = write(a, 'comment:0x1198:0x11b5', 'calls add');
  hist.record([mine]);
  b.apply(mine.op);
  const theirs = b.set('comment:0x1198:0x11b5', 'hot path');
  a.apply(theirs);
  const refused = hist.undo(a);
  assert.deepEqual(refused.ops, [], 'undo will not erase what someone else wrote since');
  assert.deepEqual(refused.skipped, [{ key: 'comment:0x1198:0x11b5', by: 'ben00000' }], 'and says who');
  const patch = [write(a, 'byte:0x11af', '05'), write(a, 'byte:0x11b0', '90')];
  hist.record(patch);
  const undone = hist.undo(a);
  assert.equal(undone.ops.length, 2);
  for (const op of [...patch.map((p) => p.op), ...undone.ops]) b.apply(op);
  assert.equal(b.value('byte:0x11af'), null, 'my own undo reaches the others as an ordinary write');
  assert.ok(hist.canRedo);
  const redone = hist.redo(a);
  assert.equal(a.value('byte:0x11af'), '05', 'redo writes it again');
  assert.equal(redone.ops.length, 2);
  const failed = write(a, 'fn:0x1149', 'adder');
  hist.record([failed]);
  const depth = hist.undoStack.length;
  hist.record([write(a, 'fn:0x1149', null)]);
  assert.equal(hist.undoStack.length, depth - 1, 'a change that takes back the last edit (the engine refused it) removes that step');
  checks.push('undo skips registers someone changed since; redo; withdrawn edits');
}

// ── the wire: messages ─────────────────────────────────────────────────────
{
  const BUILD = 'a'.repeat(64);
  const file = { name: 'sample.elf', size: 16000, hash: 'b'.repeat(64) };
  const good = [
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ana00000', name: 'Ana', color: null, sid: null, file: null },
    { t: 'hello', proto: PROTOCOL + 1, peer: 'ana00000', name: 'Ana', anything: [1, 2] },
    { t: 'welcome', sid: 'abcdefabcdef', color: COLORS[1], roster: [{ peer: 'ana00000', name: 'Ana', color: COLORS[0] }], file, example: true, send: true },
    { t: 'snap', ops: [], last: true },
    { t: 'ops', ops: [{ k: 'fn:0x10', v: 'x', c: [1, 'ana00000'] }] },
    { t: 'file', ...file },
    { t: 'roster', members: [{ peer: 'ana00000', name: 'Ana Lopez', color: COLORS[0] }] },
    { t: 'where', fn: '0x1198', view: 'split' },
    { t: 'where', fn: null, view: 'c' },
    { t: 'ping', fn: '0x1198', view: 'c', anchor: 'c:6' },
    { t: 'cur', fn: '0x1198', view: 'c', anchor: 'c:6', fx: 0.5, fy: 0.25, col: 12.5 },
    { t: 'cur', off: true },
    { t: 'full' },
    { t: 'bye' },
  ];
  for (const m of good) assert.ok(validMessage(m), `valid: ${JSON.stringify(m)}`);
  const bad = [
    null, [], 'hello', { t: 'nope' }, { t: 'constructor' }, { t: 'toString' },
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ana00000', name: 'Ana', color: '#fff', sid: null, file: null },
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ANA!', name: 'Ana', color: null, sid: null, file: null },
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ana00000', name: 'A\nB', color: null, sid: null, file: null },
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ana00000', name: ' ', color: null, sid: null, file: null },
    { t: 'hello', proto: PROTOCOL, build: BUILD, peer: 'ana00000', name: 'Ana', color: null, sid: null, file: null, x: 1 },
    { ...good[2], file: { ...file, name: '../../etc/passwd' } },
    { ...good[2], file: { ...file, size: (64 << 20) + 1 } },
    { ...good[2], roster: new Array(MAX_PEERS).fill(good[2].roster[0]) },
    { t: 'where', fn: '0x1198', view: 'kitchen' },
    { t: 'ping', fn: '0x1198', view: 'c', anchor: 'c:0' },
    { t: 'ping', fn: '0x1198', view: 'c', anchor: '#c-L6' },
    { t: 'cur', fn: '0x1198', view: 'c', anchor: 'c:6', fx: 2, fy: 0 },
    { t: 'cur', fn: '0x1198', view: 'c', anchor: 'c:6', fx: 0, fy: NaN },
    { t: 'cur', off: true, fn: '0x1198' },
    { t: 'relay', to: 'ben00000', from: 'ana00000', kind: 'offer', id: 'abcdefghij', d: { u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: ['1.2.3.4 5 hu 1\r\na=x'] } },
    { t: 'ops', ops: [] },
  ];
  for (const m of bad) assert.equal(validMessage(m), false, `invalid: ${JSON.stringify(m)}`);
  assert.deepEqual(readMessage('{"t":"bye"}'), { t: 'bye' });
  assert.equal(readMessage('{"t":"bye","x":1}'), null, 'no unexpected fields');
  assert.equal(readMessage('not json'), null);
  assert.equal(readMessage(JSON.stringify({ t: 'cur', off: true }), 'edits'), null, 'a pointer on the edits channel is dropped');
  assert.equal(readMessage(JSON.stringify({ t: 'bye' }), 'cursor'), null, 'and anything else on the pointer channel');
  const huge = JSON.stringify({ t: 'where', fn: '0x1198', view: 'c', pad: 'x'.repeat(maxBytes('where')) });
  assert.equal(readMessage(huge), null, 'an oversized message is dropped before it is parsed');
  assert.equal(readMessage('x'.repeat(300 << 10)), null);
  assert.ok(validAnchor('s:-20') && validAnchor('b:0x11e1') && validAnchor('h:5') && !validAnchor('c:1234567'));
  checks.push(`${good.length} good and ${bad.length} bad messages; size caps; channels`);
}

// ── invite and reply codes, and the SDP inside them ────────────────────────
{
  const offer = 'v=0\r\no=- 2016926046532223732 2 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0\r\na=extmap-allow-mixed\r\n' +
    'a=msid-semantic: WMS\r\nm=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\nc=IN IP4 0.0.0.0\r\n' +
    'a=candidate:4283988699 1 udp 2113937151 b7ef728a-0f79-48a6-9945-0ebf55f39fb5.local 45594 typ host generation 0 network-cost 999\r\n' +
    'a=candidate:1 1 udp 1677729535 203.0.113.9 50000 typ srflx raddr 0.0.0.0 rport 0 generation 0\r\n' +
    'a=candidate:2 1 tcp 1518280447 192.168.1.4 9 typ host tcptype active generation 0\r\n' +
    'a=candidate:3 1 udp 2113939711 2001:db8::1 40000 typ host generation 0\r\n' +
    'a=ice-ufrag:sHHk\r\na=ice-pwd:xgLjiz9ZQidX4s5NkAp4RO8F\r\na=ice-options:trickle\r\n' +
    'a=fingerprint:sha-256 AE:0A:F0:5F:5F:D1:92:12:0A:52:1F:B2:FE:B9:C4:29:E1:6A:60:86:FC:B1:10:EF:91:96:05:F0:56:E5:78:62\r\n' +
    'a=setup:actpass\r\na=mid:0\r\na=sctp-port:5000\r\na=max-message-size:262144\r\n';
  const d = compactSdp(offer);
  assert.deepEqual(d.c, ['b7ef728a-0f79-48a6-9945-0ebf55f39fb5.local 45594 hu 2113937151', '203.0.113.9 50000 su 1677729535', '2001:db8::1 40000 hu 2113939711'],
    'the candidates a peer can use (an active TCP candidate cannot be reached)');
  const back = expandSdp(d, 'answer');
  assert.deepEqual(compactSdp(back), d, 'rebuilt, the description holds the same fields');
  assert.match(back, /a=setup:passive\r\n/, 'an answer is rebuilt passive');
  assert.match(expandSdp(d, 'offer'), /a=setup:actpass\r\n/);
  assert.match(passiveAnswer(offer.replace('actpass', 'active')), /a=setup:passive\r\n/);
  assert.equal(expandSdp({ ...d, m: 'data' }, 'offer').match(/a=mid:(\S+)/)[1], 'data', 'a non-zero mid travels');
  for (const evil of [{ ...d, u: 'ab\r\na=x' }, { ...d, c: ['1.2.3.4 5 hu 1 generation 0\r\na=x'] }, { ...d, f: 'zz' }, { ...d, extra: 1 }, { ...d, c: new Array(17).fill(d.c[0]) }]) {
    assert.equal(validSdp(evil), false, `refused: ${JSON.stringify(evil).slice(0, 60)}`);
    assert.throws(() => expandSdp(evil, 'offer'));
  }
  assert.equal(compactSdp('v=0\r\n'), null);

  const invite = await encodeCode('invite', { id: 'abcdefghij', n: 'Ana', f: 'crackme.elf', z: 14080, d, b: 1 });
  assert.ok(invite.length < 400, `an invite fits in a short link (${invite.length} characters)`);
  const got = await decodeCode(invite, 'invite');
  assert.equal(got.ok, true);
  assert.equal(got.n, 'Ana');
  assert.deepEqual(got.d, d);
  assert.equal((await decodeCode(invite, 'reply')).ok, false, 'an invite is not a reply');
  const reply = await encodeCode('reply', { id: 'abcdefghij', n: 'Ben', d });
  assert.equal((await decodeCode(reply, 'reply')).n, 'Ben');
  assert.equal(codeFrom(`https://kuna.noelo.org/decompile2/#join=${invite}`, 'invite'), invite);
  assert.equal(codeFrom(`  https://x/decompile2/#reply=${reply}\n`, 'reply'), reply);
  assert.equal(codeFrom('hello there', 'reply'), null);
  await assert.rejects(() => encodeCode('invite', { id: 'abcdefghij', n: 'Ana\n', f: 'x', z: 1, d }), 'the page never makes a bad code');
  const enc = async (obj) => Buffer.from(new Uint8Array(await new Response(new Blob([JSON.stringify(obj)]).stream()
    .pipeThrough(new CompressionStream('deflate-raw'))).arrayBuffer())).toString('base64url');
  assert.deepEqual(await decodeCode(await enc({ v: PROTOCOL + 1, k: 'i' }), 'invite'), { ok: false, why: 'version' }, 'a code from another version says so');
  assert.equal((await decodeCode(await enc({ v: PROTOCOL, k: 'i', id: 'abcdefghij', n: 'Ana', f: 'x', z: 1, d, evil: 1 }), 'invite')).ok, false);
  assert.equal((await decodeCode(await enc({ v: PROTOCOL, k: 'i', id: 'abcdefghij', n: 'Ana', f: 'a/b', z: 1, d }), 'invite')).ok, false, 'no paths in file names');
  assert.equal((await decodeCode('!!!!', 'invite')).ok, false);
  assert.equal((await decodeCode('A'.repeat(5000), 'invite')).ok, false, 'an oversized code is refused');
  assert.equal((await decodeCode(await enc('x'.repeat(9000)), 'invite')).ok, false, 'so is a code that inflates past the cap');
  checks.push(`SDP cut down and rebuilt (invite link ${invite.length} characters); codes round-trip and refuse the rest`);
}

// ── small helpers ──────────────────────────────────────────────────────────
{
  let t = 0;
  const lim = limiter(2, 2, () => t);
  assert.deepEqual([lim.take(), lim.take(), lim.take()], [true, true, false], 'a burst, then nothing');
  t += 500;
  assert.equal(lim.take(), true, 'refills at its rate');
  assert.equal(lim.take(), false);
  assert.match(randomId(8), /^[a-z0-9]{8}$/);
  assert.equal(new Set(Array.from({ length: 200 }, () => randomId(8))).size, 200);
  assert.equal(initials('Ana Lopez'), 'AL');
  assert.equal(initials('ben'), 'B');
  assert.equal(initials('  '), '?');
  const colored = resolveColors([{ peer: 'b', color: COLORS[0] }, { peer: 'a', color: COLORS[0] }, { peer: 'c', color: COLORS[1] }]);
  assert.deepEqual(colored.map((m) => m.color), [COLORS[2], COLORS[0], COLORS[1]], 'of two with one colour, the larger id takes a free one');
  checks.push('rate limiter, ids, initials, colours');
}

// ── whole groups over in-memory links ──────────────────────────────────────
const BUILD = 'c'.repeat(64);
const FILE_BYTES = new Uint8Array(randomBytes(200000));
const FILE = { name: 'sample.elf', size: FILE_BYTES.length, hash: createHash('sha256').update(FILE_BYTES).digest('hex') };

function linkPair() {
  const make = () => ({
    onmessage: null, onclose: null, closed: false, queue: [],
    send(text) { this.deliver(text, 'edits'); },
    sendCursor(text) { this.deliver(text, 'cursor'); },
    sendBinary(bytes) { this.deliver(bytes.slice().buffer, 'edits'); },
    buffered() { return 0; },
    drain() { return Promise.resolve(); },
    deliver(data, channel) {
      if (this.closed) return;
      const to = this.other;
      setImmediate(() => { if (!to.closed) to.onmessage?.(data, channel); });
    },
    close() {
      if (this.closed) return;
      this.closed = true;
      this.onclose?.();
      setImmediate(() => this.other.close());
    },
  });
  const a = make();
  const b = make();
  a.other = b;
  b.other = a;
  return [a, b];
}

const FAKE_SDP = { u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: [] };
const blocked = new Set();
const offers = new Map();
function connector(me) {
  return {
    async offer() {
      const id = randomId(10);
      let settle;
      const ready = new Promise((done) => { settle = done; });
      offers.set(id, { from: me, settle });
      return { id, sdp: FAKE_SDP, bc: false, ready, answer: async () => {}, cancel() { offers.delete(id); } };
    },
    async answer({ id }) {
      const off = offers.get(id);
      if (!off) throw new Error('used');
      offers.delete(id);
      if (blocked.has([off.from, me].sort().join('-'))) throw new Error('failed');
      const [a, b] = linkPair();
      off.settle(a);
      return { sdp: null, ready: Promise.resolve(b), cancel() {} };
    },
  };
}

function page(name, { file = null, bytes = null } = {}) {
  return {
    name, file, bytes, events: [], pings: [], cursors: [], welcomes: [], progress: 0,
    fileMeta() { return this.file; },
    fileBytes() { return this.bytes; },
    isExample() { return false; },
    welcomed(info) { this.welcomes.push(info); },
    caughtUp() { this.caught = true; },
    changed() {},
    fileProgress(got) { this.progress = got; },
    fileArrived(bytes, meta) { this.arrived = { bytes, meta }; this.file = meta; this.bytes = bytes; },
    fileFailed(why) { this.failed = why; },
    roster() {},
    where() {},
    cursor(peer, m) { this.cursors.push([peer, m]); },
    ping(peer, m) { this.pings.push([peer, m]); },
    event(kind, info) { this.events.push([kind, info.name]); },
  };
}

function member(peer, name, opts = {}) {
  const replica = new Replica(peer);
  const p = page(name, opts);
  const g = new Group({ me: peer, name, build: opts.build || BUILD, replica, connect: connector(peer), page: p });
  return { g, p, replica, peer };
}

/** `guest` opens `host`'s invite: one link, the guest's end marked joining. */
function invite(host, guest) {
  const [a, b] = linkPair();
  host.g.addLink(a);
  guest.g.addLink(b, { joining: true });
  return [a, b];
}

const settle = async (ms = 60) => { for (let i = 0; i < 6; i++) await sleep(ms / 6); };
const linked = (x, y) => x.g.peers.get(y.peer)?.member === true && y.g.peers.get(x.peer)?.member === true;
const same = (...ms) => ms.every((m) => JSON.stringify([...m.replica.regs].sort()) === JSON.stringify([...ms[0].replica.regs].sort()));

{
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  ana.replica.set('fn:0x1149', 'adder');
  ana.replica.set('setting:mode', 'fast');
  const ben = member('bbbbbbbb', 'Ben');
  invite(ana, ben);
  await settle(200);
  assert.ok(linked(ana, ben), 'a guest who opens the invite joins');
  assert.equal(ben.g.sid, ana.g.sid);
  assert.equal(ben.p.welcomes[0].file.hash, FILE.hash);
  assert.equal(ben.p.welcomes[0].send, true, 'a guest without the program is sent it');
  assert.ok(ben.p.arrived, 'and receives it');
  assert.deepEqual(ben.p.arrived.bytes, FILE_BYTES, 'intact (SHA-256 checked)');
  assert.ok(ben.p.caught, 'the guest is told when it has caught up');
  assert.equal(ben.replica.value('setting:mode'), 'fast', 'with the session\'s decompiler effort');
  assert.ok(same(ana, ben));
  assert.notEqual(ben.g.color, ana.g.color, 'the newcomer gets a colour of its own');
  assert.deepEqual(ana.p.events, [['joined', 'Ben']]);
  checks.push('invite: hello, welcome, snapshot, the program sent and checked');

  const cy = member('cccccccc', 'Cy');
  invite(ben, cy);
  await settle(300);
  assert.ok(linked(ben, cy), 'a third joins through the second');
  assert.ok(linked(ana, cy), 'and the group introduces them to the first, with no second invite');
  assert.equal(ana.g.size(), 3);
  assert.equal(new Set([ana, ben, cy].map((m) => m.g.members().find((x) => x.me).color)).size, 3, 'three colours');
  ana.g.local([ana.replica.set('var:0x1198:v1:name', 'total')]);
  cy.g.local([cy.replica.set('var:0x1198:v1:type', 'unsigned long')]);
  ben.g.local([ben.replica.set('fn:0x1149', 'add_two')]);
  await settle(200);
  assert.ok(same(ana, ben, cy), 'concurrent edits converge');
  assert.equal(cy.replica.value('var:0x1198:v1:name'), 'total');
  assert.equal(ana.replica.value('var:0x1198:v1:type'), 'unsigned long', 'a rename and a retype of one local both survive');
  checks.push('three pages: relayed introduction to a full mesh; concurrent edits converge');

  const dee = member('dddddddd', 'Dee', { file: FILE, bytes: FILE_BYTES });
  dee.replica.set('comment:0x1198:0x11b5', 'mine from before');
  invite(cy, dee);
  await settle(300);
  assert.equal(dee.p.welcomes[0].send, false, 'a guest that already has the program is not sent it again');
  assert.equal(dee.p.arrived, undefined);
  dee.g.local(dee.replica.snapshot());
  await settle(200);
  assert.ok(same(ana, ben, cy, dee), 'what a newcomer brings merges everywhere');
  assert.equal(ana.replica.value('comment:0x1198:0x11b5'), 'mine from before');
  assert.ok([ana, ben, cy].every((m) => linked(m, dee)));

  const sent = [];
  ana.g.sendPing({ fn: '0x1198', view: 'c', anchor: 'c:6' });
  ana.g.sendPing({ fn: '0x1198', view: 'c', anchor: 'c:7' });
  ana.g.sendPing({ fn: '0x1198', view: 'c', anchor: 'c:8' });
  ana.g.sendCursor({ fn: '0x1198', view: 'c', anchor: 'c:6', fx: 0.5, fy: 0.5 });
  ana.g.setWhere({ fn: '0x1161', view: 'asm' });
  await settle(120);
  assert.equal(ben.p.pings.length, 2, 'pings past one a second (burst two) are dropped');
  assert.deepEqual(ben.p.cursors.map(([, m]) => m.anchor), ['c:6']);
  assert.deepEqual(ben.g.who('aaaaaaaa').where, { fn: '0x1161', view: 'asm' }, 'where each person is');
  sent.push('ok');

  const ben2 = ben.g.peers.get('aaaaaaaa');
  const before = ben2.dropped;
  const raw = ana.g.peers.get('bbbbbbbb').link;
  for (const junk of ['nope', '{"t":"ops","ops":[{"k":"raw:eve00000:1","v":"@/etc/passwd","c":[99,"eve00000"]}]}',
    JSON.stringify({ t: 'where', fn: 'main', view: 'c' }), 'x'.repeat(300 << 10), JSON.stringify({ t: 'relay', to: 'cccccccc', from: 'eeeeeeee', kind: 'offer', id: 'abcdefghij', d: FAKE_SDP })]) {
    raw.send(junk);
  }
  raw.sendBinary(new Uint8Array(10));
  await settle(120);
  assert.ok(ben2.dropped >= before + 5, `malformed, hostile and unexpected messages are dropped (${ben2.dropped - before})`);
  assert.equal(ben.replica.value('raw:eve00000:1'), null, 'including the op that would read a file');
  ana.g.local([ana.replica.set('fn:0x1161', 'summation')]);
  await settle(150);
  assert.equal(ben.replica.value('fn:0x1161'), 'summation', 'and the link still works afterwards');
  checks.push('pings and pointers rate-limited; junk dropped without breaking the link');

  const late = [];
  for (let i = 0; i < 4; i++) {
    const m = member(`e${i}eeeeee`, `Guest ${i}`);
    invite(ana, m);
    late.push(m);
    await settle(250);
  }
  assert.equal(ana.g.size(), MAX_PEERS, 'eight people');
  assert.ok(late.every((m) => m.g.size() === MAX_PEERS), 'and every one of them sees all eight');
  const ninth = member('ffffffff', 'Nine');
  invite(ben, ninth);
  await settle(200);
  assert.deepEqual(ninth.p.events, [['full', 'Ben']], 'the ninth is told the session is full');
  assert.equal(ninth.g.sid, null);
  checks.push('eight people in a mesh; the ninth is turned away');

  dee.g.leave();
  await settle(400);
  assert.ok([ana, ben, cy].every((m) => m.p.events.some(([k, n]) => k === 'left' && n === 'Dee')), 'a person who leaves is gone for everyone');
  assert.equal(ana.g.size(), MAX_PEERS - 1);
  const dee2 = member('d2dddddd', 'Dee');
  invite(cy, dee2);
  await settle(300);
  assert.equal(ana.g.size(), MAX_PEERS, 'and can join again');
  assert.ok(same(ana, dee2));
  for (const m of [ana, ben, cy, dee2, ...late]) m.g.leave();
}

{
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const other = member('bbbbbbbb', 'Ben', { build: 'd'.repeat(64) });
  invite(ana, other);
  await settle(120);
  assert.deepEqual(ana.p.events, [['mismatch', 'Ben']], 'a page of another build is refused');
  assert.deepEqual(other.p.events.filter(([k]) => k === 'mismatch'), [['mismatch', 'Ana']], 'and says whose page differs');
  assert.equal(ana.g.size(), 1);

  blocked.add('aaaaaaaa-cccccccc');
  const ben = member('bbbbbbbb', 'Ben');
  const cy = member('cccccccc', 'Cy');
  invite(ana, ben);
  await settle(200);
  invite(ben, cy);
  await settle(300);
  assert.ok(!linked(ana, cy), 'two pages that cannot link directly');
  ana.g.local([ana.replica.set('fn:0x1149', 'adder')]);
  cy.g.local([cy.replica.set('byte:0x11af', '90')]);
  await settle(200);
  assert.ok(same(ana, ben, cy), 'still converge through the page both are linked to');
  checks.push('another build refused; a pair that cannot link converges through a third');
  blocked.clear();
  for (const m of [ana, ben, cy, other]) m.g.leave();
}

console.log(`DECOMPILE2 COLLAB OK — ${checks.join('; ')}`);
process.exit(0);
