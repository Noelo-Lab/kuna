# decompile2-collab — analysis

The question: can several people work on one program in `/decompile2/` at the
same time, Google-Docs style — rename, retype, comment and patch together, ping
a line to draw the others' attention, and see each other's pointers — with no
server of ours? This file records what was measured; `proposal.md` is the
design built on it. The spikes are in `spike/`; each runs in seconds to a few
minutes against a built `integrations/web/dist/` (and `survey.py` against the
native `kuna` from `make binaries`):

| Spike | Shows |
|---|---|
| `engine.mjs` | the engine is deterministic and order-blind; the mode changes what a symbol means; the language does not; two directive forms read files |
| `survey.py` | how often the mode and the language change a symbol, over 503 functions |
| `lww.mjs`, `lww-test.mjs` | the replicated register map converges, and rejects hostile ops |
| `webrtc.mjs` | two Chrome processes connect with no server; code sizes, latency, throughput |

## 1. What has to be shared is already tiny

The page never shares C. Every wasm call is a fresh process that replays the
session's `--assert` directives (`decompile2/session.js`), so what the student
has changed *is* a small map of records, keyed by what they describe:

| Record key (session.js) | Changed by | Stable under renames? |
|---|---|---|
| `var:<fn addr>:<engine symbol>` (fields `name`, `type`) | rename, retype | yes: keyed by the function's address and the engine's own symbol (`v1`), not the display name |
| `fn:<addr>` | function rename | yes |
| `proto:<addr>` | signature, parameter rename/retype | yes |
| `data:<addr>`, `typedef:<tag>` | globals, types | yes |
| `comment:<fn addr>:<insn addr>` | notes | yes |
| one entry per patched byte (`bytes` map, addr → value) | byte patches | yes |
| `raw:<n>` | imported directives the page does not model | **no**: `n` is a per-page counter, so two peers collide |

If every peer holds the same map and runs the same wasm on the same bytes, every
peer prints the same C, and nothing else needs to travel. Three things have to
hold for that.

**The engine is deterministic and order-blind.** `spike/engine.mjs` runs the
shipped wasm under `node:wasi` in separate processes: `inspect main` with five
directives (a function rename, a prototype, a byte patch, a retype-and-rename,
a comment) in three different orders, plus a repeat run.

```
order: 1 document (sha256 a6d8fbd02abd7c12) for 5 directives in 3 orders across 4 processes
```

One document, byte for byte (the `assertions[]` outcome rows compared as a set).
So syncing the *set* is enough, as long as each peer sends it in one canonical
order anyway (cheap insurance against directives that do interact, e.g. two raw
ones). The same must hold across peers' copies of the page: a peer on an older
deploy of the wasm can print different C, so peers must compare a build id. The
SHA-256 of the `kuna_wasm.wasm` bytes the page already fetches, plus the page's
own collaboration protocol version, is enough; no engine change.

**The decompiler effort (mode) must be shared; the output language need not
be.** A record key names a local by the engine's symbol (`v20`), and which
symbols exist, and what they stand for, depends on the passes the mode runs.
`engine.mjs` shows it with the wasm, `survey.py` counts it with the native CLI:

```
modes: "name v20 counter" in i386_pie_nl main applies in both, but declares
  `int *counter; // stack - 0x18` in Automatic and `unsigned int *counter; // stack - 0x44`
  in Fast; list finds 141 vs 140 functions
language: 3/3 functions list the same variables and give a rename the same outcome in C and in Rust

