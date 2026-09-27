# decompile2-collab — proposal

Live multi-user sessions in `/decompile2/`: several people open one program,
rename, retype, comment and patch it together, ping a line to draw the others'
attention, and see each other's pointers as faint arrows, with no server of
ours. Evidence for every claim below is in `analysis.md` and `spike/`.

## What a user sees

1. Ana opens a program, then ⋯ → **Work together…**. She types her name (a
   colour is picked for her) and gets an **invite link**
   (`…/decompile2/#join=<code>`, ~210 characters, also shown as a QR code).
2. Ben opens the link. The page shows "Join Ana's session on `crackme.elf`"
   and, if he has not opened that file, offers to receive it from Ana (Ana is
   asked first) or lets him open his own copy (checked by SHA-256). He gets a
   **reply code** to send back.
3. Ana pastes the reply code. Both see each other in the top bar: initials in
   their colour, a tooltip with where each one is ("Ben: sum_to, Assembly"),
   and a click to follow.
4. From then on:
   - **Edits**: a rename, retype, signature, note or byte patch by anyone
     appears for everyone, re-decompiled on each machine. *Your changes*
     becomes *Changes*, each line with its author's colour. If two people
     change the same field at once, one wins everywhere and the other gets a
     toast ("Ben renamed v1 to count after you").
   - **Pointers**: each other person's mouse is a translucent arrow (about 45 %
     opacity) with a name tag in their colour, drawn over the same line,
     instruction, byte or stack slot on your screen, whatever your window size
     or view. When they are somewhere you are not, the arrow hides and the
     roster says where they are.
   - **Pings**: Alt+click (or `p`) on a line, instruction, byte or slot sends a
     ping: a pulsing ring in the sender's colour on that thing for everyone,
     and a toast "Ana pinged line 6 of main" with **Go there** for anyone
     looking elsewhere.
   - **Undo** undoes *your* last change, unless someone has changed that field
     since (then it says so instead of overwriting them).
   - **Decompiler effort** is the session's, not each person's: a guest takes
     the inviter's, and changing it (⋯ → Decompiler effort, which then says
     "changes it for everyone") re-decompiles everyone. **Show code as** (C or
     Rust) stays each person's own.
5. A third person joins with a new invite from anyone already in the session;
   the group introduces them to everyone else by itself. Leaving closes that
   person's connections; the rest carry on. A dropped connection that comes
   back merges what each side did meanwhile.

## Design

**Shared state: field registers.** A new DOM-free `collab/replica.js` holds the
session as last-writer-wins registers (the spike's `lww.mjs`, grown up): one
per field a student can change (`var:<fn>:<sym>:name`, `var:<fn>:<sym>:type`,
`fn:<addr>`, `proto:<addr>`, `data:<addr>`, `typedef:<tag>`,
`comment:<fn>:<addr>`, `byte:<addr>`, `raw:<peer>:<n>`), plus one for the
decompiler effort (`setting:mode`), each write stamped with a Lamport clock
`[counter, peer id]`. The mode is a register because the engine's symbols depend
on it: the same `v20` is a different variable in Fast than in Automatic
(analysis §1), so peers on different modes would rename different things. A
session starts with the inviter's mode; a change re-runs `list` and the open
function on every peer, and records the new mode rejects are marked refused
for everyone alike, exactly as when one person switches modes today. The
output language is left per person: it changed no symbol in 503 functions.

`Session` stays the page's model: after every local edit `app.js` diffs the
session's records and bytes against the previous state and broadcasts the
changed registers; a remote op patches
`session.records`/`session.bytes` directly and takes a *remote* path through
`applyEdit` (re-inspect only when the open function or a global changed,
debounced 300 ms, no undo entry, no snapshot restore on failure). Directives
are sent in one canonical order (by kind, then key) whether or not a session is
shared.

**Transport: `collab/link.js`.** One `RTCPeerConnection` per pair of peers, two
data channels: `edits` (ordered, reliable) for register ops, snapshots, pings,
roster and the file transfer; `cursor` (unordered, `maxRetransmits: 0`) for
pointer positions, throttled to 30 Hz. The handshake:

- The invite carries only the ICE username/password, the DTLS fingerprint and
  the candidates, deflated and base64url'd in the URL fragment (never sent to
  any server); the reply code likewise. Both are single use.
- ICE: host candidates always (mDNS names, as Chrome does by default); the
  public STUN server only if the user leaves "Connect across the internet" on
  (default on, with one line explaining that it reveals your public address to
  the people you invite); an optional TURN URL field for networks that need a
  relay. Gathering is capped at 1.5 s.
