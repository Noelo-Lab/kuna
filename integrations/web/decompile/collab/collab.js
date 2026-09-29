// collab.js — "Work together", loaded by the study view only when a session
// starts or an invite or reply link is opened: the invite, join and reply
// dialogs, who is here in the top bar, following, and the others' pointers
// and pings. The data half is sync.js (the page's Session and the group's
// registers kept in step, joining and leaving), which runs the protocol
// (group.js) over the connections link.js makes; presence.js draws the
// pointers.
import { escapeHtml } from '../../assets/js/highlight-c.js';
import { bare } from '../addr.js';
import {
  MAX_PEERS, encodeCode, decodeCode, codeFrom, randomId, initials, limiter, cleanName, describeFile, quietly,
} from './wire.js';
import { Sync } from './sync.js';
import { makeOffer, takeOffer, holdPresenceLock } from './link.js';
import { createPresence, anchorAt, findAnchor } from './presence.js';

export const PREFS_KEY = 'kuna.d2.collab';
const STUN = 'stun:stun.l.google.com:19302';
const JOIN_MS = 20000;
const VIEW_WORDS = { c: 'Code', split: 'Side by side', asm: 'Assembly', bytes: 'Bytes', stack: 'Stack' };
const EFFORTS = { auto: 'Automatic', fast: 'Fast', reliable: 'Reliable', aggressive: 'Thorough' };
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

const sizeWords = (n) => (n < 1024 ? `${n} bytes` : n < 1 << 20 ? `${Math.round(n / 1024)} KB` : `${(n / (1 << 20)).toFixed(1)} MB`);
const statusLine = (text, cls = '') => `<p class="cb-status ${cls}" data-status>${escapeHtml(text || '')}</p>`;

export async function createCollab(api) {
  const collab = new Collab(api);
  collab.init();
  return collab;
}

class Collab {
  constructor(api) {
    this.api = api;
    this.me = randomId(8);
    this.name = cleanName(loadCollabPrefs(api.storage).name);
    this.sync = new Sync({ me: this.me, app: this.#syncApp(), ui: this.#syncUi() });
    this.invites = new Map();
    this.closedInvites = new Set();
    this.current = null;
    this.join = null;
    this.seen = new Map();
    this.following = null;
    this.whereKey = null;
    this.lastFn = null;
    this.desc = null;
    this.descBytes = null;
    this.pointer = null;
    this.pointerAt = 0;
    this.pointerTimer = 0;
    this.pointerSent = false;
    this.pingLimit = limiter(1, 2);
    this.dialog = null;
    this.view = null;
    this.statusNow = null;
    this.leftForPage = false;
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
      if (this.following && !e.target.closest?.('.d2-who, #d2collab')) this.#follow(null);
    }, { capture: true });
    document.addEventListener('keydown', (e) => {
      if (this.following && !['Shift', 'Control', 'Alt', 'Meta', 'Tab'].includes(e.key)) this.#follow(null);
    }, { capture: true });
    addEventListener('pagehide', (e) => {
      if (!e.persisted) {
        this.group?.leave();
        return;
      }
      if (!this.active && !this.sync.joining) return;
      this.leftForPage = true;
      this.leave({ quiet: true });
    });
    addEventListener('pageshow', (e) => {
      if (!e.persisted || !this.leftForPage) return;
      this.leftForPage = false;
      this.api.toast('You left the session when you went to another page.', { kind: 'warn', detail: 'Ask for a new invite link to join again.' });
    });
    this.api.onBuild((build) => {
      if (this.group && this.group.build !== build) {
        this.leave({ why: 'This page\'s decompiler was updated (the site changed), so it left the session. Reload the page, then join again.' });
      }
    });
    if (globalThis.BroadcastChannel) {
      this.replyChannel = new BroadcastChannel('kuna.d2.reply');
      this.replyChannel.onmessage = ({ data }) => this.#handOff(data);
    }
  }

  get active() {
    return this.sync.active;
  }

  get group() {
    return this.sync.group;
  }

  get shared() {
    return this.sync.shared;
  }

  /** What sync.js needs from the page: the study view's api, and the program as peers describe it. */
  #syncApp() {
    return { ...this.api, fileMeta: () => this.#fileMetaNow() };
  }