SURVEY — 503 functions: 8 vN name a different variable in Fast, 10 exist in one
mode only; 0 variable tables differ between C and Rust
```

So a rename made by a peer on Fast can land, silently, on a different variable
for a peer on Automatic, and a Fast peer can see a function the others do not
have. The mode has to be one setting for the whole session. The output language
changed no symbol in any of the 503 functions (the engine applies the same C
directives in both), so C or Rust can stay each person's choice.

**Concurrent edits need a merge rule, and a record is too coarse a unit.**
`session.js` folds a rename and a retype of one local into one record so it can
send one pinned `type v1 unsigned long total` directive. If peers replicated
whole records, Ana renaming `v1` while Ben retypes it would lose one of the two.
Replicating each *field* as its own last-writer-wins register (name, type,
each byte, each comment) keeps both. `spike/lww.mjs` is that register map (its
core is 30 lines; the rest is the schema check of §5) and `spike/lww-test.mjs`
checks it:

```
LWW OK — 17 hostile ops rejected (@FILE, bytes @FILE, a newline, bad shapes); 2000
random rounds (2-3 peers, duplicates, any order, late-join snapshots) converge;
rename+retype merge; undo respects later writers
```

Each write carries a Lamport clock `[counter, peer]`; the larger clock wins; a
delete is a `null` write. That is commutative, idempotent and associative, so
any delivery order, duplicates, or a newcomer catching up from one peer's
snapshot all end in the same map. Concurrent writes to the *same* field (two
names for one function) pick one winner everywhere; the loser is told. A
general CRDT library (Yjs, Automerge) is not needed: there is no shared text to
merge character by character, only atomic values.

## 2. Transport: WebRTC with the handshake carried by the people

A browser can only talk to another browser directly over WebRTC, and WebRTC
needs the two sides to swap an *offer* and an *answer* (SDP: ICE credentials, a
DTLS fingerprint, candidate addresses) before any packet flows. Something has
to carry those two messages. With no server, people carry them: an invite link
one way, a reply code the other.

`spike/webrtc.mjs` does exactly that between two separate headless Chrome
processes (two profiles, standing in for two users), each on `/decompile2/`;
the script passes the codes the way a person would paste them. Five runs of the
first configuration, three of the others:

| ICE configuration | Connected | Time to open | RTT median / p95 | Invite code (whole SDP, deflate + base64url) | Only the fields a peer needs |
|---|---|---|---|---|---|
| no ICE servers, Chrome defaults (host addresses hidden behind random `.local` mDNS names) | yes, every run | 204–230 ms | 0.3–0.4 / 0.4–0.6 ms | 567–571 chars | 207–212 chars |
| no ICE servers, raw host addresses (what the committed spike checks) | yes, every run | 200–215 ms | 0.3–0.4 / 0.4–0.8 ms | 546–550 chars | 180–184 chars |
| public STUN (`stun.l.google.com`) | yes | ~2.07 s (gathering never reports complete; the spike stops at 2 s) | 0.3–0.4 / 0.5–0.7 ms | 631–639 chars | 238–239 chars |

Sending 4 MiB over the ordered channel in 16 KiB messages ran at 8–280 Mbit/s
on loopback (headless Chrome on a loaded machine; SCTP's ramp-up dominates
short transfers). Even the slowest run moves a 1 MiB program in about a second.

What this shows, and what it does not:

- **No server is needed to connect**, on one machine or one network: host
  candidates (mDNS names by default, so the invite does not even reveal a LAN
  address) connected every time in about 0.2 s.
- **An invite fits in a link.** Sent whole, the SDP compresses to ~570
  characters; sending only the ICE username/password, the fingerprint and the
  candidates (and rebuilding the SDP boilerplate on the other side) is ~210.
  Either fits in a URL fragment (`/decompile2/#join=…`, never sent to any
  server) or a QR code.
- **Across the internet, a STUN server is needed to learn one's public
  address** (the srflx candidate above). STUN is a third-party server, but it
  only answers "what is my address"; it sees no content. Gathering should be
  bounded (1–2 s) or trickled over the channel once open.
- **Not measured, and not fixable without a server:** peers behind symmetric
  NAT or strict corporate firewalls (commonly quoted at 10–20 % of pairs)
  cannot connect directly and need a TURN relay, which is a server. The design
  lets a user paste a TURN URL they trust; by default those pairs get a clear
  "could not connect directly" message.
- **Every hop is encrypted** (DTLS), end to end between the browsers.
- **In CI**, resolving Chrome's default `.local` candidates needs multicast,
  which hosted runners do not reliably provide. The committed spike therefore
  runs its pass/fail case with raw host candidates
  (`--disable-features=WebRtcHideLocalIpsWithMdns`) and reports `SKIPPED`, not
  a failure, when ICE cannot connect at all; `--all` adds the mDNS and STUN
  rows. The implementation's primary CI path is the `BroadcastChannel`
  transport (two tabs, same interface), with the real-WebRTC test on top.

**A reply that comes back late.** People carry the reply, so the inviter may
apply it minutes after the guest made it. With Chrome's defaults the answer
takes the DTLS client role (`a=setup:active`), and a reply applied more than
about 223 s after it was made did not connect. The guest's side of ICE
connects at once (the inviter answers its connectivity checks before it has
the reply), so the guest starts the DTLS handshake immediately and gives up
when its retransmissions run out. Making the guest's answer passive
(`a=setup:passive`, so the inviter starts the handshake once it applies the
reply) removes the limit. Two headless Chrome processes, raw host candidates,
the reply applied after a delay (`rtc-delay-passive.mjs`, kept out of the
tree; one run per delay):