- Joining a group: the newcomer's first link is to whoever invited them; that
  peer then relays offers and answers between the newcomer and each other
  member over existing `edits` channels, forming a full mesh.
- A `BroadcastChannel` transport with the same interface lets two tabs of one
  browser share a session: the test transport, and a zero-setup demo.

**Hello.** On connect, peers exchange `{protocol version, build id, file
SHA-256, name, colour}` and refuse politely on any mismatch ("Ben's page is a
different version of Kuna; reload both"). The build id is the SHA-256 of the
`kuna_wasm.wasm` bytes the loader already fetches (hashed once, from a clone of
that response) plus the page's collaboration protocol version, so no engine
change is needed. Then a register snapshot each way (which carries the mode),
merged; then live ops.

**Presence: `collab/presence.js`.** A cursor message is `{fn, view, anchor, fx,
fy}` where `anchor` is the element id the page already uses (`c-L5`,
`a-0x11b5`, `.hb[data-a=…]`, `tr[data-slot=…]`) and fx/fy the fraction across
it. The receiver finds that element in its own DOM and draws a
`pointer-events: none` SVG arrow, positioned with `transform` in one overlay
per pane (no layout per message). Pings use the same anchor and the existing
flash animation, in the sender's colour.

**Guards.** Every op from a peer passes the schema check before it touches
the session (`validOp` in the spike, tested against 17 hostile ops): a known key
shape; a value of its kind's shape (a byte is two hex digits, a name is an
identifier, a mode is one of the four); no control characters, so nothing can
split into a second directive in an exported `.kuna` file; and never a form that
makes the engine read a file, i.e. no `@` token in a raw directive (which covers
both `@FILE` and `bytes ADDR @FILE`) and no `@` in a type or signature.
Messages are size-capped (64 KiB, except the file, which is chunked and capped
at the size the page already opens) and rate limited (edits 20/s, pings 1/s,
cursors 30/s per peer). Remote directives still go through the page's refusal
path, and names are escaped as today. Any peer can edit; there are no roles in
the first version.

## Milestones (each one a PR with its own tests)

| | Delivers | Tests |
|---|---|---|
| C1 | `link.js` + manual invite/reply UI + `BroadcastChannel` transport, connection status | CI's main path is two tabs over `BroadcastChannel`; on top, two Chrome processes connect over real WebRTC through harness-carried codes, launched with raw host candidates (runners lack the multicast `.local` names need) and reporting SKIPPED, not failing, when ICE cannot connect; bad or reused codes are refused |
| C2 | `replica.js` behind `Session`, the shared mode register, canonical directive order, hello with file-hash and build-id checks, snapshot on join, file transfer on the host's say-so, merge after reconnect, the schema check | the spike's convergence and hostile-op tests grown to the full key set; a rename on page A shows on page B; concurrent rename + retype keeps both; switching the mode on A re-decompiles B; an offline edit merges |
| C3 | roster, follow, content-anchored translucent cursors | when A points at line 6, A's arrow sits on line 6 in B's window at 1440 px and at 1024 px, in C and in side by side; it hides when B opens another function |
| C4 | pings | an Alt+click on A pulses the same line on B; **Go there** opens it |
| C5 | three or more peers: relayed introductions, mesh, leaving and rejoining | three pages, the third joining through the second |
| C6 | limits, errors, docs (`docs/web-integration.md` §4.2), help text | oversize and malformed messages dropped; the page stays usable |

The collaboration code stays in `integrations/web/decompile2/collab/` and is
loaded only when a session starts or a `#join=` link is opened; nothing changes
for a solo user (the build keeps a solo page's directive order; see the
departures below).

## Decisions taken (2026-09-27), and what was built

The user answered the five questions below; the feature was built to those
answers in PR #742.

1. **Handshake: approved.** The user asked whether a guest could just paste
   the link and be in. Not with no server at all: the guest's reply has to
   reach the inviter, and only a relay (a public third-party service, or one of
   ours) could carry it on its own. Built instead: the guest opens the invite
   link and gets a *reply link*; the inviter clicks it, which opens a tab of the
   same page in their own browser that hands the reply to the waiting tab over
   `BroadcastChannel` and says "Connected — you can close this tab". A paste
   box in the invite stays as the fallback (another browser, a phone). Between
   tabs of one browser the invite alone is enough: the guest's page finds the
   inviter's tab over `BroadcastChannel` and no reply is needed.
