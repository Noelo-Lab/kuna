// collab.js — "Work together", loaded by the study view only when a session
// starts or an invite or reply link is opened. It keeps the page's Session
// and the group's registers in step (this page's edits go out as register
// ops; the others' come back through the page's remote path, which
// re-decompiles only what they touch), runs the invite, join and reply
// dialogs, shows who is here in the top bar, hands the program to a newcomer
// who does not have it, and draws the others' pointers and pings. group.js
// runs the protocol, link.js the connections, presence.js the pointers.
import { escapeHtml } from '../../assets/js/highlight-c.js';
import { Session } from '../session.js';
import { Replica, History, registersOf, applyRegisters, adoptRawKeys, describeRegister, recordKeyOf, validOp } from './replica.js';
import { MAX_PEERS, encodeCode, decodeCode, codeFrom, randomId, initials, limiter } from './wire.js';
import { Group } from './group.js';
import { makeOffer, takeOffer, holdPresenceLock } from './link.js';
import { createPresence, anchorAt, findAnchor } from './presence.js';
import { sha256Hex } from './sha256.js';

export const PREFS_KEY = 'kuna.d2.collab';
const STUN = 'stun:stun.l.google.com:19302';
const VIEW_WORDS = { c: 'C code', split: 'Side by side', asm: 'Assembly', bytes: 'Bytes', stack: 'Stack', src: 'Original source' };
const PEOPLE = '<svg width="16" height="16" viewBox="0 0 16 16" aria-hidden="true"><circle cx="6" cy="5.5" r="2.4" fill="none" stroke="currentColor" stroke-width="1.4"/>' +
  '<path d="M1.8 13.5c.5-2.4 2.2-3.7 4.2-3.7s3.7 1.3 4.2 3.7" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>' +
  '<circle cx="11.3" cy="6" r="1.9" fill="none" stroke="currentColor" stroke-width="1.3"/><path d="M11 9.6c1.7 0 2.9 1.1 3.3 3.2" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>';
const NETWORK = 'Could not connect directly. You may need to be on the same network.';
const CLOSED = 'This reply is for an invite that is no longer open. Make a new invite link.';

/** The collab setting (`kuna.d2.collab`): the name last used, and the connection setting. */
export function loadCollabPrefs(storage) {
  try {
    const p = JSON.parse(storage?.getItem(PREFS_KEY) || '{}');
    return p && typeof p === 'object' && !Array.isArray(p) ? p : {};
  } catch (_) {
    return {};
  }
}

/**
 * The ICE servers the setting asks for. None by default (pages on one machine
 * or one network); `stun: true` adds Google's public STUN server, a `stun:`
 * URL another, and `turn: {urls, username, credential}` a TURN relay.
 */
export function iceServersFrom(prefs) {
  const out = [];
  if (prefs?.stun === true) out.push({ urls: STUN });
  else if (typeof prefs?.stun === 'string' && /^stuns?:\S+$/.test(prefs.stun)) out.push({ urls: prefs.stun });
  const t = prefs?.turn;
  if (t && typeof t === 'object' && typeof t.urls === 'string' && /^turns?:\S+$/.test(t.urls)) {
    out.push({ urls: t.urls, username: String(t.username ?? ''), credential: String(t.credential ?? '') });
  }
  return out;
}

const plainName = (v) => {
  const t = String(v || '').replace(/[\u0000-\u001f\u007f-\u009f]/g, '').trim();
  return t.slice(0, 40);
};
const bare = (hex) => String(hex).replace(/^0x/, '');
const sizeWords = (n) => (n < 1024 ? `${n} bytes` : n < 1 << 20 ? `${Math.round(n / 1024)} KB` : `${(n / (1 << 20)).toFixed(1)} MB`);

export async function createCollab(api) {
  const collab = new Collab(api);
  collab.init();
  return collab;
}

class Collab {
  constructor(api) {
    this.api = api;
    this.me = randomId(8);
    this.prefs = loadCollabPrefs(api.storage);
    this.name = plainName(this.prefs.name);
    this.group = null;
    this.replica = null;
    this.replicaHash = null;
    this.history = new History();
    this.shared = false;
    this.holdDiff = false;
    this.localOnly = new Set();
    this.invites = new Map();
    this.closedInvites = new Set();
    this.join = null;
    this.seen = new Map();
    this.cursors = new Map();
    this.following = null;
    this.whereKey = null;
    this.file = null;
    this.fileFor = null;
    this.pointer = null;
    this.pointerAt = 0;
    this.pointerTimer = 0;
    this.pointerSent = false;
    this.pingLimit = limiter(1, 2);
    this.dialog = null;
    this.view = null;
    this.buildPromise = null;
  }

  init() {
    const { els } = this.api;
    holdPresenceLock(this.me);
    const css = document.createElement('link');
    css.rel = 'stylesheet';
    css.href = new URL('./collab.css', import.meta.url).href;
    document.head.appendChild(css);
    this.presence = createPresence({ panes: els.panes });
    els.panes.addEventListener('pointermove', (e) => {
      this.pointer = { target: e.target, x: e.clientX, y: e.clientY };
      if (this.group?.active) this.#queuePointer();
    }, { passive: true });
    els.panes.addEventListener('pointerleave', () => {
      this.pointer = null;
      this.#pointerOff();
    });
    els.panes.addEventListener('click', (e) => {
      if (!e.altKey || !this.group?.active) return;
      e.preventDefault();
      e.stopPropagation();
      this.ping({ target: e.target, x: e.clientX, y: e.clientY });
    }, { capture: true });
    document.addEventListener('pointerdown', (e) => {
      if (this.following && !e.target.closest?.('.d2-who')) this.#follow(null);
    }, { capture: true });
    document.addEventListener('keydown', (e) => {
      if (this.following && !['Shift', 'Control', 'Alt', 'Meta'].includes(e.key)) this.#follow(null);
    }, { capture: true });
    addEventListener('pagehide', () => this.group?.leave());
    if (globalThis.BroadcastChannel) {
      this.replyChannel = new BroadcastChannel('kuna.d2.reply');
      this.replyChannel.onmessage = ({ data }) => this.#handOff(data);
    }
  }

  get active() {
    return !!this.group?.active;
  }

  ice() {
    return iceServersFrom(loadCollabPrefs(this.api.storage));
  }

