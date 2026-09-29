// sync.js — one page's side of a live session, with no DOM: it keeps the
// page's Session and the group's registers in step, and runs the data half of
// joining and leaving (collab.js is the dialogs, the roster and the pointers;
// group.js the protocol).
//
// A page sends only what its student changed. A change is found against a
// base, the Session's registers as they were when the Session last matched
// the replica, never against the replica itself, which runs ahead of the
// Session while the others' changes wait to be applied (a frame at a time).
// Applying those changes moves the base with them, so nothing another page
// wrote is ever sent back as this page's edit.
//
// Joining: until the inviter's registers have all arrived, this page's
// changes stay its own. Then a joiner that has the program open brings what
// it had: a field the registers already hold (a value or a deletion) keeps
// the session's, and one they do not is written with the oldest clock there
// is, so it still loses to any write another page makes of it that this page
// had not heard of yet; what the student changed while joining is newest. A
// joiner at another decompiler effort than the session's brings no variable
// change (the engine numbers variables per effort). A page that left a session and
// joins the same one again sends what it changed since it left. A joiner that receives the program opens it with
// the session's changes; if it had its own stored changes to that program,
// and the session does not already hold all of them, the session is saved
// apart from them (`slot` 'shared') and they come back when it leaves.
import { Session } from '../session.js';
import {
  Replica, History, applyRegisters, adoptRawKeys, registersOf, recordKeyOf, changedBetween, localChanges, recordIndex,
  birthOrder, describeRegister, newer, oldestClock,
} from './replica.js';
import { Group, MAX_REGISTERS, TIMERS } from './group.js';

const APPLY_MS = 16;
const STALL_MS = 20000;

/** Does a change to register `key` change the directives of the function at `fn`? */
function touchesFunction(key, fn) {
  const p = key.split(':');
  if (p[0] === 'var' || p[0] === 'comment' || p[0] === 'rawf') return p[1] === fn;
  return p[0] !== 'setting';
}

/** Does `session` already hold everything `own` holds (so keeping `own` apart would keep nothing)? */
function holdsAll(session, own) {
  const regs = registersOf(session);
  for (const [key, value] of registersOf(own)) if (regs.get(key) !== value) return false;
  const raw = (s) => [...s.records.values()].filter((r) => r.kind === 'raw').map((r) => `${r.func || ''}\n${r.text}`);
  const texts = new Set(raw(session));
  return raw(own).every((t) => texts.has(t));
}

/** Clear every variable field of `session` the registers do not hold (a joiner's, made at another decompiler effort). */
function dropUnheldVars(session, replica) {
  for (const rec of [...session.records.values()]) {
    if (rec.kind !== 'var') continue;
    const id = `var:${rec.func}:${rec.sym}`;
    const name = rec.name && !replica.regs.has(`${id}:name`) ? null : undefined;
    const type = rec.type && !replica.regs.has(`${id}:type`) ? null : undefined;
    if (name === null || type === null) session.setVar(rec.func, rec.sym, { name, type });
  }
}

const setBase = (base, key, value) => (value === null || value === undefined ? base.delete(key) : base.set(key, value));

export class Sync {
  /**
   * `app`: the page (session(), binary(), fileMeta(), mode(), target(),
   * nameOf(addr), toast(), remoteChanged(), clearUndo(), shareStarted(),
   * refresh(), endShared(), ownSession(hash, bytes, name), openShared()). `ui`: what
   * collab.js shows (nameOf(peer), remember, welcomed, fileProgress,
   * fileOpening, joined, joinFailed, mergeReplaced, roster, where, cursor,
   * ping, event); every hook is optional.
   */
  constructor({ me, app, ui = {}, timers = TIMERS, hash = undefined, groupOptions = {} }) {
    this.me = me;
    this.app = app;
    this.ui = ui;
    this.timers = timers;
    this.hash = hash;
    this.groupOptions = groupOptions;
    this.group = null;
    this.replica = null;
    this.base = null;
    this.phase = 'solo';
    this.history = new History();
    this.pending = [];
    this.pendingTimer = 0;
    this.applying = false;
    this.join = null;
    this.kept = null;
    this.slot = 'own';
    this.aside = null;
    this.refusedShown = new Set();
    this.index = null;
    this.order = null;
    this.runs = null;
    this.orderOf = (recordKey) => this.#order()(recordKey);
  }