2. **STUN: built, off.** STUN is free (Google's public server, no account);
   only a TURN relay costs money. By default there are no ICE servers (same
   machine or same network). One setting turns them on, with no switch in the
   page: `localStorage` key `kuna.d2.collab`, `{"stun": true}` for Google's
   server (or a `stun:` URL), `{"turn": {"urls", "username", "credential"}}` for
   a relay (`docs/web-integration.md` §4.2).
3. **The program travels automatically.** The inviter's page sends it on join
   (64 KiB chunks, paced by `bufferedAmount`, SHA-256 checked, at most 64 MiB)
   and the guest's page opens it with no prompt; the invite dialog says that
   the people who join receive a copy. A guest who has the same program open
   (same SHA-256) is not sent it again.
4. **Up to 8 people**, a full mesh with relayed introductions; the ninth is
   told "This session is full (8 people)".
5. **No roles.** Everyone edits.

Where the build departs from the design above, and why:

- **No QR code.** The invite is a link of about 300 characters; a QR encoder
  was not in the approved build list.
- **The file hash is not in the invite.** The link carries the inviter's name,
  the program's name and size and the connection details; the SHA-256 comes in
  the welcome over the encrypted channel, which keeps the link short.
- **Pointers carry a character column** (`col`) for code rows as well as the
  fraction across the anchor, so an arrow lands on the same name in windows of
  any width; bytes and stack slots use the fraction.
- **The build id is the SHA-256 of the bytes the Worker compiles**, computed
  by the Worker from a clone of the response it compiles (a client that does
  not ask for it, like `/decompile/`, pays nothing), and again after each
  restart; a page whose engine changes during a session leaves it. (The first
  build hashed its own fetch of `kuna_wasm.wasm`; a review showed two pages on
  either side of a deploy could then pass the check.)
- **Directive order: as made when alone, by birth when shared.** The design's
  "one canonical order (by kind, then key)" broke replay wherever order
  matters (a type used by a later type, the later of two prototypes, a rename
  chain), so a page on its own keeps the order edits were made in, and a
  shared session orders each kind by each register's birth clock (the
  smallest clock any page wrote it with, merged as a grow-only minimum), which
  every page agrees on. A solo page is unchanged.
- **A guest's own stored changes are kept apart, not merged,** when the
  program arrives with the join: the session is saved under its own key
  (`kuna.d2.shared.<hash>`), the session dialog offers to save the guest's
  changes as a `.kuna` file, and leaving brings them back (a toast offers the
  session's changes instead). A guest with nothing stored keeps the session as
  its own. A guest who already has the program open when joining brings its
  changes into the session (the session's value wins where both changed a
  field).
- **In a session, a change is never rolled back by a failed or cancelled
  request.** The others already have it; the page says so and Undo takes it
  back. The design's "no snapshot restore" held for remote changes only.
- **SHA-256 has a plain-JavaScript fallback**, since a page served over plain
  HTTP on a local network has no `crypto.subtle`.
- **One PR with milestone commits**, rather than one PR per milestone.

## Open questions for review (the user's decisions)

1. **Handshake.** Is copy-paste of an invite link and a reply code acceptable,
   or should an opt-in "easy join" use public third-party signalling (trackers
   or Nostr relays, as `trystero` does) for one-click joins?
   *Recommendation:* copy-paste only in version one. It is the only option with
   no server anywhere, and one exchange per newcomer is enough once a group
   exists. Revisit "easy join" as an opt-in if people find it too fiddly.
2. **STUN.** Is using Google's public STUN server by default acceptable?
   Without it, sessions only work on one network.
   *Recommendation:* on by default, with one line saying it tells the people you
   invite your public address, and a switch to turn it off. STUN sees no
   content.
3. **Sharing the program.** May a guest receive the binary from the host
   (convenient; the host is asked each time), or must every guest open their
   own copy?
   *Recommendation:* allow both. The host is asked each time, and a guest can
   always open their own copy instead (checked by SHA-256).
4. **Group size.** A mesh for 2–8 people, or plan for a whole class (a star or
   a tree, and who owns the session)?
   *Recommendation:* a full mesh capped at 8 in version one. A class-sized star
   is a later milestone, if it is wanted.
5. **Permissions.** Everyone edits in version one; is a read-only "watch"
   role wanted?
   *Recommendation:* everyone edits. A per-guest "watch only" switch is cheap
   to add later: the other peers drop that guest's edit ops.