| Delay before the inviter applies the reply | Answer active (Chrome's default) | Answer passive |
|---|---|---|
| a few seconds | open | open (5 s) |
| more than about 223 s | not open | open (7 min; 30 min: open 231 ms after the reply was applied) |

The 30-minute log, guest then inviter: `setup:a=setup:passive | 0s
ice:checking | 0s conn:connecting | 0s ice:connected | 1800s conn:connected |
1800s OPEN` and `1s ice:checking | 1801s ice:connected`. The implementation
answers passive (`sdp.js`), and `test/decompile2-collab-rtc.mjs --late 60` runs
the same case through the page: with the reply applied 60 s after it was made,
the guest had the program 520 ms later on Chrome 131 and 522 ms later on Chrome
154.

The alternatives, for the record:

| Carrier for the handshake | Server of ours? | Cost |
|---|---|---|
| People paste an invite link and a reply code (this proposal) | none | two copy-pastes per newcomer (only one per newcomer once a group exists, below) |
| Public signalling (WebTorrent trackers, Nostr relays, public MQTT; the `trystero` approach) | none of ours, but third-party servers we do not control | one-click join; the room id and SDP pass through strangers' servers; availability is theirs |
| `BroadcastChannel` | none | same browser only; useful as a test and demo transport, not for people |
| A signalling server | yes | ruled out by the brief |

## 3. More than two people

A newcomer needs one out-of-band exchange with *any* member. After that the
group can introduce them to everyone else by relaying the offer and answer over
the channels that already exist, so a full mesh forms with no further pasting.
A mesh is fine for the 2–8 people a study group is; each peer then sends its
cursor to n-1 others (about 30 small messages a second each). For a whole
class, a star through the inviting peer scales better but ends the session when
that peer leaves; the proposal starts with a mesh.

## 4. Presence and pings: anchor to content, not to pixels

Screen coordinates mean nothing across two windows: different sizes, different
views (C, assembly, side by side), different scroll positions, the Explain
panel open or not. The page already gives every thing a person can point at a
stable DOM identity, the same ids the linking code uses:

| Pointing at | Anchor |
|---|---|
| a C line / a name on it | `#c-L<n>` / `.t[data-sym]` in the open function |
| an instruction | `#a-<addr>` |
| a byte | `.hb[data-a=<addr>]` |
| a stack slot | `tr[data-slot=<offset>]` |

So a cursor travels as `{fn, view, anchor, fx, fy}` (fx/fy: the fraction across
the anchored element) and each peer draws it where *that* element is in *its*
window, or not at all when the element is not on screen; the roster then says
"Ben is in sum_to, Assembly" with a Follow button. Cursors go on an unordered,
no-retransmit channel (a late cursor is worthless), edits on an ordered,
reliable one; the spike opens both. A ping is the same anchor plus a name,
sent reliably, drawn as a pulse in the sender's colour, with a "Go there"
action when the pinged thing is not on screen.

## 5. Costs a design has to respect

- **Every remote edit re-decompiles on every peer.** Each peer runs its own
  engine; an `inspect` of a small function is ~0.2–0.5 s. Remote changes that do
  not touch the open function (or the globals) must not re-inspect, and a burst
  should be debounced.
- **Everyone must have the same bytes.** The page already hashes every file
  (SHA-256, `persist.js`); peers compare hashes on connect. A guest who has not
  opened the file can receive it from the host (the transfer above), which
  should be the host's explicit choice: some binaries are not the host's to
  share.
- **Remote input is untrusted, and two directive forms read files.** A value
  that starts with `@` is a directive file (`parse_directive_flag`), and
  `bytes ADDR @FILE` reads a file into a patch. Inside WASI only `/work` and
  `/specs` are mounted, so this is not an escape, but a peer must not be able to
  make your engine read files; `engine.mjs` shows both forms at work:

  ```
  @FILE: "@/work/sample.c" makes the engine parse that file (error: /work/sample.c:2: …);
    "bytes 0x11af @/work/sample.c" reads all 288 bytes of it into a patch
  ```

  A newline in a remote value is the same threat one step later: the page's
  `.kuna` export joins directives with newlines, so a note ending in
  `\nbytes 0x401000 @/home/you/.ssh/id_rsa` would become its own line, and the
  native CLI, which does see your files, would read it on replay. So every op
  from a peer passes a schema check before it touches the session: a known key
  shape, a value of its kind's shape (a byte is two hex digits, a name an
  identifier), no control characters, no `@` token in a raw directive, no `@`
  in a type, and size caps. `lww.mjs` has that check (`validOp`) and
  `lww-test.mjs` feeds it 17 hostile ops:

  ```
  LWW OK — 17 hostile ops rejected (@FILE, bytes @FILE, a newline, bad shapes); …
  ```

  Beyond that, remote directives still go through the page's refusal path (a
  bad one is marked refused and cannot lock anyone out), names and notes are
  already escaped when rendered, and messages need a rate limit.
- **Offline edits merge on reconnect** for free with the register map; each
  peer keeps persisting its merged session in `localStorage` as today.
- **The raw-record counter collides** across peers (table in §1); keys become
  `raw:<peer>:<n>`.