  #savePrefs() {
    const p = loadCollabPrefs(this.api.storage);
    p.name = this.name;
    try { this.api.storage?.setItem(PREFS_KEY, JSON.stringify(p)); } catch (_) { /* no storage */ }
  }

  /** The build id: the SHA-256 of the engine's wasm, which every page in a session must share. */
  build() {
    this.buildPromise ||= fetch(this.api.wasmUrl)
      .then((r) => {
        if (!r.ok) throw new Error(`the engine could not be read (${r.status})`);
        return r.arrayBuffer();
      })
      .then((b) => sha256Hex(new Uint8Array(b)))
      .catch((e) => {
        this.buildPromise = null;
        throw e;
      });
    return this.buildPromise;
  }

  /** `{name, size, hash}` of the open program (SHA-256), computed once per program. */
  async fileMeta() {
    const bin = this.api.binary();
    if (!bin) return null;
    if (this.fileFor !== bin) {
      const hash = /^sha256:[0-9a-f]{64}$/.test(bin.hash || '') ? bin.hash.slice(7) : await sha256Hex(bin.bytes);
      if (this.api.binary() !== bin) return this.fileMeta();
      this.file = { name: bin.name.replace(/[\\/]/g, '_').slice(0, 255) || 'program', size: bin.bytes.length, hash };
      this.fileFor = bin;
    }
    return this.file;
  }

  #fileMetaNow() {
    return this.fileFor && this.fileFor === this.api.binary() ? this.file : null;
  }

  // ── the group ────────────────────────────────────────────────────────────

  #connect() {
    return {
      offer: () => makeOffer({ me: this.me, iceServers: this.ice(), bc: true }),
      answer: async ({ id, sdp, bc }) => {
        const res = await takeOffer({ me: this.me, id, sdp, iceServers: this.ice(), bc });
        return res.link ? { sdp: null, ready: Promise.resolve(res.link), cancel() {} } : res;
      },
    };
  }

  #newGroup(build) {
    this.group?.leave();
    this.group = new Group({
      me: this.me, name: this.name, build, replica: this.replica, connect: this.#connect(),
      page: {
        fileMeta: () => this.#fileMetaNow(),
        fileBytes: () => this.api.binary()?.bytes || null,
        isExample: () => !!this.api.binary()?.example,
        welcomed: (info) => this.#welcomed(info),
        caughtUp: () => this.#caughtUp(),
        changed: (changes, from) => this.#changed(changes, from),
        fileProgress: (got, size) => this.#progress(got, size),
        fileArrived: (bytes, meta) => this.#fileArrived(bytes, meta),
        fileFailed: (why) => this.#fileFailed(why),
        roster: () => this.#rosterChanged(),
        where: () => this.#whereOf(),
        cursor: (peer, m) => this.#cursor(peer, m),
        ping: (peer, m) => this.#pinged(peer, m),
        event: (kind, info) => this.#event(kind, info),
      },
    });
  }

  /** Start a session as its first member, with this page's changes as its registers. */
  async start() {
    if (this.active) return true;
    const [build, file] = await Promise.all([this.build(), this.fileMeta()]);
    if (!file) return false;
    if (this.replicaHash !== file.hash) {
      this.replica = new Replica(this.me);
      this.replicaHash = file.hash;
    }
    this.#newGroup(build);
    this.group.create();
    this.#share();
    this.#diff({ record: false });
    if (this.replica.value('setting:mode') !== this.api.mode()) this.group.local([this.replica.set('setting:mode', this.api.mode())]);
    this.group.setWhere(this.api.current());
    this.#rosterChanged();
    return true;
  }

  #share() {
    this.shared = true;
    this.localOnly.clear();
    this.history.clear();
    this.api.clearUndo();
    this.api.refresh();
  }

  /** Leave: close every link; this page keeps the session as it is. */
  leave({ quiet = false } = {}) {
    this.group?.leave();
    this.group = null;
    for (const inv of this.invites.values()) {
      inv.offer?.cancel?.();
      this.closedInvites.add(inv.id);
    }
    this.invites.clear();
    this.join?.res?.cancel?.();
    this.join = null;
    this.shared = false;
    this.holdDiff = false;
    this.following = null;
    this.history.clear();
    this.api.clearUndo();
    this.cursors.clear();
    this.presence.clear();
    this.#rosterChanged();
    this.api.refresh();
    if (!quiet) this.api.toast('You left the session. Your changes stay here.');
  }

  /** Before opening another program: leaving is the only way; ask first. */
  async confirmLeave(name) {
    if (!this.active) return true;
    const ok = await this.api.confirm(`Opening ${name} ends your part in this session. Leave the session and open it?`, 'Leave and open');
    if (ok) this.leave({ quiet: true });
    return ok;
  }

  // ── this page's edits ────────────────────────────────────────────────────

  #write(key, value) {
    const prev = this.replica.value(key);
    const prevClock = this.replica.clock(key);
    const op = this.replica.set(key, value);
    return { key, prev, prevClock, op };
  }

  /** Compare the page's Session with the registers and send what this page changed. */
  #diff({ record = true } = {}) {
    if (!this.shared || this.holdDiff || !this.replica) return;
    const session = this.api.session();
    adoptRawKeys(session, this.me);
    const regs = registersOf(session, this.me);
    const changes = [];
    let refused = 0;
    for (const [k, v] of regs) {
      if (this.localOnly.has(k) || this.replica.value(k) === v) continue;
      if (!validOp({ k, v, c: [1, this.me] })) {
        this.localOnly.add(k);
        refused++;
        continue;
      }
      changes.push(this.#write(k, v));
    }
    for (const k of this.replica.live().keys()) {
      if (!k.startsWith('setting:') && !regs.has(k)) changes.push(this.#write(k, null));
    }
    if (refused) {
      this.api.toast('One of your changes stays on this page only.', {
        kind: 'warn', detail: 'It reads a file or holds text the others\' pages do not accept, so it is not shared.',
      });
    }
    if (!changes.length) return;
    this.authors = null;
    this.group?.local(changes.map((c) => c.op));
    if (record) this.history.record(changes);
  }

  /** The page's session changed (an edit, an undo, a restore). */
  localChanged() {
    this.#diff();
  }

  modeChanged(mode) {
    if (this.shared && this.replica.value('setting:mode') !== mode) this.group?.local([this.replica.set('setting:mode', mode)]);
  }

  get canUndo() { return this.history.canUndo; }

  get canRedo() { return this.history.canRedo; }

  undo() { return this.#step('undo'); }

  redo() { return this.#step('redo'); }

  #step(which) {
    if (!this.shared) return false;
    const res = this.history[which](this.replica);
    if (!res) return false;
    if (res.skipped.length) {
      const names = [...new Set(res.skipped.map((s) => this.#nameOf(s.by)))].join(' and ');
      this.api.toast(res.ops.length ? `${which === 'undo' ? 'Undone' : 'Redone'}, except what ${names} changed after you.`
        : `${names} changed that after you, so it was not ${which === 'undo' ? 'undone' : 'redone'}.`, { kind: 'warn' });
    }
    if (!res.ops.length) {
      this.api.refresh();
      return false;
    }
    this.authors = null;
    applyRegisters(this.api.session(), this.replica, res.ops.map((op) => op.k));
    this.group?.local(res.ops);
    return true;
  }

  // ── the others' edits ────────────────────────────────────────────────────

  #changed(changes, from) {
    this.authors = null;
    for (const ch of changes) this.#remember(ch.by);
    if (!this.shared) return;
    const words = (c) => `${this.#nameOf(c.by)} ${describeRegister(c.key, c.value, c.prev, this.api.nameOf)}`;
    const first = changes.find((c) => c.by !== this.me) || changes[0];
    const label = words(first);
    const replaced = changes.filter((c) => c.prevBy === this.me && c.by !== this.me && c.value !== c.prev);
    const afterYou = replaced.length ? `${words(replaced[0])} after you` : null;
    const session = this.api.session();
    const fn = this.api.current().fn;
    const before = fn ? session.assertionsFor(fn).join('\n') : '';
    const keys = changes.map((c) => c.key).filter((k) => !k.startsWith('setting:'));
    if (keys.length) applyRegisters(session, this.replica, keys);
    const after = fn ? session.assertionsFor(fn).join('\n') : '';
    const mode = changes.some((c) => c.key === 'setting:mode') ? this.replica.value('setting:mode') : null;
    if (afterYou) {
      const more = replaced.length - 1;
      this.api.toast(afterYou, { kind: 'warn', detail: more ? `and ${more} more of your change${more === 1 ? '' : 's'}` : '' });
    }
    this.api.remoteChanged({ inspect: before !== after, mode: mode && mode !== this.api.mode() ? mode : null, label });
  }

  // ── inviting ─────────────────────────────────────────────────────────────

  /** Make a one-person invite link (starting a session if there is none). */
  async makeInvite() {
    if (!(await this.start())) return null;
    if (this.group.size() >= MAX_PEERS) {
      this.#status('This session is full (8 people).', 'err');
      return null;
    }
    this.#status('Making an invite link…', 'busy');
    let offer;
    try {
      offer = await makeOffer({ me: this.me, iceServers: this.ice(), bc: true });
    } catch (e) {
      this.#status(`This browser could not make a connection: ${e.message}`, 'err');
      return null;
    }
    const file = this.#fileMetaNow() || await this.fileMeta();
    const code = await encodeCode('invite', { id: offer.id, n: this.name, f: file.name, z: file.size, d: offer.sdp, b: 1 });
    const inv = { id: offer.id, offer, link: `${this.#base()}#join=${code}`, state: 'waiting', guest: null };
    this.invites.set(offer.id, inv);
    offer.onstate = (s) => {
      if (inv.state === 'connecting' && s === 'checking') this.#inviteStatus(inv);
    };
    offer.ready.then((link) => {
      if (!this.active) {
        link.close();
        return;
      }
      inv.state = 'open';
      this.group.addLink(link);
      this.#inviteStatus(inv);
      this.#tellReplyTab(inv, 'open');
    }).catch((e) => {
      if (e.message === 'cancelled') return;
      inv.state = 'failed';
      this.#inviteStatus(inv);
      this.#tellReplyTab(inv, 'failed');
    });
    this.current = inv;
    return inv;
  }

  #base() {
    return location.origin + location.pathname + location.search;
  }

  /** A reply (pasted, or handed over by another tab): connect the invite it answers. */
  async applyReply(text) {
    const code = codeFrom(text, 'reply');
    if (!code) return { ok: false, text: 'That is not a reply link. It starts with the page address and has #reply= in it.' };
    const r = await decodeCode(code, 'reply');
    if (!r.ok) {
      return { ok: false, text: r.why === 'version' ? 'That reply was made by a different version of Kuna. Reload both pages and try again.'
        : 'That reply link is not complete. Copy all of it and try again.' };
    }
    const inv = this.invites.get(r.id);
    if (!inv && this.closedInvites.has(r.id)) return { ok: false, text: CLOSED };
    if (!inv) return { ok: false, text: 'That reply is for an invite made in another tab or browser. Open it there.' };
    if (inv.state !== 'waiting') return { ok: false, text: CLOSED, inv };
    inv.guest = r.n;
    inv.state = 'connecting';
    this.#inviteStatus(inv);
    try {
      await inv.offer.answer(r.d);
    } catch (e) {
      inv.state = 'failed';
      this.#inviteStatus(inv);
      return { ok: false, text: e.message === 'used' ? CLOSED : NETWORK, inv };
    }
    return { ok: true, inv };
  }

  /** Another tab of this browser opened a reply link: take it if the invite is ours. */
  async #handOff(data) {
    if (data?.k !== 'reply' || typeof data.id !== 'string' || typeof data.code !== 'string' || typeof data.from !== 'string') return;
    const inv = this.invites.get(data.id);
    const post = (msg) => { try { this.replyChannel.postMessage({ ...msg, id: data.id, to: data.from }); } catch (_) { /* closed */ } };
    if (!inv || inv.state !== 'waiting') {
      if (inv || this.closedInvites.has(data.id)) post({ k: 'used' });
      return;
    }
    post({ k: 'got', name: this.name });
    inv.replyTab = data.from;
    const res = await this.applyReply(data.code);
    if (!res.ok) post({ k: 'state', state: 'failed', text: res.text });
  }

  #tellReplyTab(inv, state) {
    if (!inv.replyTab) return;
    try { this.replyChannel?.postMessage({ k: 'state', id: inv.id, to: inv.replyTab, state }); } catch (_) { /* closed */ }
  }

  // ── joining ──────────────────────────────────────────────────────────────

  /** An opened `#join=` or `#reply=` link. */
  async handleHash(hash) {
    if (/^#join=/.test(hash)) return this.#openInvite(codeFrom(hash, 'invite'));
    if (/^#reply=/.test(hash)) return this.#openReply(codeFrom(hash, 'reply'));
    return null;
  }

  async #openInvite(code) {
    const inv = await decodeCode(code, 'invite');
    if (!inv.ok) {
      this.#show({
        kind: 'notice', title: 'This invite link does not work',
        text: inv.why === 'version' ? 'It was made by a different version of Kuna. Ask for a new link, and reload this page first.'
          : 'Part of it is missing. Ask for the link again, and copy all of it.',
      });
      return;
    }
    if (this.invites.has(inv.id)) {
      this.#show({ kind: 'notice', title: 'This is your own invite link', text: 'Send it to the person you want to invite. When they open it, they get a reply link to send back to you.' });
      return;
    }
    this.#show({ kind: 'join', inv });
  }

  async #joinNow(inv) {
    if (this.active) this.leave({ quiet: true });
    this.join = { inv, state: 'connecting', example: false };
    this.#render();
    let build;
    let res;
    try {
      [build] = await Promise.all([this.build(), this.fileMeta()]);
      res = await takeOffer({ me: this.me, id: inv.id, sdp: inv.d, iceServers: this.ice(), bc: inv.b === 1 });
    } catch (e) {
      this.#joinFailed(e.message === 'used' ? 'This invite link was already used: each link lets one person in. Ask for a new one.'
        : `This browser could not make a connection: ${e.message}`);
      return;
    }
    if (!this.join || this.join.inv !== inv) return;
    if (res.link) {
      this.#joined(res.link, build);
      return;
    }
    this.join.res = res;
    const code = await encodeCode('reply', { id: inv.id, n: this.name, d: res.sdp });
    this.join.reply = `${this.#base()}#reply=${code}`;
    this.join.state = 'reply';
    res.onstate = (s) => {
      if (s === 'failed' && this.join?.res === res && this.join.state === 'reply') {
        this.join.stuck = true;
        this.#render();
      }
    };
    res.ready.then((link) => { if (this.join?.res === res) this.#joined(link, build); else link.close(); }).catch(() => {});
    this.#render();
  }

  #joined(link, build) {
    const bin = this.api.binary();
    if (!(this.replica && this.fileFor === bin && this.file && this.replicaHash === this.file.hash)) {
      this.replica = new Replica(this.me);
      this.replicaHash = null;
    }
    this.#newGroup(build);
    this.join.state = 'joining';
    this.group.addLink(link, { joining: true });
    this.#render();
  }

  #joinFailed(text) {
    if (this.join) {
      this.join.state = 'failed';
      this.join.error = text;
    }
    this.group?.leave();
    this.group = null;
    this.#render();
    this.#rosterChanged();
  }

  #welcomed({ from, name, file, example, send }) {
    if (!this.join) this.join = { inv: null, state: 'joining' };
    Object.assign(this.join, { sponsor: from, sponsorName: name, file, send, example, state: send ? 'receiving' : 'merging' });
    this.#remember(from);
    if (this.replicaHash !== file.hash) {
      if (this.replica.regs.size) {
        this.replica = new Replica(this.me);
        this.group.replica = this.replica;
      }
      this.replicaHash = file.hash;
      this.holdDiff = true;
    }
    const mine = this.#fileMetaNow();
    if (mine && mine.hash === file.hash) {
      this.#share();
      this.#diff({ record: false });
    }
    this.group.setWhere(this.api.current());
    this.#rosterChanged();
    this.#render();
  }

  #caughtUp() {
    const held = this.holdDiff;
    this.holdDiff = false;
    if (!this.shared) return;
    if (held) this.#diff({ record: false });
    this.#joinDone();
  }

  #progress(got, size) {
    if (!this.join) return;
    this.join.got = got;
    this.join.size = size;
    const bar = this.dialog?.querySelector('[data-progress]');
    if (bar) {
      bar.style.width = `${Math.round((got / size) * 100)}%`;
      this.#setStatusText(`Receiving ${this.join.file?.name || 'the program'}… ${Math.round((got / size) * 100)}%`);
    }
  }

  async #fileArrived(bytes, meta) {
    if (!this.join || this.shared) return;
    this.join.state = 'opening';
    this.#render();
    const session = new Session();
    applyRegisters(session, this.replica, [...this.replica.regs.keys()]);
    const example = !!this.join.example && await this.api.isExample(bytes);
    const old = await this.api.storedSession(bytes);
    const mode = this.replica.value('setting:mode');
    const open = this.group?.who(this.join.sponsor)?.where?.fn || null;
    const ok = await this.api.openShared({ name: meta.name, bytes, example, session, mode, open });
    if (!ok || !this.group) {
      this.#joinFailed(`Could not open ${meta.name}.`);
      return;
    }
    await this.fileMeta();
    this.#share();
    const opened = JSON.stringify(this.api.session().toJSON());
    applyRegisters(this.api.session(), this.replica, [...this.replica.regs.keys()]);
    const moved = JSON.stringify(this.api.session().toJSON()) !== opened;
    const lateMode = this.replica.value('setting:mode') !== this.api.mode() ? this.replica.value('setting:mode') : null;
    if (moved || lateMode) this.api.remoteChanged({ inspect: true, mode: lateMode, label: '' });
    if (old?.size) {
      this.api.toast(`Your own earlier changes to ${meta.name} are not part of this session.`, {
        kind: 'warn', ms: 15000, detail: `${old.size} change${old.size === 1 ? '' : 's'} from before.`,
        action: { label: 'Save them as a file', run: () => this.api.exportSession(old, meta.name) },
      });
    }
    this.#joinDone();
  }

  #fileFailed(why) {
    this.#joinFailed(why === 'hash' ? 'The program did not arrive intact. Ask for a new invite link and try again.'
      : 'The connection closed while the program was on its way. Ask for a new invite link and try again.');
  }

  #joinDone() {
    const j = this.join;
    if (!j || j.state === 'done' || !this.shared || !this.group) return;
    j.state = 'done';
    const where = this.group.who(j.sponsor)?.where;
    if (where?.fn && this.api.current().fn && where.fn !== this.api.current().fn) this.api.openFunction(where.fn);
    this.api.toast(`You joined ${j.sponsorName}'s session.`, { detail: 'Their changes and yours now appear on every page.' });
    if (this.view?.kind === 'join') this.close();
    this.join = null;
    this.#rosterChanged();
  }

  async #openReply(code) {
    const r = await decodeCode(code, 'reply');
    if (!r.ok) {
      this.#show({
        kind: 'notice', title: 'This reply link does not work',
        text: r.why === 'version' ? 'It was made by a different version of Kuna. Reload both pages and try again.'
          : 'Part of it is missing. Copy all of it and try again.',
      });
      return;
    }
    if (this.invites.has(r.id) || this.closedInvites.has(r.id)) {
      const res = await this.applyReply(code);
      if (!this.active) {
        this.#show({ kind: 'notice', title: `${r.n}'s reply`, text: res.ok ? 'Connecting…' : res.text });
        return;
      }
      this.#show({ kind: 'session' });
      if (res.inv) {
        this.current = res.inv;
        this.#render();
        this.#inviteStatus(res.inv);
      }
      if (!res.ok) this.#status(res.text, 'err');
      return;
    }
    const view = { kind: 'reply', reply: r, code, state: 'handing', text: '' };
    this.#show(view);
    if (!this.replyChannel) {
      view.state = 'nobody';
      this.#render();
      return;
    }
    const from = randomId(8);
    const listen = new BroadcastChannel('kuna.d2.reply');
    let give = 0;
    const end = (state, text = '') => {
      clearTimeout(timer);
      clearTimeout(give);
      listen.close();
      view.state = state;
      view.text = text;
      this.#render();
    };
    const timer = setTimeout(() => end('nobody'), 1500);
    listen.onmessage = ({ data }) => {
      if (data?.to !== from || data.id !== r.id) return;
      if (data.k === 'got') {
        clearTimeout(timer);
        view.state = 'connecting';
        give = setTimeout(() => end('failed', NETWORK), 25000);
        this.#render();
      } else if (data.k === 'used') {
        end('failed', CLOSED);
      } else if (data.k === 'state') {
        end(data.state === 'open' ? 'open' : 'failed', data.state === 'open' ? '' : (typeof data.text === 'string' ? data.text.slice(0, 200) : NETWORK));
      }
    };
    listen.postMessage({ k: 'reply', id: r.id, code, from });
  }

  // ── presence: where people are, pointers, pings, following ───────────────

  #remember(peer) {
    const m = this.group?.who(peer);
    if (m) this.seen.set(peer, { name: m.name, color: m.color });
  }

  #nameOf(peer) {
    if (peer === this.me) return 'You';
    return this.group?.who(peer)?.name || this.seen.get(peer)?.name || 'Someone';
  }

  #member(peer) {
    const m = this.group?.who(peer);
    return m ? { name: m.name, color: m.color } : this.seen.get(peer) || { name: 'Someone', color: '#a39894' };
  }

  whereChanged() {
    const w = this.api.current();
    const key = `${w.fn}|${w.view}`;
    if (key === this.whereKey) return;
    const moved = this.whereKey?.split('|')[0] !== String(w.fn);
    this.whereKey = key;
    if (this.group?.active) {
      this.group.setWhere(w);
      if (moved) this.#pointerOff();
    }
    for (const [peer, m] of this.cursors) this.presence.show(peer, m, this.#member(peer), m.fn === w.fn, { moved: false });
  }

  redraw() {
    this.presence?.redraw();
  }

  #queuePointer() {
    if (this.pointerTimer) return;
    const wait = Math.max(0, 34 - (performance.now() - this.pointerAt));
    this.pointerTimer = setTimeout(() => {
      let sent = false;
      const send = () => {
        if (sent) return;
        sent = true;
        this.pointerTimer = 0;
        this.#sendPointer();
      };
      if (!document.hidden) requestAnimationFrame(send);
      setTimeout(send, 50);
    }, wait);
  }

  #sendPointer() {
    const p = this.pointer;
    const cur = this.api.current();
    if (!p || !cur.fn || !this.group?.active) return;
    const a = anchorAt(p.target, p.x, p.y);
    if (!a) {
      this.#pointerOff();
      return;
    }
    this.pointerAt = performance.now();
    const r3 = (v) => Math.round(v * 1000) / 1000;
    this.group.sendCursor({ fn: cur.fn, view: cur.view, anchor: a.anchor, fx: r3(a.fx), fy: r3(a.fy), ...(a.col === undefined ? {} : { col: a.col }) });
    this.pointerSent = true;
  }

  #pointerOff() {
    if (!this.pointerSent || !this.group?.active) return;
    this.pointerSent = false;
    this.group.sendCursor({ off: true });
  }

  #cursor(peer, m) {
    if (m.off) {
      this.cursors.delete(peer);
      this.presence.hide(peer);
      return;
    }
    this.cursors.set(peer, m);
    const mine = this.api.current().fn;
    this.presence.show(peer, m, this.#member(peer), m.fn === mine);
    if (this.following === peer && m.fn === mine && !this.presence.visible(m.anchor)) {
      findAnchor(m.anchor)?.scrollIntoView({ block: 'nearest' });
    }
  }

  #pingLabel(fn, anchor) {
    const name = this.api.nameOf(fn);
    const v = anchor.slice(2);
    switch (anchor[0]) {
      case 'c': case 'h': return `line ${v} of ${name}`;
      case 'a': return `the instruction at ${bare(v)} in ${name}`;
      case 'b': return `the byte at ${bare(v)} in ${name}`;
      default: {
        const off = Number(v);
        return `the stack slot ${Math.abs(off)} bytes ${off < 0 ? 'below' : 'above'} the return address in ${name}`;
      }
    }
  }

  /** Point the others to a thing: the one clicked, else the one under the mouse, else the selection. */
  ping(at = null) {
    if (!this.group?.active) return false;
    const cur = this.api.current();
    if (!cur.fn) return false;
    const under = at ? anchorAt(at.target, at.x, at.y) : this.pointer ? anchorAt(this.pointer.target, this.pointer.x, this.pointer.y) : null;
    const anchor = under?.anchor || this.api.selectionAnchor();
    if (!anchor) {
      this.api.toast('Point at a line, an instruction, a byte or a stack slot, then press P (or Alt+click it).', { kind: 'warn' });
      return true;
    }
    if (!this.pingLimit.take()) return true;
    this.group.sendPing({ fn: cur.fn, view: cur.view, anchor });
    this.presence.pulse(anchor, this.#member(this.me).color || this.group.color, this.me);
    this.api.status(`You pointed the others to ${this.#pingLabel(cur.fn, anchor)}`);
    return true;
  }

  #pinged(peer, m) {
    const who = this.#member(peer);
    if (this.api.current().fn === m.fn) this.presence.pulse(m.anchor, who.color, peer);
    this.api.toast(`${who.name} pinged ${this.#pingLabel(m.fn, m.anchor)}`, {
      ms: 10000, action: { label: 'Go there', run: () => this.goTo(m.fn, m.anchor, who.color, peer) },
    });
  }

  async goTo(fn, anchor, color, peer) {
    await this.api.goTo(fn, anchor);
    findAnchor(anchor)?.scrollIntoView({ block: 'center' });
    setTimeout(() => this.presence.pulse(anchor, color, peer), 60);
  }

  #follow(peer) {
    this.following = peer;
    this.#rosterChanged();
    if (!peer) return;
    const m = this.group?.who(peer);
    if (!m?.where?.fn) {
      this.api.toast(`${m?.name || 'They'} ${m?.linked === false ? 'are still connecting' : 'have not opened a function yet'}.`);
      this.following = null;
      this.#rosterChanged();
      return;
    }
    this.#followTo(m);
    this.api.toast(`Following ${m.name}. Click anywhere or press a key to stop.`);
  }

  #followTo(m) {
    if (m.where.fn !== this.api.current().fn) this.api.openFunction(m.where.fn, { view: m.where.view });
    else if (m.where.view !== this.api.current().view) this.api.setView(m.where.view);
  }

  /** Someone moved to another function or view: the roster's tooltips, and whoever is followed. */
  #whereOf() {
    if (this.following) {
      const m = this.group?.who(this.following);
      if (m?.where?.fn) this.#followTo(m);
    }
    this.#renderRoster();
    if (this.view?.kind === 'session') this.#renderPeople();
  }

  #rosterChanged() {
    this.authors = null;
    if (this.following) {
      const m = this.group?.who(this.following);
      if (!m) this.following = null;
      else if (m.where?.fn) this.#followTo(m);
    }
    for (const m of this.group?.members() || []) this.seen.set(m.peer, { name: m.name, color: m.color });
    for (const peer of [...this.cursors.keys()]) {
      if (!this.group?.who(peer)?.linked) {
        this.cursors.delete(peer);
        this.presence.forget(peer);
      }
    }
    this.#renderRoster();
    this.api.refresh();
    if (this.view?.kind === 'session' || this.view?.kind === 'start') this.#renderPeople();
  }

  #whereWords(m) {
    if (!m.linked) return `${m.name}: connecting…`;
    if (!m.where?.fn) return `${m.name}: no function open yet`;
    return `${m.name}: ${this.api.nameOf(m.where.fn)}, ${VIEW_WORDS[m.where.view] || 'C code'}`;
  }

  #renderRoster() {
    const { els } = this.api;
    if (!this.rosterEl) {
      this.rosterEl = document.createElement('div');
      this.rosterEl.className = 'd2-roster';
      this.rosterEl.id = 'd2roster';
      this.rosterEl.setAttribute('role', 'group');
      this.rosterEl.setAttribute('aria-label', 'People working together');
      els.hintsBox.closest('label').before(this.rosterEl);
      this.rosterEl.addEventListener('click', (e) => {
        const chip = e.target.closest('.d2-who');
        if (chip) this.#follow(this.following === chip.dataset.peer ? null : chip.dataset.peer);
        else if (e.target.closest('[data-act=collab-open]')) this.open();
      });
    }
    const members = this.group?.active ? this.group.members() : [];
    this.rosterEl.hidden = !members.length;
    if (!members.length) {
      this.rosterEl.innerHTML = '';
      return;
    }
    const chips = members.filter((m) => !m.me).map((m) => {
      const on = this.following === m.peer;
      return `<button class="d2-who${on ? ' on' : ''}${m.linked ? '' : ' wait'}" data-peer="${m.peer}" style="--who:${m.color}"` +
        ` title="${escapeHtml(`${this.#whereWords(m)}\n${on ? 'Following: click to stop' : 'Click to follow'}`)}"` +
        ` aria-pressed="${on}">${escapeHtml(initials(m.name))}</button>`;
    }).join('');
    this.rosterEl.innerHTML = chips +
      `<button class="d2-btn ghost d2-together" data-act="collab-open" title="Working together: invite someone, or leave">${PEOPLE}` +
      `<span>${members.length}</span><span class="d2-sr"> people</span></button>`;
  }

  /** The author of a record in the Changes list (whoever wrote its newest register): `{name, color, me}` or null. */
  authorOf(key) {
    if (!this.shared || !this.replica) return null;
    const later = (a, b) => !b || a[0] > b[0] || (a[0] === b[0] && a[1] > b[1]);
    if (!this.authors) {
      this.authors = new Map();
      for (const [k, { c }] of this.replica.regs) {
        const rk = recordKeyOf(k);
        if (later(c, this.authors.get(rk))) this.authors.set(rk, c);
      }
    }
    let best = null;
    if (key.startsWith('bytes:')) {
      const run = this.api.session().byteRuns().find((r) => `bytes:0x${r.addr.toString(16)}` === key);
      for (let i = 0; run && i < run.values.length; i++) {
        const c = this.replica.clock(`byte:0x${(run.addr + BigInt(i)).toString(16)}`);
        if (c && later(c, best)) best = c;
      }
    } else {
      best = this.authors.get(key) || null;
    }
    if (!best) return null;
    const who = best[1] === this.me ? { name: this.name, color: this.group?.color || this.#member(this.me).color } : this.#member(best[1]);
    return { name: who.name, color: who.color, me: best[1] === this.me };
  }

  #event(kind, info) {
    const name = info?.name || 'Someone';
    switch (kind) {
      case 'joined':
        this.api.toast(`${name} joined.`);
        if (this.current?.state === 'open' && !this.current.guest) {
          this.current.guest = name;
          this.#inviteStatus(this.current);
        }
        break;
      case 'left':
        this.api.toast(`${name} left the session.`);
        this.cursors.delete(info.peer);
        this.presence.forget(info.peer);
        break;
      case 'lost':
        this.api.toast(`Lost the connection to ${name}.`, { kind: 'warn', detail: info.reachable ? 'Trying again through the others…' : '' });
        this.cursors.delete(info.peer);
        this.presence.forget(info.peer);
        break;
      case 'full':
        this.#joinFailed('This session is full (8 people).');
        break;
      case 'mismatch':
        if (this.join) this.#joinFailed(`${name}'s page is a different version of Kuna; reload both.`);
        else this.api.toast(`${name}'s page is a different version of Kuna; reload both.`, { kind: 'err' });
        break;
      case 'closed':
        if (this.join && this.join.state !== 'done') this.#joinFailed('The connection closed before you joined. Ask for a new invite link.');
        break;
      default: break;
    }
    this.#rosterChanged();
  }

  // ── the dialog ───────────────────────────────────────────────────────────

  /** ⋯ → Work together…: invite (or, in a session, who is here, invite more, leave). */
  open() {
    this.#show({ kind: this.join && this.join.state !== 'done' ? 'join' : this.active ? 'session' : 'start', inv: this.join?.inv });
  }

  close() {
    this.view = null;
    if (this.dialog?.open) this.dialog.close();
  }

  #show(view) {
    this.view = view;
    if (!this.dialog) {
      this.dialog = document.createElement('dialog');
      this.dialog.id = 'd2collab';
      this.dialog.className = 'd2-collab';
      this.dialog.setAttribute('aria-labelledby', 'd2collabtitle');
      document.body.appendChild(this.dialog);
      this.dialog.addEventListener('keydown', (e) => e.stopPropagation());
      this.dialog.addEventListener('click', (e) => this.#click(e));
      this.dialog.addEventListener('submit', (e) => this.#submit(e));
      this.dialog.addEventListener('close', () => { this.view = null; });
    }
    this.#render();
    if (!this.dialog.open) this.dialog.showModal();
    const first = this.dialog.querySelector('[autofocus]');
    first?.focus();
    if (first?.select) first.select();
  }

  #status(text, cls = '') {
    this.statusText = { text, cls };
    this.#setStatusText(text, cls);
  }

  #setStatusText(text, cls = null) {
    const el = this.dialog?.querySelector('[data-status]');
    if (!el) return;
    el.textContent = text;
    if (cls !== null) el.className = `cb-status ${cls}`;
  }

  #inviteStatus(inv) {
    if (this.current !== inv) return;
    const words = {
      waiting: ['Waiting for their reply link…', 'busy'],
      connecting: [`Connecting to ${inv.guest || 'them'}…`, 'busy'],
      open: [`${inv.guest || 'They'} joined.`, 'ok'],
      failed: [NETWORK, 'err'],
    }[inv.state] || ['', ''];
    this.#status(...words);
    const paste = this.dialog?.querySelector('[data-paste]');
    if (paste) paste.hidden = inv.state !== 'waiting';
  }

  #nameField() {
    return '<label class="cb-field"><span>Your name</span>' +
      `<input class="d2-input" name="name" maxlength="40" autocomplete="nickname" spellcheck="false" value="${escapeHtml(this.name)}" required autofocus></label>`;
  }

  #copyRow(value, label) {
    return `<div class="cb-copy"><input class="d2-input" readonly value="${escapeHtml(value)}" aria-label="${escapeHtml(label)}" data-copytext>` +
      '<button type="button" class="d2-btn" data-act="copy">Copy</button></div>';
  }

  #render() {
    const v = this.view;
    if (!this.dialog || !v) return;
    const bin = this.api.binary();
    let title = 'Work together';
    let body = '';
    if (v.kind === 'notice') {
      title = v.title;
      body = `<p>${escapeHtml(v.text)}</p><div class="cb-row"><button type="button" class="d2-btn primary" data-act="close" autofocus>OK</button></div>`;
    } else if (v.kind === 'join') {
      ({ title, body } = this.#joinView(v));
    } else if (v.kind === 'reply') {
      ({ title, body } = this.#replyView(v));
    } else if (!bin && !this.active) {
      body = '<p>Open a program first. Then you can invite people to work on it with you.</p>' +
        '<div class="cb-row"><button type="button" class="d2-btn primary" data-act="close" autofocus>OK</button></div>';
    } else if (!this.active) {
      body = `<p>Invite someone to work on <b>${escapeHtml(bin.name)}</b> with you. You both see the same program and each other's changes — names, types, notes and patched bytes — and where the other person is pointing.</p>` +
        `<p class="cb-note">The people who join receive a copy of ${escapeHtml(bin.name)} from your browser.</p>` +
        `<form data-form="start">${this.#nameField()}<div class="cb-row"><button class="d2-btn primary" type="submit">Make an invite link</button></div></form>` +
        '<p class="cb-small d2-teach">It works between tabs of this browser and between computers on the same network. Nothing is sent to a server.</p>';
    } else {
      const alone = this.group.size() < 2;
      title = alone ? 'Invite someone' : `Working together on ${bin ? bin.name : 'a program'}`;
      body = `<div data-people${alone ? ' hidden' : ''}>${this.#peopleHtml()}</div>${this.#inviteHtml()}` +
        '<p class="cb-small d2-teach">Everyone can change anything. Undo takes back only your own changes. Alt+click a line (or press P) to point the others to it.</p>' +
        '<div class="cb-row cb-end"><button type="button" class="d2-btn" data-act="leave">Leave the session</button>' +
        '<button type="button" class="d2-btn primary" data-act="close">Done</button></div>';
    }
    this.dialog.innerHTML = '<div class="cb-head">' +
      `<h2 id="d2collabtitle">${escapeHtml(title)}</h2>` +
      '<button type="button" class="d2-iconbtn small" data-act="close" aria-label="Close" title="Close">×</button></div>' +
      `<div class="cb-body">${body}</div>`;
    if (this.statusText && (v.kind === 'session' || v.kind === 'start')) this.#setStatusText(this.statusText.text, this.statusText.cls);
  }

  #peopleHtml() {
    const members = this.group?.members() || [];
    return '<ul class="cb-people">' + members.map((m) => `<li style="--who:${m.color}"><span class="cb-dot"></span>` +
      `<span class="cb-name">${escapeHtml(m.name)}${m.me ? ' <span class="cb-muted">(you)</span>' : ''}</span>` +
      `<span class="cb-muted">${escapeHtml(m.me ? '' : this.#whereWords(m).slice(m.name.length + 2))}</span></li>`).join('') + '</ul>';
  }

  #renderPeople() {
    const box = this.dialog?.querySelector('[data-people]');
    const alone = (this.group?.size() || 0) < 2;
    if (box && box.hidden === alone) box.innerHTML = this.#peopleHtml();
    else if (this.dialog?.open && this.active) {
      if (this.view?.kind === 'start') this.view.kind = 'session';
      this.#render();
      if (this.current) this.#inviteStatus(this.current);
    }
  }

  #inviteHtml() {
    const inv = this.current && this.invites.get(this.current.id) === this.current ? this.current : null;
    if (!inv || inv.state === 'open') {
      const full = (this.group?.size() || 0) >= MAX_PEERS;
      return `<div class="cb-row"><button type="button" class="d2-btn${inv ? '' : ' primary'}" data-act="invite"${full ? ' disabled' : ''}>Invite someone</button>` +
        `${full ? '<span class="cb-muted">This session is full (8 people).</span>' : ''}</div>` +
        (inv ? '<p class="cb-status ok" data-status></p>' : '');
    }
    const program = this.api.binary()?.name || 'the program';
    return '<div class="cb-step"><h3>1. Send this link to one person</h3>' + this.#copyRow(inv.link, 'Invite link') +
      `<p class="cb-small">Whoever opens it receives a copy of ${escapeHtml(program)} from your browser.` +
      '<span class="d2-teach"> It lets one person in: make another link for each person you invite.</span></p></div>' +
      '<div class="cb-step"><h3>2. Open the reply link they send back</h3>' +
      '<p class="cb-small">Click it (it opens in this browser and connects by itself), or paste it here:</p>' +
      `<form class="cb-copy" data-form="reply" data-paste${inv.state === 'waiting' ? '' : ' hidden'}>` +
      '<input class="d2-input" name="reply" placeholder="Paste the reply link" autocomplete="off" spellcheck="false" aria-label="Reply link">' +
      '<button class="d2-btn" type="submit">Connect</button></form></div>' +
      '<p class="cb-status" data-status></p>' +
      '<div class="cb-row"><button type="button" class="d2-btn small" data-act="invite">Make another link</button></div>';
  }

  #joinView(v) {
    const j = this.join;
    const inv = v.inv || j?.inv;
    const from = inv ? escapeHtml(inv.n) : 'Someone';
    const title = `Join ${inv ? inv.n : 'the'}'s session`;
    if (!j) {
      const body = `<p><b>${from}</b> invites you to work on <b>${escapeHtml(inv.f)}</b> (${sizeWords(inv.z)}) together.</p>` +
        `<p class="cb-note">When you join, ${from}'s browser sends you a copy of the program and it opens here.</p>` +
        (this.active ? '<p class="cb-note">You will leave the session you are in now.</p>' : '') +
        `<form data-form="join">${this.#nameField()}<div class="cb-row"><button type="button" class="d2-btn" data-act="close">Not now</button>` +
        '<button class="d2-btn primary" type="submit">Join</button></div></form>';
      return { title, body };
    }
    let body = '';
    if (j.state === 'failed') {
      body = `<p class="cb-status err">${escapeHtml(j.error || NETWORK)}</p><div class="cb-row"><button type="button" class="d2-btn primary" data-act="close" autofocus>OK</button></div>`;
    } else if (j.state === 'reply') {
      body = `<div class="cb-step"><h3>Send this reply link back to ${from}</h3>${this.#copyRow(j.reply, 'Reply link')}</div>` +
        `<p class="cb-small">Send it the way the invite reached you. When ${from} opens it, you are connected. Keep this page open until then.</p>` +
        `<p class="cb-status ${j.stuck ? 'err' : 'busy'}" data-status>${j.stuck ? NETWORK : `Waiting for ${from} to open your reply link…`}</p>`;
    } else {
      const words = {
        connecting: ['Connecting…', 'busy'],
        joining: [`Connected to ${inv ? inv.n : 'the session'}. Joining…`, 'busy'],
        receiving: [`Receiving ${j.file?.name || 'the program'}…`, 'busy'],
        merging: ['Joining…', 'busy'],
        opening: [`Opening ${j.file?.name || 'the program'}…`, 'busy'],
      }[j.state] || ['Joining…', 'busy'];
      body = `<p class="cb-status ${words[1]}" data-status>${escapeHtml(words[0])}</p>` +
        (j.state === 'receiving' ? `<div class="cb-bar"><span data-progress style="width:${j.size ? Math.round((j.got / j.size) * 100) : 0}%"></span></div>` : '');
    }
    return { title, body };
  }

  #replyView(v) {
    const guest = escapeHtml(v.reply.n);
    const words = {
      handing: [`Handing ${guest}'s reply to your Kuna tab…`, 'busy'],
      connecting: [`Connecting to ${guest}…`, 'busy'],
      open: ['Connected — you can close this tab.', 'ok'],
      failed: [escapeHtml(v.text || NETWORK), 'err'],
    }[v.state];
    if (v.state === 'nobody') {
      return {
        title: `${v.reply.n}'s reply`,
        body: '<p>No Kuna tab in this browser has this invite open. If you made the invite in this browser, its tab was closed or reloaded: make a new invite link. Otherwise open this reply link in the browser you sent the invite from.</p>' +
          this.#copyRow(`${this.#base()}#reply=${v.code}`, 'Reply link'),
      };
    }
    return { title: `${v.reply.n}'s reply`, body: `<p class="cb-status ${words[1]}" data-status>${words[0]}</p>` };
  }

  async #click(e) {
    if (e.target === this.dialog) {
      this.close();
      return;
    }
    const act = e.target.closest('[data-act]')?.dataset.act;
    if (!act) return;
    if (act === 'close') this.close();
    else if (act === 'copy') this.#copy(e.target.closest('.cb-copy')?.querySelector('[data-copytext]'));
    else if (act === 'leave') {
      this.leave();
      this.close();
    } else if (act === 'invite') {
      const inv = await this.makeInvite();
      if (inv) {
        this.view = { kind: 'session' };
        this.#render();
        this.#inviteStatus(inv);
        this.dialog.querySelector('[data-copytext]')?.select();
      }
    }
  }

  async #copy(input) {
    if (!input) return;
    input.select();
    try {
      await navigator.clipboard.writeText(input.value);
      this.api.toast('Copied the link.');
    } catch (_) {
      this.api.toast('Press Ctrl+C (or ⌘C) to copy the selected link.', { kind: 'warn' });
    }
  }

  async #submit(e) {
    e.preventDefault();
    const form = e.target.closest('form');
    const kind = form?.dataset.form;
    if (kind === 'start' || kind === 'join') {
      const name = plainName(form.elements.name.value);
      if (!name) {
        form.elements.name.focus();
        return;
      }
      this.name = name;
      this.#savePrefs();
      if (kind === 'join') {
        await this.#joinNow(this.view.inv);
        return;
      }
      this.view = { kind: 'session' };
      const inv = await this.makeInvite();
      this.#render();
      if (inv) {
        this.#inviteStatus(inv);
        this.dialog.querySelector('[data-copytext]')?.select();
      }
    } else if (kind === 'reply') {
      const res = await this.applyReply(form.elements.reply.value);
      if (!res.ok) this.#status(res.text, 'err');
    }
  }
}
