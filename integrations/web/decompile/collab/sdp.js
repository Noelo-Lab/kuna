// sdp.js — a WebRTC data-channel description cut down to what the other side
// needs (the ICE username and password, the DTLS fingerprint, the candidate
// addresses) and rebuilt on arrival, so an invite fits in a link. The rebuilt
// description comes only from fields that passed `validSdp`, so nothing a peer
// sends can add a line to it. An answer is rebuilt as `a=setup:passive`: the
// inviter starts the DTLS handshake, which keeps a reply that is applied
// minutes later working. DOM-free.

const ICE = /^[A-Za-z0-9+/]{4,256}$/;
const FP = /^[0-9A-F]{64}$/;
const MID = /^[A-Za-z0-9_-]{1,32}$/;
const CAND = /^([0-9A-Fa-f.:]{2,45}|[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.local) (\d{1,5}) ([hspr])([ut]) (\d{1,10})(?: (active|passive|so))?$/;
const TYPES = { h: 'host', s: 'srflx', p: 'prflx', r: 'relay' };
const PROTOS = { u: 'udp', t: 'tcp' };
const MAX_CANDIDATES = 16;

/** The fields of `sdp` a peer needs, `{u, p, f, c[, m]}`, or null. */
export function compactSdp(sdp) {
  const lines = String(sdp || '').split(/\r?\n/);
  const field = (prefix) => {
    const line = lines.find((l) => l.startsWith(prefix));
    return line === undefined ? null : line.slice(prefix.length).trim();
  };
  const fp = /^sha-256 ((?:[0-9A-Fa-f]{2}:){31}[0-9A-Fa-f]{2})$/i.exec(field('a=fingerprint:') || '');
  const out = { u: field('a=ice-ufrag:'), p: field('a=ice-pwd:'), f: fp ? fp[1].replace(/:/g, '').toUpperCase() : null, c: [] };
  for (const line of lines) {
    const m = /^a=candidate:\S+ 1 (udp|tcp) (\d+) (\S+) (\d+) typ (host|srflx|prflx|relay)(.*)$/i.exec(line);
    if (!m || out.c.length >= MAX_CANDIDATES) continue;
    const tcp = /\btcptype (active|passive|so)\b/.exec(m[6])?.[1];
    if (tcp === 'active') continue;
    const text = `${m[3]} ${m[4]} ${m[5][0].toLowerCase()}${m[1][0].toLowerCase()} ${m[2]}${tcp ? ` ${tcp}` : ''}`;
    if (CAND.test(text)) out.c.push(text);
  }
  const mid = field('a=mid:');
  if (mid && mid !== '0') out.m = mid;
  return validSdp(out) ? out : null;
}

/** Whether `d` is a compact description with every field in its expected shape. */
export function validSdp(d) {
  if (!d || typeof d !== 'object' || Array.isArray(d)) return false;
  const keys = Object.keys(d);
  if (keys.some((k) => !['u', 'p', 'f', 'c', 'm'].includes(k))) return false;
  return typeof d.u === 'string' && ICE.test(d.u) && typeof d.p === 'string' && ICE.test(d.p) && d.p.length >= 22 &&
    typeof d.f === 'string' && FP.test(d.f) && Array.isArray(d.c) && d.c.length <= MAX_CANDIDATES &&
    d.c.every((c) => typeof c === 'string' && CAND.test(c)) && (d.m === undefined || (typeof d.m === 'string' && MID.test(d.m)));
}

/** Whether a compact description reaches beyond its network: it carries a STUN-learned (`srflx`) or relay address. */
export function crossesNetworks(d) {
  return validSdp(d) && d.c.some((c) => /^\S+ \d+ [sr]/.test(c));
}

/** A full description from a compact one: `type` 'offer' (actpass) or 'answer' (passive). */
export function expandSdp(d, type) {
  if (!validSdp(d)) throw new Error('not a description');
  const mid = d.m || '0';
  const candidates = d.c.map((c, i) => {
    const [, addr, port, typ, proto, priority, tcp] = CAND.exec(c);
    const related = typ === 'h' ? '' : ' raddr 0.0.0.0 rport 0';
    return `a=candidate:${i + 1} 1 ${PROTOS[proto]} ${priority} ${addr} ${port} typ ${TYPES[typ]}${related}${tcp ? ` tcptype ${tcp}` : ''}`;
  });
  return [
    'v=0', 'o=- 0 2 IN IP4 127.0.0.1', 's=-', 't=0 0', `a=group:BUNDLE ${mid}`, 'a=msid-semantic: WMS',
    'm=application 9 UDP/DTLS/SCTP webrtc-datachannel', 'c=IN IP4 0.0.0.0', ...candidates,
    `a=ice-ufrag:${d.u}`, `a=ice-pwd:${d.p}`, 'a=ice-options:trickle',
    `a=fingerprint:sha-256 ${d.f.match(/../g).join(':')}`, `a=setup:${type === 'offer' ? 'actpass' : 'passive'}`,
    `a=mid:${mid}`, 'a=sctp-port:5000', 'a=max-message-size:262144', '',
  ].join('\r\n');
}

/** An answer the page is about to use as its own, turned passive (see the file header). */
export function passiveAnswer(sdp) {
  return String(sdp).replace(/a=setup:active(?=\r?\n|$)/g, 'a=setup:passive');
}