  get shared() { return this.phase === 'shared'; }

  get joining() { return this.phase === 'joining'; }

  get active() { return !!this.group?.active; }

  get canUndo() { return this.shared && this.history.canUndo; }

  get canRedo() { return this.shared && this.history.canRedo; }

  #index() {
    if (!this.index) this.index = this.replica ? recordIndex(this.replica) : new Map();
    return this.index;
  }

  #order() {
    if (!this.order) this.order = birthOrder(this.#index());
    return this.order;
  }

  #dirty() {
    this.index = null;
    this.order = null;
    this.runs = null;
  }

  /** The newest write to a record (`[counter, peer]`: who changed it last), or null. */
  clockOf(recordKey) {
    if (!this.shared) return null;
    if (recordKey.startsWith('bytes:')) return this.#runClock(recordKey);
    return this.#index().get(recordKey)?.clock || null;
  }

  #runClock(key) {
    if (!this.runs) {
      this.runs = new Map();
      for (const run of this.app.session().byteRuns()) {
        let best = null;
        for (let i = 0; i < run.values.length; i++) {
          const c = this.replica.clock(`byte:0x${(run.addr + BigInt(i)).toString(16)}`);
          if (c && (!best || newer(c, best))) best = c;
        }
        this.runs.set(`bytes:0x${run.addr.toString(16)}`, best);
      }
    }
    return this.runs.get(key) || null;
  }

  #newGroup({ build, name, connect }) {
    this.group?.leave();
    let group = null;
    const live = (fn) => (...args) => (this.group === group ? fn(...args) : undefined);
    group = new Group({
      me: this.me, name, build, replica: this.replica, connect, timers: this.timers, hash: this.hash, ...this.groupOptions,
      page: {
        fileMeta: () => this.app.fileMeta(),
        fileBytes: () => this.app.binary()?.bytes || null,
        isExample: () => !!this.app.binary()?.example,
        welcomed: live((info) => this.#welcomed(info)),
        caughtUp: live(() => this.#caughtUp()),
        changed: live((changes) => this.#remote(changes)),
        fileProgress: live((got, size) => {
          this.#stall();
          this.ui.fileProgress?.(got, size);
        }),
        fileArrived: live((bytes, meta) => this.#fileArrived(bytes, meta)),
        fileFailed: live((why) => this.#failJoin(why === 'hash' ? 'hash' : 'lost')),
        roster: live(() => this.ui.roster?.()),
        where: live((peer) => this.ui.where?.(peer)),
        cursor: live((peer, m) => this.ui.cursor?.(peer, m)),
        ping: live((peer, m) => this.ui.ping?.(peer, m)),
        event: live((kind, info) => this.#event(kind, info)),
      },
    });
    this.group = group;
    return group;
  }

  // ── starting, joining, leaving ───────────────────────────────────────────

  /** Start a session as its first member: this page's changes become its registers. */
  start({ build, name, connect }) {
    this.leave();
    this.kept = null;
    this.replica = new Replica(this.me);
    this.base = new Map();
    this.#newGroup({ build, name, connect });
    this.group.create();
    this.slot = 'own';
    this.aside = null;
    this.#share();
    this.local({ record: false });
    this.group.local([this.replica.set('setting:mode', this.app.mode())]);
  }

  /** The link an invite (`invite`, its id) made reached this page's session. */
  addLink(link, invite = null) {
    if (this.active) this.group.addLink(link, { invite });
    else link.close();
  }

  /** Join through `link` (the connection an invite made); what the Session holds now is what this page brings. */
  beginJoin({ build, name, connect, link }) {
    this.leave();
    this.#newGroup({ build, name, connect });
    this.phase = 'joining';
    const session = this.app.session();
    adoptRawKeys(session, this.me);
    this.join = { sponsor: null, file: null, send: false, have: false, session, anchor: registersOf(session), timer: 0 };
    this.group.addLink(link, { joining: true });
  }

  /** Stop joining, or leave: see `leave`. */
  stop() {
    this.leave();
  }

  /**
   * Leave the session (or stop joining). The page keeps what it shows, except
   * that a session kept apart gives way to the student's own changes again
   * (app.endShared), also when a join stops after the program it received was
   * opened with the session's changes. A page that keeps the session's changes
   * remembers where it left, so joining the same session again sends only
   * what changed since.
   */
  leave() {
    const was = this.phase;
    const sid = this.group?.sid || null;
    this.group?.leave();
    this.group = null;
    this.timers.clearTimeout(this.pendingTimer);
    this.pendingTimer = 0;
    this.pending = [];
    const opened = was === 'joining' && !!this.join?.opened;
    if (this.join) this.timers.clearTimeout(this.join.timer);
    this.join = null;
    const session = this.app.session();
    if (was === 'shared' || opened) session.reorder();
    const { replica, base } = this;
    this.replica = null;
    this.base = null;
    this.phase = 'solo';
    this.history.clear();
    this.aside = null;
    this.#dirty();
    if (opened) {
      this.app.endShared();
      this.slot = 'own';
    }
    if (was !== 'shared') return was;
    const meta = this.app.fileMeta();
    this.app.clearUndo();
    this.app.endShared();
    this.slot = 'own';
    if (sid && meta && base && this.app.session() === session) this.kept = { sid, hash: meta.hash, session, replica, base };
    return was;
  }

  #share() {
    this.phase = 'shared';
    this.refusedShown.clear();
    this.history.clear();
    this.#dirty();
    this.app.clearUndo();
    this.app.session().orderOf = this.orderOf;
    this.app.shareStarted?.();
    this.app.refresh?.();
  }

  /** Restart the join's stall timer: a join that hears nothing for STALL_MS fails. */
  #stall() {
    const j = this.join;
    if (!j || this.phase !== 'joining' || j.opening) return;
    this.timers.clearTimeout(j.timer);
    j.timer = this.timers.setTimeout(() => {
      if (this.join === j && this.phase === 'joining') this.#failJoin('stalled');
    }, STALL_MS);
  }

  #welcomed({ from, name, file, example, send, sid }) {
    const j = this.join;
    if (!j || this.phase !== 'joining') return;
    const session = this.app.session();
    Object.assign(j, { sponsor: from, sponsorName: name, file, send, have: this.app.fileMeta()?.hash === file.hash });
    const k = this.kept;
    if (j.have && k && k.sid === sid && k.hash === file.hash && k.session === session) {
      this.replica = k.replica;
      this.base = k.base;
    } else {
      this.replica = new Replica(this.me);
      this.base = null;
    }
    this.group.replica = this.replica;
    this.#dirty();
    this.#stall();
    this.ui.remember?.(from);
    this.ui.welcomed?.({ from, name, file, example, send });
  }

  #caughtUp() {
    const j = this.join;
    if (!j || this.phase !== 'joining') return;
    this.#stall();
    if (j.have && this.app.fileMeta()?.hash === j.file.hash) this.#merge();
    else if (!j.send) this.#failJoin('program');
  }

  /**
   * A joiner with the program open has all the inviter's registers: send what
   * this page brings (see the file header), then the Session takes the registers.
   */
  #merge() {
    const j = this.join;
    const session = this.app.session();
    adoptRawKeys(session, this.me);
    const now = registersOf(session);
    const mode = this.#lateMode();
    const older = [];
    let want;
    if (this.base) want = changedBetween(this.base, now);
    else {
      const anchor = session === j.session ? j.anchor : now;
      want = changedBetween(anchor, now);
      for (const [key, value] of anchor) if (!want.has(key)) older.push([key, value]);
    }
    const apart = mode ? this.#effortApart(want, older) : new Set();
    const mine = older.filter(([key]) => !(mode && key.startsWith('var:')));
    const replaced = mine.filter(([key, value]) => this.replica.regs.has(key) && this.replica.value(key) !== value).length;
    const ops = [];
    const unheld = new Map(mine.filter(([key]) => !this.replica.regs.has(key)));
    const { changes, refused: earlier } = localChanges(unheld, this.replica, { maxLive: MAX_REGISTERS });
    for (const { key, value } of changes) {
      const op = { k: key, v: value, c: oldestClock(this.me), b: oldestClock(this.me) };
      if (this.replica.apply(op)) ops.push(op);
    }
    const copy = replaced || apart.size ? Session.fromJSON(JSON.parse(JSON.stringify(session.toJSON()))) : null;
    const { written, refused } = this.#writeAll(want);
    for (const w of written) ops.push(w.op);
    this.kept = null;
    this.#share();
    this.applying = true;
    try {
      applyRegisters(session, this.replica, [...this.replica.regs.keys()]);
      if (mode) dropUnheldVars(session, this.replica);
    } finally {
      this.applying = false;
    }
    this.base = registersOf(session);
    for (const r of [...earlier, ...refused]) setBase(this.base, r.key, this.replica.value(r.key));
    if (ops.length) this.group.local(ops);
    this.#tellRefused([...earlier, ...refused]);
    this.#joined();
    if (copy) this.ui.mergeReplaced?.({ count: replaced, apart: apart.size, mode: this.replica.value('setting:mode'), copy });
    this.app.remoteChanged({ inspect: true, mode, label: '' });
  }

  /**
   * A joiner at another decompiler effort than the session's brings no
   * variable change (the engine numbers variables per effort, so the same
   * `v1` is another variable there): take them out of `want`, and return the
   * variables whose fields differ from the session's.
   */
  #effortApart(want, older) {
    const apart = new Set();
    for (const [key, value] of [...older, ...want]) {
      if (key.startsWith('var:') && this.replica.value(key) !== value) apart.add(key.split(':').slice(0, 3).join(':'));
    }
    for (const key of [...want.keys()]) if (key.startsWith('var:')) want.delete(key);
    return apart;
  }

  /** The session's decompiler effort, when this page's is another. */
  #lateMode() {
    const mode = this.replica.value('setting:mode');
    return mode && mode !== this.app.mode() ? mode : null;
  }

  async #fileArrived(bytes, meta) {
    const j = this.join;
    if (!j || this.phase !== 'joining') return;
    j.opening = true;
    this.timers.clearTimeout(j.timer);
    this.ui.fileOpening?.(meta);
    const hash = `sha256:${meta.hash}`;
    const session = new Session();
    applyRegisters(session, this.replica, [...this.replica.regs.keys()]);
    session.orderOf = this.orderOf;
    const own = this.app.ownSession(hash, bytes, meta.name);
    const apart = !!own && !holdsAll(session, own);
    this.slot = apart ? 'shared' : 'own';
    const open = this.group.who(j.sponsor)?.where?.fn || null;
    const opened = registersOf(session);
    let ok = false;
    try {
      j.opened = true;
      ok = await this.app.openShared({
        name: meta.name, bytes, hash, session, mode: this.replica.value('setting:mode'), open, slot: this.slot,
        still: () => this.join === j && this.phase === 'joining',
      });
    } catch (_) {
      ok = false;
    }
    if (this.join !== j || this.phase !== 'joining') return;
    if (!ok) {
      this.#failJoin('open', { name: meta.name });
      return;
    }
    const s = this.app.session();
    adoptRawKeys(s, this.me);
    const mode = this.#lateMode();
    const want = changedBetween(opened, registersOf(s));
    const late = mode ? this.#effortApart(want, []) : new Set();
    const copy = late.size ? Session.fromJSON(JSON.parse(JSON.stringify(s.toJSON()))) : null;
    const { written, refused } = this.#writeAll(want);
    this.kept = null;
    this.#share();
    this.aside = apart ? { count: own.size, hash, name: meta.name } : null;
    const before = JSON.stringify(s.toJSON());
    applyRegisters(s, this.replica, [...this.replica.regs.keys()]);
    if (mode) dropUnheldVars(s, this.replica);
    this.base = registersOf(s);
    for (const r of refused) setBase(this.base, r.key, this.replica.value(r.key));
    if (written.length) this.group.local(written.map((w) => w.op));
    this.#tellRefused(refused);
    const moved = JSON.stringify(s.toJSON()) !== before;
    this.#joined();
    if (copy) this.ui.mergeReplaced?.({ count: 0, apart: late.size, mode: this.replica.value('setting:mode'), copy });
    this.app.remoteChanged({ inspect: moved, mode, label: '' });
  }

  #joined() {
    const j = this.join;
    this.timers.clearTimeout(j.timer);
    this.join = null;
    this.ui.joined?.({ sponsor: j.sponsor, sponsorName: j.sponsorName, aside: this.aside });
  }

  #failJoin(reason, info = {}) {
    if (this.phase !== 'joining') return;
    this.leave();
    this.ui.joinFailed?.(reason, info);
  }

  #event(kind, info) {
    if (this.phase === 'joining') {
      if (kind === 'full' || kind === 'mismatch' || kind === 'closed') {
        this.#failJoin(kind, info);
        return;
      }
      if ((kind === 'lost' || kind === 'left') && info?.peer && info.peer === this.join?.sponsor) {
        this.#failJoin('sponsor', info);
        return;
      }
    }
    this.ui.event?.(kind, info);
  }

  // ── this page's edits ────────────────────────────────────────────────────

  /** Write `want` (`key → value`) to the registers: `{written: [{key, prev, op}], refused}`. */
  #writeAll(want) {
    const { changes, refused } = localChanges(want, this.replica, { maxLive: MAX_REGISTERS });
    const written = changes.map(({ key, value }) => {
      const prev = this.replica.value(key);
      return { key, prev, op: this.replica.set(key, value) };
    });
    if (written.length) this.#dirty();
    return { written, refused };
  }

  /** Say (once per value) that changes the registers refused stay on this page. */
  #tellRefused(refused) {
    const fresh = refused.filter((r) => !this.refusedShown.has(`${r.key}\n${r.value}`));
    if (fresh.length) {
      for (const r of fresh) this.refusedShown.add(`${r.key}\n${r.value}`);
      const full = fresh.some((r) => r.why === 'full');
      this.app.toast?.(full ? 'This session holds as many changes as it can, so this one stays on this page.' : 'One of your changes stays on this page only.', {
        kind: 'warn', detail: full ? `A session holds at most ${MAX_REGISTERS.toLocaleString()} changes.`
          : 'It reads a file or holds text the others\' pages do not accept, so it is not shared.',
      });
    }
  }

  /**
   * The page's Session changed (an edit, an undo, a byte burst): send what
   * the student changed, except bytes still being typed (their burst goes out
   * as one edit when it ends).
   */
  local({ record = true } = {}) {
    if (this.phase !== 'shared' || this.applying || !this.replica) return;
    const session = this.app.session();
    adoptRawKeys(session, this.me);
    const now = registersOf(session);
    const want = changedBetween(this.base, now);
    if (this.app.typing?.()) for (const key of [...want.keys()]) if (key.startsWith('byte:')) want.delete(key);
    if (!want.size) return;
    const { written, refused } = this.#writeAll(want);
    this.#tellRefused(refused);
    const held = new Set(refused.map((r) => r.key));
    for (const [key, value] of want) if (!held.has(key)) setBase(this.base, key, value);
    if (!written.length) return;
    this.group?.local(written.map((w) => w.op));
    if (record) this.history.record(written);
  }

  modeChanged(mode) {
    if (this.shared && this.replica.value('setting:mode') !== mode) this.group?.local([this.replica.set('setting:mode', mode)]);
  }

  undo() { return this.#step('undo'); }

  redo() { return this.#step('redo'); }

  #step(which) {
    if (!this.shared) return false;
    const res = this.history[which](this.replica);
    if (!res) return false;
    if (res.skipped.length) {
      const names = [...new Set(res.skipped.map((s) => this.#who(s.by)))].join(' and ');
      this.app.toast?.(res.ops.length ? `${which === 'undo' ? 'Undone' : 'Redone'}, except what ${names} changed after you.`
        : `${names} changed that after you, so it was not ${which === 'undo' ? 'undone' : 'redone'}.`, { kind: 'warn' });
    }
    if (!res.ops.length) {
      this.app.refresh?.();
      return false;
    }
    this.#dirty();
    const session = this.app.session();
    const keys = res.ops.map((op) => op.k);
    this.applying = true;
    try {
      applyRegisters(session, this.replica, keys);
    } finally {
      this.applying = false;
    }
    this.#rebase(session, keys);
    this.group?.local(res.ops);
    return true;
  }

  #who(peer) {
    return this.ui.nameOf?.(peer) ?? peer;
  }

  /** The registers `keys` (and the rest of their records) reached the Session: those fields are in step again. */
  #rebase(session, keys) {
    const now = registersOf(session);
    const records = new Set();
    for (const key of keys) {
      setBase(this.base, key, now.get(key) ?? null);
      if (key.startsWith('var:') || key.startsWith('data:')) records.add(recordKeyOf(key));
    }
    for (const rk of records) {
      for (const field of ['name', 'type']) setBase(this.base, `${rk}:${field}`, now.get(`${rk}:${field}`) ?? null);
    }
  }

  // ── the others' edits ────────────────────────────────────────────────────

  #remote(changes) {
    this.#dirty();
    for (const by of new Set(changes.map((c) => c.by))) this.ui.remember?.(by);
    if (this.phase === 'joining') {
      this.#stall();
      return;
    }
    if (this.phase !== 'shared') return;
    for (const c of changes) this.pending.push(c);
    if (!this.pendingTimer) this.pendingTimer = this.timers.setTimeout(() => this.#applyPending(), APPLY_MS);
  }

  /**
   * The bytes the student is typing and has not sent yet (a burst goes out
   * as one edit when it ends): they stay on the page as typed, and go out
   * with the rest of the burst even where someone else wrote them meanwhile.
   */
  #typedBytes() {
    const now = registersOf(this.app.session());
    const typed = new Set();
    for (const key of changedBetween(this.base, now).keys()) if (key.startsWith('byte:')) typed.add(key);
    return typed;
  }

  /**
   * Apply a frame's worth of the others' changes to the page at once, after
   * sending what the student changed (except bytes still being typed).
   */
  #applyPending() {
    this.pendingTimer = 0;
    const changes = this.pending;
    this.pending = [];
    if (this.phase !== 'shared' || !changes.length) return;
    const typed = this.app.typing?.() ? this.#typedBytes() : null;
    this.local();
    const words = (c) => `${this.#who(c.by)} ${describeRegister(c.key, c.value, c.prev, this.app.nameOf || ((a) => a))}`;
    const values = changes.filter((c) => c.value !== c.prev);
    const first = values.find((c) => c.by !== this.me);
    const replaced = values.filter((c) => c.prevBy === this.me && c.by !== this.me);
    const keys = [...new Set(changes.map((c) => c.key))].filter((k) => !k.startsWith('setting:') && !typed?.has(k));
    const fn = this.app.target();
    const mode = changes.some((c) => c.key === 'setting:mode') ? this.#lateMode() : null;
    if (replaced.length) {
      const more = replaced.length - 1;
      this.app.toast?.(`${words(replaced[0])} after you`, { kind: 'warn', detail: more ? `and ${more} more of your change${more === 1 ? '' : 's'}` : '' });
    }
    const session = this.app.session();
    this.applying = true;
    try {
      if (keys.length) {
        applyRegisters(session, this.replica, keys);
        this.#rebase(session, keys);
      }
      for (const c of changes) if (typed?.has(c.key)) setBase(this.base, c.key, this.replica.value(c.key));
      this.app.remoteChanged({ inspect: keys.some((k) => touchesFunction(k, fn)), mode, label: first ? words(first) : '' });
    } finally {
      this.applying = false;
    }
  }
}