  /** What sync.js tells this page's dialogs, roster and pointers. */
  #syncUi() {
    return {
      nameOf: (peer) => this.#nameOf(peer),
      remember: (peer) => this.#remember(peer),
      welcomed: (info) => this.#welcomed(info),
      fileProgress: (got, size) => this.#progress(got, size),
      fileOpening: () => {
        if (!this.join) return;
        this.join.state = 'opening';
        this.#render();
      },
      joined: (info) => this.#joinDone(info),
      joinFailed: (reason, info) => this.#joinFailed(this.#failText(reason, info)),
      mergeReplaced: ({ count, apart = 0, mode = null, copy }) => {
        const replaced = `The session's changes replaced ${count} of yours, where you both changed the same thing.`;
        const title = apart
          ? `${apart === 1 ? 'One of your variables was' : `${apart} of your variables were`} changed at another decompiler effort, so ${apart === 1 ? 'it stays' : 'they stay'} out of the session.`
          : replaced;
        const detail = apart
          ? `The session runs at ${EFFORTS[mode] || mode}, where the same variable names can be other variables.${count ? ` ${replaced}` : ''} Everything else you had changed is now part of the session.`
          : 'Everything else you had changed is now part of the session.';
        this.api.toast(title, { kind: 'warn', ms: 15000, detail, action: { label: 'Save yours as a file', run: () => this.api.exportSession(copy) } });
      },
      roster: () => this.#rosterChanged(),
      where: () => this.#whereOf(),
      cursor: (peer, m) => this.#cursor(peer, m),
      ping: (peer, m) => this.#pinged(peer, m),
      event: (kind, info) => this.#event(kind, info),
    };
  }

  ice() {
    return iceServersFrom(loadCollabPrefs(this.api.storage));
  }

  #savePrefs() {
    const p = loadCollabPrefs(this.api.storage);
    p.name = this.name;
    quietly(() => this.api.storage?.setItem(PREFS_KEY, JSON.stringify(p)));
  }

  /** The engine's build id (the SHA-256 of the wasm this page's Worker compiled). */
  async build() {
    const build = await this.api.build();
    if (!/^[0-9a-f]{64}$/.test(build || '')) throw new Error('this page could not tell which version of the decompiler it runs');
    return build;
  }

  /** How peers describe the open program: `{meta}`, `{problem}` when it cannot travel, or null with none open. */
  describe() {
    const bin = this.api.binary();
    if (!bin) return null;
    if (this.descBytes !== bin.bytes || !this.desc) {
      this.desc = describeFile(bin.name, bin.bytes.length, String(bin.hash || '').replace(/^sha256:/, ''));
      this.descBytes = bin.bytes;
    }
    return this.desc;
  }

  #fileMetaNow() {
    return this.describe()?.meta || null;
  }

  // ── the group ────────────────────────────────────────────────────────────

  /** How the group links two members through a third (an introduction). */
  #connect() {
    return {
      offer: () => makeOffer({ me: this.me, iceServers: this.ice() }),
      answer: ({ id, sdp }) => takeOffer({ me: this.me, id, sdp, iceServers: this.ice() }),
    };
  }

  /** Start a session as its first member, with this page's changes as its registers. */
  async start() {
    if (this.active) return true;
    const desc = this.describe();
    if (!desc) return false;
    if (desc.problem) throw new Error(`${this.api.binary().name} cannot be shared: ${desc.problem}`);
    const build = await this.build();
    if (this.active) return true;
    if (!this.describe() || this.describe().problem) return false;
    this.sync.start({ build, name: this.name, connect: this.#connect() });
    this.group.setWhere(this.#where());
    this.#rosterChanged();
    return true;
  }

  /** Leave: close every link and stop a join in progress. The page keeps what it shows (see sync.js). */
  leave({ quiet = false, why = null } = {}) {
    const was = this.active;
    for (const inv of this.invites.values()) {
      inv.offer?.cancel?.();
      this.closedInvites.add(inv.id);
    }
    this.invites.clear();
    this.current = null;
    this.#endJoin();
    this.sync.leave();
    this.following = null;
    this.presence.clear();
    this.#rosterChanged();
    if (!quiet && (was || why)) this.api.toast(why || 'You left the session. Its changes stay on this page.', { kind: why ? 'err' : 'ok' });
  }

  /** Before opening another program: leaving (or stopping a join) is the only way; ask first. */
  async confirmLeave(name) {
    const joining = this.join && !['failed', 'done'].includes(this.join.state);
    if (!this.active && !joining) return true;
    const ok = await this.api.confirm(joining
      ? `You are joining ${this.join.inv?.n || 'someone'}'s session. Opening ${name} stops that. Open it anyway?`
      : `Opening ${name} ends your part in this session. Leave the session and open it?`, joining ? 'Stop and open' : 'Leave and open');
    if (ok) this.leave({ quiet: true });
    return ok;
  }

  // ── this page's edits ────────────────────────────────────────────────────

  /** The page's session changed (an edit, an undo, a byte burst). */
  localChanged() {
    this.sync.local();
  }

  modeChanged(mode) {
    this.sync.modeChanged(mode);
  }

  get canUndo() { return this.sync.canUndo; }

  get canRedo() { return this.sync.canRedo; }

  undo() { return this.sync.undo(); }

  redo() { return this.sync.redo(); }

  // ── inviting ─────────────────────────────────────────────────────────────

  /** Make a one-person invite link (starting a session if there is none). */
  async makeInvite() {
    let offer = null;
    try {
      if (!this.name) throw new Error('type your name first');
      if (!(await this.start())) throw new Error('open a program first');
      if (this.group.size() >= MAX_PEERS) throw new Error('this session is full (8 people)');
      this.#status('Making an invite link…', 'busy');
      offer = await makeOffer({ me: this.me, iceServers: this.ice() });
      if (!this.active) throw new Error('the session ended');
      const file = this.#fileMetaNow();
      if (!file) throw new Error('open a program first');
      const code = await encodeCode('invite', { id: offer.id, n: this.name, f: file.name, z: file.size, d: offer.sdp });
      if (!this.active) throw new Error('the session ended');
      const inv = { id: offer.id, offer, link: `${this.#base()}#join=${code}`, state: 'waiting', guest: null };
      this.invites.set(offer.id, inv);
      offer.ready.then((link) => {
        if (!this.active) {
          link.close();
          return;
        }
        inv.state = 'linked';
        this.sync.addLink(link, inv.id);
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
    } catch (e) {
      offer?.cancel?.();
      this.#status(`Could not make an invite link: ${e.message}.`, 'err');
      return null;
    }
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

  /** Another tab of this browser opened a reply link: take it if the invite is (or was) this page's. */
  async #handOff(data) {
    if (data?.k !== 'reply' || typeof data.id !== 'string' || typeof data.code !== 'string' || typeof data.from !== 'string') return;
    const inv = this.invites.get(data.id);
    const post = (msg) => quietly(() => this.replyChannel.postMessage({ ...msg, id: data.id, to: data.from }));
    if (!inv || inv.state !== 'waiting') {
      if (inv || this.closedInvites.has(data.id)) post({ k: 'used' });
      return;
    }
    post({ k: 'got' });
    inv.replyTab = data.from;
    const res = await this.applyReply(data.code);
    if (!res.ok) post({ k: 'state', state: 'failed', text: res.text });
  }

  #tellReplyTab(inv, state) {
    if (!inv.replyTab) return;
    quietly(() => this.replyChannel?.postMessage({ k: 'state', id: inv.id, to: inv.replyTab, state }));
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
    if (this.invites.has(inv.id) || this.closedInvites.has(inv.id)) {
      this.#show({ kind: 'notice', title: 'This is your own invite link', text: 'Send it to the person you want to invite. When they open it, they get a reply link to send back to you.' });
      return;
    }
    if (this.join?.state === 'failed') this.join = null;
    this.#show({ kind: 'join', inv, replacing: this.join && this.join.inv !== inv ? this.join : null });
  }

  async #joinNow(inv) {
    this.#endJoin();
    if (this.active) this.leave({ quiet: true });
    const join = { inv, state: 'connecting' };
    this.join = join;
    this.view = { kind: 'join', inv };
    this.#render();
    const [built, taken] = await Promise.allSettled([this.build(), takeOffer({ me: this.me, id: inv.id, sdp: inv.d, iceServers: this.ice() })]);
    const res = taken.status === 'fulfilled' ? taken.value : null;
    if (built.status === 'rejected' || taken.status === 'rejected' || this.join !== join) {
      res?.cancel?.();
      if (this.join !== join) return;
      if (taken.status === 'rejected') {
        this.#joinFailed(taken.reason?.message === 'used' ? 'This invite link was already used: each link lets one person in. Ask for a new one.'
          : `This browser could not make a connection: ${taken.reason?.message}.`);
      } else this.#joinFailed(`Could not join: ${built.reason?.message}.`);
      return;
    }
    const build = built.value;
    join.res = res;
    if (res.sdp === null) {
      res.ready.then((link) => this.#joined(join, link, build));
      return;
    }
    let code;
    try {
      code = await encodeCode('reply', { id: inv.id, n: this.name, d: res.sdp });
    } catch (e) {
      this.#joinFailed(`Could not make a reply link: ${e.message}.`);
      return;
    }
    if (this.join !== join) return;
    join.reply = `${this.#base()}#reply=${code}`;
    join.state = 'reply';
    res.onstate = (s) => {
      if (s === 'failed' && this.join === join && join.state === 'reply') {
        join.stuck = true;
        this.#render();
      }
    };
    res.ready.then((link) => { if (this.join === join) this.#joined(join, link, build); else link.close(); }).catch(() => {});
    this.#render();
  }

  #joined(join, link, build) {
    if (this.join !== join) {
      link.close();
      return;
    }
    join.state = 'joining';
    this.sync.beginJoin({ build, name: this.name, connect: this.#connect(), link });
    join.timer = setTimeout(() => {
      if (this.join === join && join.state === 'joining') this.#joinFailed(`${join.inv.n}'s page did not answer. Ask for a new invite link.`);
    }, JOIN_MS);
    this.#render();
  }

  /** Stop a join in progress (its connection closes). */
  #endJoin() {
    const j = this.join;
    if (!j) return;
    clearTimeout(j.timer);
    if (j.state !== 'done') j.res?.cancel?.();
    if (this.sync.joining) this.sync.stop();
    this.join = null;
  }

  #joinFailed(text) {
    const j = this.join;
    if (this.sync.joining) this.sync.stop();
    if (j) {
      clearTimeout(j.timer);
      j.res?.cancel?.();
      j.state = 'failed';
      j.error = text;
      this.join = j;
    }
    this.#rosterChanged();
    if (this.dialog?.open && this.view?.kind === 'join') this.#render();
    else this.api.toast(text, { kind: 'err' });
  }

  /** Why a join failed, in words (`reason` from sync.js). */
  #failText(reason, info = {}) {
    const who = info.name || this.join?.sponsorName || this.join?.inv?.n || 'The other person';
    switch (reason) {
      case 'full': return 'This session is full (8 people).';
      case 'mismatch': return `${who}'s page is a different version of Kuna; reload both.`;
      case 'closed': return `${who}'s page closed the connection before you joined. Ask for a new invite link.`;
      case 'sponsor': return `${who}'s page closed the connection before you finished joining. Ask for a new invite link.`;
      case 'stalled': return `${who}'s page stopped answering while you were joining. Ask for a new invite link.`;
      case 'hash': return 'The program did not arrive intact. Ask for a new invite link and try again.';
      case 'lost': return 'The connection closed while the program was on its way. Ask for a new invite link and try again.';
      case 'open': return `Could not open ${info.name || 'the program'}.`;
      default: return 'The program on this page changed while you were joining. Ask for a new invite link.';
    }
  }

  #welcomed({ from, name, file, send }) {
    const j = this.join;
    if (!j) return;
    clearTimeout(j.timer);
    Object.assign(j, { sponsor: from, sponsorName: name, file, send, state: send ? 'receiving' : 'merging' });
    this.group.setWhere(this.#where());
    this.#rosterChanged();
    this.#render();
  }

  #progress(got, size) {
    const j = this.join;
    if (!j) return;
    j.got = got;
    j.size = size;
    const bar = this.dialog?.querySelector('[data-progress]');
    if (bar) {
      bar.style.width = `${Math.round((got / size) * 100)}%`;
      this.#setStatus(`Receiving ${j.file?.name || 'the program'}… ${Math.round((got / size) * 100)}%`, 'busy');
    }
  }

  /** sync.js finished the join: go where the inviter is, and say so. */
  #joinDone({ sponsor, aside }) {
    const j = this.join;
    if (!j || j.state === 'done') return;
    j.state = 'done';
    clearTimeout(j.timer);
    const where = this.group?.who(sponsor)?.where;
    if (where?.fn && this.api.target() && where.fn !== this.api.target()) this.api.openFunction(where.fn);
    this.api.toast(`You joined ${j.sponsorName}'s session.`, { detail: 'Their changes and yours now appear on every page.' });
    if (aside) {
      this.api.toast(`Your own ${aside.count} change${aside.count === 1 ? '' : 's'} to ${aside.name} are kept as they were.`, {
        ms: 10000, detail: 'This session is saved apart from them, and they come back when you leave. The session dialog can save them as a file.',
      });
    }
    if (this.view?.kind === 'join') this.close();
    this.join = null;
    this.group?.setWhere(this.#where());
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
    return peer === this.me ? 'You' : this.#member(peer).name;
  }

  #member(peer) {
    const m = this.group?.who(peer);
    return m ? { name: m.name, color: m.color } : this.seen.get(peer) || { name: 'Someone', color: '#a39894' };
  }

  /** Where this page is, as the others see it: the function on screen (not one still opening), and the view. */
  #where() {
    return this.api.current();
  }

  whereChanged() {
    const shown = this.api.current().fn;
    this.presence.setFunction(shown);
    const w = this.#where();
    const key = `${w.fn}|${w.view}`;
    if (key === this.whereKey) return;
    const moved = this.lastFn !== w.fn;
    this.whereKey = key;
    this.lastFn = w.fn;
    if (this.group?.active) {
      this.group.setWhere(w);
      if (moved) this.#pointerOff();
    }
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
      this.presence.hide(peer);
      return;
    }
    this.presence.show(peer, m, this.#member(peer));
    if (this.following === peer && m.fn === this.api.current().fn && !this.presence.visible(m.anchor)) {
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
    this.#renderRoster();
    if (!peer) return;
    const m = this.group?.who(peer);
    if (!m?.where?.fn) {
      this.api.toast(`${m?.name || 'They'} ${m?.linked === false ? 'are still connecting' : 'have not opened a function yet'}.`);
      this.following = null;
      this.#renderRoster();
      return;
    }
    this.#followTo(m);
    this.api.toast(`Following ${m.name}. Click anywhere or press a key to stop.`);
  }

  /** Open where `m` is, unless this page is already there or on its way. */
  #followTo(m) {
    if (m.where.fn !== this.api.target()) this.api.openFunction(m.where.fn, { view: m.where.view });
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
    const members = this.group?.members() || [];
    if (this.following && !members.some((m) => m.peer === this.following)) this.following = null;
    for (const m of members) this.seen.set(m.peer, { name: m.name, color: m.color });
    this.presence.keepOnly(new Set(members.filter((m) => m.linked).map((m) => m.peer)));
    this.#renderRoster();
    this.api.refresh();
    if (this.dialog?.open && (this.view?.kind === 'session' || this.view?.kind === 'start')) this.#renderPeople();
  }

  #whereWords(m) {
    if (!m.linked) return `${m.name}: connecting…`;
    if (!m.where?.fn) return `${m.name}: no function open yet`;
    return `${m.name}: ${this.api.nameOf(m.where.fn)}, ${VIEW_WORDS[m.where.view] || 'Code'}`;
  }

  /** The people in the top bar, updated in place so a focused one keeps focus. */
  #renderRoster() {
    const { els } = this.api;
    if (!this.rosterEl) {
      this.rosterEl = document.createElement('div');
      this.rosterEl.className = 'd2-roster';
      this.rosterEl.id = 'd2roster';
      this.rosterEl.setAttribute('role', 'group');
      this.rosterEl.setAttribute('aria-label', 'People working together');
      els.hintsBox.closest('label').before(this.rosterEl);
      this.together = document.createElement('button');
      this.together.className = 'd2-btn ghost d2-together';
      this.together.dataset.act = 'collab-open';
      this.together.title = 'Working together: invite someone, or leave';
      this.together.innerHTML = `${PEOPLE}<span data-count></span><span class="d2-sr"> people</span>`;
      this.rosterEl.appendChild(this.together);
      this.rosterEl.addEventListener('click', (e) => {
        const chip = e.target.closest('.d2-who');
        if (chip) this.#follow(this.following === chip.dataset.peer ? null : chip.dataset.peer);
        else if (e.target.closest('[data-act=collab-open]')) this.open();
      });
    }
    const members = this.group?.active ? this.group.members() : [];
    this.rosterEl.hidden = !members.length;
    const chips = new Map([...this.rosterEl.querySelectorAll('.d2-who')].map((b) => [b.dataset.peer, b]));
    for (const m of members.filter((x) => !x.me)) {
      let b = chips.get(m.peer);
      chips.delete(m.peer);
      if (!b) {
        b = document.createElement('button');
        b.dataset.peer = m.peer;
      }
      const on = this.following === m.peer;
      const cls = `d2-who${on ? ' on' : ''}${m.linked ? '' : ' wait'}`;
      if (b.className !== cls) b.className = cls;
      b.style.setProperty('--who', m.color);
      const title = `${this.#whereWords(m)}\n${on ? 'Following: click to stop' : 'Click to follow'}`;
      if (b.title !== title) b.title = title;
      b.setAttribute('aria-pressed', String(on));
      const text = initials(m.name);
      if (b.textContent !== text) b.textContent = text;
      if (b.parentElement !== this.rosterEl) this.together.before(b);
    }
    for (const b of chips.values()) b.remove();
    const count = this.together.querySelector('[data-count]');
    if (count.textContent !== String(members.length)) count.textContent = String(members.length);
  }

  /** The author of a record in the Changes list (whoever wrote its newest register): `{name, color, me}` or null. */
  authorOf(key) {
    const clock = this.sync.clockOf(key);
    if (!clock) return null;
    const mine = clock[1] === this.me;
    const who = mine ? { name: this.name, color: this.group?.color || this.#member(this.me).color } : this.#member(clock[1]);
    return { name: who.name, color: who.color, me: mine };
  }

  /** Someone joined, left or was lost (a join's own failures come through #joinFailed). */
  #event(kind, info) {
    const name = info?.name || 'Someone';
    switch (kind) {
      case 'joined': {
        this.api.toast(`${name} joined.`);
        const inv = info.invite && this.invites.get(info.invite);
        if (inv) {
          inv.state = 'open';
          inv.guest = name;
          this.#inviteStatus(inv);
        }
        break;
      }
      case 'unjoined': {
        const inv = info.invite && this.invites.get(info.invite);
        if (inv && inv.state !== 'open') {
          inv.state = 'failed';
          inv.error = `${inv.guest || 'The other person'} did not finish joining. Make a new invite link.`;
          this.#inviteStatus(inv);
        }
        break;
      }
      case 'left':
        this.api.toast(`${name} left the session.`);
        this.presence.forget(info.peer);
        break;
      case 'lost':
        this.api.toast(`Lost the connection to ${name}.`, { kind: 'warn', detail: info.reachable ? 'Trying again through the others…' : '' });
        this.presence.forget(info.peer);
        break;
      case 'mismatch':
        this.api.toast(`${name}'s page is a different version of Kuna; reload both.`, { kind: 'err' });
        break;
      case 'misbehaved':
        this.api.toast(`${name}'s page sent changes this page cannot accept, so it stopped linking to it.`, {
          kind: 'err', detail: 'Its changes were numbered far faster than anyone edits. The others\' pages may still show it.',
        });
        this.presence.forget(info.peer);
        break;
      default: break;
    }
    this.#rosterChanged();
  }

  // ── the dialog ───────────────────────────────────────────────────────────

  /** Collaborate: invite (or, in a session, who is here, invite more, leave). */
  open() {
    if (this.join?.state === 'failed') this.join = null;
    this.#show({ kind: this.join ? 'join' : this.active ? 'session' : 'start', inv: this.join?.inv });
  }

  close() {
    this.view = null;
    if (this.dialog?.open) this.dialog.close();
  }

  #show(view) {
    this.view = view;
    this.statusNow = null;
    if (!this.dialog) {
      this.dialog = document.createElement('dialog');
      this.dialog.id = 'd2collab';
      this.dialog.className = 'd2-collab';
      this.dialog.setAttribute('aria-labelledby', 'd2collabtitle');
      document.body.appendChild(this.dialog);
      this.dialog.addEventListener('keydown', (e) => e.stopPropagation());
      this.dialog.addEventListener('click', (e) => this.#click(e));
      this.dialog.addEventListener('submit', (e) => this.#submit(e));
      this.dialog.addEventListener('close', () => {
        this.view = null;
        if (this.join?.state === 'failed') this.join = null;
      });
    }
    this.#render();
    if (!this.dialog.open) this.dialog.showModal();
    const first = this.dialog.querySelector('[autofocus]');
    first?.focus();
    if (first?.select) first.select();
  }

  /** The dialog's status line (it keeps it across re-renders). */
  #status(text, cls = '') {
    this.statusNow = { text, cls };
    this.#setStatus(text, cls);
  }

  #setStatus(text, cls = '') {
    const el = this.dialog?.querySelector('[data-status]');
    if (!el) return;
    el.textContent = text;
    el.className = `cb-status ${cls}`;
  }

  #inviteStatus(inv) {
    if (this.current !== inv) return;
    const [text, cls] = {
      waiting: ['Waiting for their reply link…', 'busy'],
      connecting: [`Connecting to ${inv.guest || 'them'}…`, 'busy'],
      linked: [`Connected. ${inv.guest || 'They'} are joining…`, 'busy'],
      open: [`${inv.guest || 'They'} joined.`, 'ok'],
      failed: [inv.error || NETWORK, 'err'],
    }[inv.state] || ['', ''];
    this.#status(text, cls);
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
        statusLine(this.statusNow?.text, this.statusNow?.cls) +
        '<p class="cb-small d2-teach">It works between tabs of this browser and between computers on the same network. Nothing is sent to a server.</p>';
    } else {
      const alone = this.group.size() < 2;
      title = alone ? 'Invite someone' : `Working together on ${bin ? bin.name : 'a program'}`;
      body = `<div data-people${alone ? ' hidden' : ''}>${this.#peopleHtml()}</div>${this.#inviteHtml()}${this.#asideHtml()}` +
        '<p class="cb-small d2-teach">Everyone can change anything. Undo takes back only your own changes. Alt+click a line (or press P) to point the others to it.</p>' +
        '<div class="cb-row cb-end"><button type="button" class="d2-btn" data-act="leave">Leave the session</button>' +
        '<button type="button" class="d2-btn primary" data-act="close">Done</button></div>';
    }
    this.dialog.innerHTML = '<div class="cb-head">' +
      `<h2 id="d2collabtitle">${escapeHtml(title)}</h2>` +
      '<button type="button" class="d2-iconbtn small" data-act="close" aria-label="Close" title="Close">×</button></div>' +
      `<div class="cb-body">${body}</div>`;
    if (v.kind === 'session' && this.current && this.invites.get(this.current.id) === this.current) this.#inviteStatus(this.current);
  }

  #peopleHtml() {
    const members = this.group?.members() || [];
    return '<ul class="cb-people">' + members.map((m) => `<li style="--who:${m.color}"><span class="cb-dot"></span>` +
      `<span class="cb-name">${escapeHtml(m.name)}${m.me ? ' <span class="cb-muted">(you)</span>' : ''}</span>` +
      `<span class="cb-muted">${escapeHtml(m.me ? '' : this.#whereWords(m).slice(m.name.length + 2))}</span>` +
      (!m.me && m.where?.fn ? `<button type="button" class="d2-link cb-follow" data-act="follow" data-peer="${m.peer}">Follow</button>` : '') +
      '</li>').join('') + '</ul>';
  }

  #renderPeople() {
    const box = this.dialog?.querySelector('[data-people]');
    const alone = (this.group?.size() || 0) < 2;
    if (box && box.hidden === alone) box.innerHTML = this.#peopleHtml();
    else if (this.dialog?.open && this.active) {
      if (this.view?.kind === 'start') this.view.kind = 'session';
      this.#render();
    }
  }

  #inviteHtml() {
    const inv = this.current && this.invites.get(this.current.id) === this.current ? this.current : null;
    if (!inv || inv.state === 'open') {
      const full = (this.group?.size() || 0) >= MAX_PEERS;
      return `<div class="cb-row"><button type="button" class="d2-btn${inv ? '' : ' primary'}" data-act="invite"${full ? ' disabled' : ''}>Invite someone</button>` +
        `${full ? '<span class="cb-muted">This session is full (8 people).</span>' : ''}</div>` +
        statusLine(this.statusNow?.text, this.statusNow?.cls);
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
      statusLine(this.statusNow?.text, this.statusNow?.cls) +
      '<div class="cb-row"><button type="button" class="d2-btn small" data-act="invite">Make another link</button></div>';
  }

  /** While in a session joined with the program: this page's own earlier changes to it, kept apart. */
  #asideHtml() {
    const a = this.sync.aside;
    if (!a) return '';
    return `<p class="cb-note">Your own ${a.count} change${a.count === 1 ? '' : 's'} to ${escapeHtml(a.name)} from before are kept as they were; they come back when you leave. ` +
      '<button type="button" class="d2-link" data-act="save-aside">Save them as a file</button></p>';
  }

  #joinView(v) {
    const j = this.join && this.join.inv === v.inv ? this.join : null;
    const inv = v.inv || j?.inv;
    const from = inv ? inv.n : 'Someone';
    const title = `Join ${from}'s session`;
    if (!j) {
      const was = v.replacing && v.replacing.state !== 'failed' ? v.replacing.inv?.n : null;
      const body = `<p><b>${escapeHtml(from)}</b> invites you to work on <b>${escapeHtml(inv.f)}</b> (${sizeWords(inv.z)}) together.</p>` +
        `<p class="cb-note">When you join, ${escapeHtml(from)}'s browser sends you a copy of the program and it opens here.</p>` +
        (this.active ? '<p class="cb-note">You will leave the session you are in now.</p>' : '') +
        (was ? `<p class="cb-note">You were joining ${escapeHtml(was)}'s session; joining this one stops that.</p>` : '') +
        `<form data-form="join">${this.#nameField()}<div class="cb-row"><button type="button" class="d2-btn" data-act="close">Not now</button>` +
        '<button class="d2-btn primary" type="submit">Join</button></div></form>';
      return { title, body };
    }
    if (j.state === 'failed') {
      return { title, body: statusLine(j.error || NETWORK, 'err') + '<div class="cb-row"><button type="button" class="d2-btn primary" data-act="close" autofocus>OK</button></div>' };
    }
    if (j.state === 'reply') {
      return {
        title,
        body: `<div class="cb-step"><h3>Send this reply link back to ${escapeHtml(from)}</h3>${this.#copyRow(j.reply, 'Reply link')}</div>` +
          `<p class="cb-small">Send it the way the invite reached you. When ${escapeHtml(from)} opens it, you are connected. Keep this page open until then.</p>` +
          statusLine(j.stuck ? NETWORK : `Waiting for ${from} to open your reply link…`, j.stuck ? 'err' : 'busy'),
      };
    }
    const [text, cls] = {
      connecting: ['Connecting…', 'busy'],
      joining: [`Connected to ${from}. Joining…`, 'busy'],
      receiving: [`Receiving ${j.file?.name || 'the program'}…`, 'busy'],
      merging: ['Joining…', 'busy'],
      opening: [`Opening ${j.file?.name || 'the program'}…`, 'busy'],
    }[j.state] || ['Joining…', 'busy'];
    return {
      title,
      body: statusLine(text, cls) +
        (j.state === 'receiving' ? `<div class="cb-bar"><span data-progress style="width:${j.size ? Math.round((j.got / j.size) * 100) : 0}%"></span></div>` : ''),
    };
  }

  #replyView(v) {
    const title = `${v.reply.n}'s reply`;
    if (v.state === 'nobody') {
      return {
        title,
        body: '<p>No Kuna tab in this browser has this invite open. If you made the invite in this browser, its tab was closed or reloaded: make a new invite link. Otherwise open this reply link in the browser you sent the invite from.</p>' +
          this.#copyRow(`${this.#base()}#reply=${v.code}`, 'Reply link'),
      };
    }
    const [text, cls] = {
      handing: [`Handing ${v.reply.n}'s reply to your Kuna tab…`, 'busy'],
      connecting: [`Connecting to ${v.reply.n}…`, 'busy'],
      open: ['Connected — you can close this tab.', 'ok'],
      failed: [v.text || NETWORK, 'err'],
    }[v.state];
    return { title, body: statusLine(text, cls) };
  }

  async #click(e) {
    if (e.target === this.dialog) {
      this.close();
      return;
    }
    const act = e.target.closest('[data-act]')?.dataset.act;
    if (!act) return;
    if (act === 'close') this.close();
    else if (act === 'follow') {
      this.close();
      this.#follow(e.target.closest('[data-peer]').dataset.peer);
    } else if (act === 'copy') this.#copy(e.target.closest('.cb-copy')?.querySelector('[data-copytext]'));
    else if (act === 'save-aside') {
      const own = this.sync.aside && this.api.ownSession(this.sync.aside.hash);
      if (own) this.api.exportSession(own);
    } else if (act === 'leave') {
      this.leave();
      this.close();
    } else if (act === 'invite') {
      const inv = await this.makeInvite();
      if (this.view?.kind === 'start') this.view.kind = 'session';
      this.#render();
      if (inv) this.dialog.querySelector('[data-copytext]')?.select();
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
      const name = cleanName(form.elements.name.value);
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
      const inv = await this.makeInvite();
      if (this.active) this.view = { kind: 'session' };
      this.#render();
      if (inv) this.dialog.querySelector('[data-copytext]')?.select();
    } else if (kind === 'reply') {
      const res = await this.applyReply(form.elements.reply.value);
      if (!res.ok) this.#status(res.text, 'err');
    }
  }
}
