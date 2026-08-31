/* kuna RE-friction dashboard — vanilla ES2019, no build step, no CDN.
 *
 * The server publishes one payload per cache generation and pushes {"seq":N} over SSE when
 * that generation advances; this file refetches /api/state on a bump and otherwise sits idle.
 * Elapsed/stale counters are derived here from started_at/updated_at against a server clock
 * offset, which is why a quiet pipeline produces no seq churn at all.
 */
'use strict';

var S = null;              /* last /api/state payload */
var SKEW = 0;              /* serverNow - clientNow, seconds */
var PANE = 'board';
var EV_TAB = 'backlog';
var LAST_SEQ = -1;
var FILTERED = null;       /* /api/needs result while a filter is active */

function $(id) { return document.getElementById(id); }
function esc(s) {
  return String(s === null || s === undefined ? '' : s)
    .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}
function now() { return Date.now() / 1000 + SKEW; }
function dur(sec) {
  if (sec === null || sec === undefined || isNaN(sec)) return '—';
  sec = Math.max(0, Math.floor(sec));
  if (sec < 60) return sec + 's';
  if (sec < 3600) return Math.floor(sec / 60) + 'm' + String(sec % 60).padStart(2, '0') + 's';
  return Math.floor(sec / 3600) + 'h' + String(Math.floor((sec % 3600) / 60)).padStart(2, '0') + 'm';
}
function usd(v) { return (v === null || v === undefined) ? '—' : '$' + Number(v).toFixed(2); }
function num(v) { return (v === null || v === undefined) ? '—' : Number(v).toLocaleString(); }
function gb(v) { return (v === null || v === undefined) ? '—' : Number(v).toFixed(0) + ' GB'; }
function clock(ts) {
  if (!ts) return '';
  var d = new Date(ts * 1000);
  return String(d.getHours()).padStart(2, '0') + ':' + String(d.getMinutes()).padStart(2, '0')
    + ':' + String(d.getSeconds()).padStart(2, '0');
}
function nothing(title, hint) {
  return '<div class="empty"><b>' + esc(title) + '</b>' + (hint || '') + '</div>';
}

/* ── header ───────────────────────────────────────────────────────────── */

function renderHeader() {
  var h = S.header || {}, sup = S.supervisor || {};
  $('h-round').innerHTML = h.round === null || h.round === undefined
    ? '<em>none yet</em>'
    : esc(h.round) + (h.rounds_planned ? '<em> / ' + esc(h.rounds_planned) + '</em>' : '');
  $('h-sup').textContent = sup.state || '—';
  $('h-agents').innerHTML = esc(h.agents_live || 0) + '<em> / ' + esc(h.agents_max || 0) + '</em>';
  var disk = h.disk_free_gb;
  $('h-disk').textContent = gb(disk);
  $('h-disk-w').className = 'stat' + (disk !== null && disk !== undefined
    && disk < (h.disk_red_gb || 100) ? ' bad' : '');
  $('h-spend').innerHTML = usd(h.run_usd) + '<em> / ' + usd(h.run_cap) + '</em>';
  $('n-agents').textContent = (S.agents || []).length ? '(' + S.agents.length + ')' : '';
  var t = (S.needs && S.needs.totals) || {};
  $('n-evidence').textContent = t.admitted ? '(' + t.admitted + ')' : '';
  var a = S.acceptance || {};
  $('n-acceptance').textContent = a.regressions ? '(' + a.regressions + '!)' : '';
  $('f-foot').innerHTML = 'state ' + esc(S.state_dir) + ' · needs from <code>'
    + esc((S.needs || {}).source || '—') + '</code> · gh '
    + (S.gh_ok ? 'ok' : '<b>(gh unavailable)</b>')
    + ' · modules ' + Object.keys(S.modules || {}).map(function (k) {
      return k + (S.modules[k] ? '&#10003;' : '&#183;');
    }).join(' ') + ' · read-only dashboard';
}

/* ── 1. round board ───────────────────────────────────────────────────── */

function laneHTML(name, states, current) {
  var idx = states.indexOf(current);
  var chips = states.map(function (s, i) {
    var cls = 'chip';
    if (s === current) cls += ' now';
    else if (idx >= 0 && i < idx) cls += ' done';
    return '<span class="' + cls + '">' + esc(s) + '</span>';
  }).join('');
  return '<div class="lane"><div class="lanehead"><b>' + esc(name) + '</b>'
    + '<span class="cur">' + esc(current || 'not started') + '</span></div>'
    + '<div class="chips">' + chips + '</div></div>';
}

/* One barrier per round, not one per lane: INTEGRATE runs once, after both tracks. */
function barrierHTML(done) {
  return '<div class="lane barrierlane"><div class="lanehead"><b>Barrier</b>'
    + '<span class="cur">' + (done ? 'INTEGRATE ran — acceptance suite replayed on merged main'
      : 'not reached — both tracks must land first') + '</span></div>'
    + '<div class="chips"><span class="chip barrier' + (done ? ' done' : '')
    + '">INTEGRATE</span></div></div>';
}

function renderBoard() {
  var r = S.current_round, lanes = S.lanes || {};
  $('b-round').textContent = r ? r.round : '—';
  $('b-slate').textContent = r && r.slate ? r.slate.length + ' challenges' : '—';
  $('b-needs').textContent = r ? ((r.needs_filed === null || r.needs_filed === undefined ? '—' : r.needs_filed)
    + ' / ' + (r.needs_closed === null || r.needs_closed === undefined ? '—' : r.needs_closed)) : '—';
  $('b-lanes').innerHTML = r
    ? laneHTML('TestTrack', lanes.test || [], r.test_state)
      + laneHTML('BuildTrack', lanes.build || [], r.build_state)
      + barrierHTML(r.integrated)
    : nothing('No round has started',
      'The Supervisor pins MAIN_SHA at <code>BOOT</code>, then TestTrack(1) opens at <code>T_PLAN</code>.');

  var tail = (r && r.tail) || [];
  $('b-tcount').textContent = r ? (r.transitions || 0) + ' transitions' : '';
  $('b-tlog').innerHTML = tail.length ? tail.map(function (t) {
    return '<div class="row"><span class="t">' + esc(clock(t.ts)) + '</span>'
      + '<span class="tr">' + esc(t.track || '—') + '</span>'
      + '<span class="m">' + esc(t.from || '·') + ' &rarr; <b>' + esc(t.to || '?') + '</b>'
      + (t.note ? ' — ' + esc(t.note) : '') + '</span></div>';
  }).join('') : nothing('No transitions recorded',
    'Every legal move appends one line to <code>rounds/&lt;n&gt;/transitions.jsonl</code>.');

  var c = S.corpus || {}, tot = c.totals || {};
  $('b-cov').textContent = (tot.attempted || 0) + ' / ' + (tot.total || 0) + ' challenges touched';
  $('b-strata').innerHTML = (c.strata || []).length ? c.strata.map(function (s) {
    return '<div><b>' + esc(s.stratum) + '</b><span class="v">' + s.attempted
      + '<em>&#8202;/&#8202;' + s.total + '</em></span>'
      + '<span class="s">' + s.coverage_pct + '% covered</span>'
      + '<span class="s">' + s.solved + ' solved &#183; ' + s.gave_up + ' gave up</span></div>';
  }).join('') : nothing('Dataset manifest not readable', '');

  var wt = S.worktrees || [], lz = S.leases || {};
  var lk = Object.keys(lz);
  $('b-infra').innerHTML =
    '<div style="font-size:.75rem;line-height:1.9">'
    + '<b>worktrees</b> ' + (wt.length ? wt.map(function (w) {
      return '<br><span class="dim">' + esc(w.path) + '</span> [' + esc(w.branch || '?') + ']';
    }).join('') : '<span class="dim">none</span>')
    + '<br><br><b>leases</b> ' + (lk.length ? lk.map(function (k) {
      return '<br><span class="pill ink">' + esc(k) + '</span> ' + esc(lz[k].holder);
    }).join('') : '<span class="dim">none held</span>')
    + '</div>';
}

/* ── 2. agents ────────────────────────────────────────────────────────── */

function ciPill(ci) {
  if (!ci) return '<span class="pill mute">—</span>';
  var m = { pass: 'ok', fail: 'bad', running: 'warn', none: 'mute', unknown: 'mute' };
  return '<span class="pill ' + (m[ci.state] || 'mute') + '" title="' + esc(ci.detail || '')
    + '">' + esc(ci.state) + '</span>';
}

function renderAgents() {
  var rows = S.agents || [];
  var head = '<thead><tr><th>Agent</th><th>Role</th><th>Pool</th><th>Phase</th>'
    + '<th>Elapsed</th><th>Stale</th><th class="right">Tokens in/out</th>'
    + '<th class="right">USD</th><th>Detail</th><th>Log</th></tr></thead>';
  if (!rows.length) {
    $('a-table').innerHTML = head + '<tbody><tr><td colspan="10">'
      + nothing('No agents registered',
        'Workers appear here as soon as <code>state register</code> runs; the cap is '
        + esc((S.header || {}).agents_max || 7) + ' (1 captain + testers + builders).')
      + '</td></tr></tbody>';
    return;
  }
  var t = now();
  var body = rows.map(function (a) {
    var el = t - (a.started_at || t), st = t - (a.updated_at || t);
    var detail;
    if (a.role === 'tester') {
      detail = '<span class="pill">' + esc(a.stratum || 'stratum ?') + '</span> '
        + (a.challenge ? '<span class="mono">' + esc(a.challenge) + '</span>' : '')
        + (a.outcome ? ' <span class="pill ' + (a.outcome === 'solved' ? 'ok'
          : a.outcome === 'gave_up' ? 'hot' : 'warn') + '">' + esc(a.outcome) + '</span>' : '')
        + (a.gave_up_reason ? '<span class="sub">' + esc(a.gave_up_reason) + '</span>' : '');
    } else {
      detail = (a.branch ? '<span class="mono">' + esc(a.branch) + '</span>' : '<span class="dim">—</span>')
        + (a.pr ? ' <a href="' + esc(a.pr_url || '#') + '" target="_blank" rel="noopener">#'
          + esc(a.pr) + '</a>' : '')
        + ' ' + ciPill(a.ci)
        + (a.leases && a.leases.length
          ? '<span class="sub">leases: ' + a.leases.map(esc).join(', ') + '</span>' : '')
        + (a.need ? '<span class="sub">need: ' + esc(a.need) + '</span>' : '');
    }
    return '<tr><td class="id">' + esc(a.id)
      + (a.note ? '<span class="sub">' + esc(a.note) + '</span>' : '') + '</td>'
      + '<td><span class="pill ' + (a.role === 'captain' ? 'ink' : '') + '">' + esc(a.role) + '</span></td>'
      + '<td class="dim">' + esc(a.pool || '—') + '</td>'
      + '<td>' + esc(a.phase || '—') + '<span class="sub">' + esc(a.status || '') + '</span></td>'
      + '<td class="num" data-el="' + (a.started_at || '') + '">' + esc(dur(el)) + '</td>'
      + '<td class="num stale' + (st > 120 ? ' red' : '') + '" data-st="' + (a.updated_at || '') + '">'
      + esc(dur(st)) + (st > 120 ? '!' : '') + '</td>'
      + '<td class="num">' + num(a.tokens_in) + ' / ' + num(a.tokens_out) + '</td>'
      + '<td class="num">' + usd(a.usd) + '</td>'
      + '<td>' + detail + '</td>'
      + '<td>' + (a.has_log
        ? '<button class="linkish" data-log="' + esc(a.id) + '">tail</button>'
        : '<span class="dim">—</span>') + '</td></tr>';
  }).join('');
  $('a-table').innerHTML = head + '<tbody>' + body + '</tbody>';

  var prs = S.prs;
  $('a-ghnote').textContent = prs === null ? '(gh unavailable)' : (prs.length + ' open');
  $('a-prs').innerHTML = '<thead><tr><th>PR</th><th>Title</th><th>Branch</th></tr></thead><tbody>'
    + (prs && prs.length ? prs.map(function (p) {
      return '<tr><td class="id"><a href="' + esc(p.url) + '" target="_blank" rel="noopener">#'
        + esc(p.number) + '</a></td><td>' + esc(p.title) + '</td><td class="mono">'
        + esc(p.headRefName) + '</td></tr>';
    }).join('') : '<tr><td colspan="3">' + nothing(
      prs === null ? 'gh unavailable' : 'No open pull requests',
      prs === null ? 'The open-PR fetch failed; the cached value is shown as null, exactly as <code>status.py</code> renders it.' : '')
      + '</td></tr>') + '</tbody>';
}

/* ── 3. evidence ──────────────────────────────────────────────────────── */

function hypCell(h) {
  var v = String(h || 'inconclusive').toLowerCase();
  if (v === 'overturned') return '<span class="pill hot">OVERTURNED</span>';
  if (v === 'upheld') return '<span class="pill ok">UPHELD</span>';
  return '<span class="pill mute">INCONCLUSIVE</span>';
}
function probeCell(id, status) {
  if (!id) return '<span class="dim">—</span>';
  var cls = status === 'pass' ? 'ok' : status === 'fail' ? 'bad' : 'mute';
  return '<button class="linkish" data-probe="' + esc(id) + '">' + esc(id) + '</button>'
    + ' <span class="pill ' + cls + '">' + esc(status || 'unrun') + '</span>';
}

function renderEvidence() {
  var n = S.needs || {}, tot = n.totals || {};
  $('e-denom').innerHTML =
    '<div><b>Filed</b><span>' + (tot.filed || 0) + '</span></div>'
    + '<div><b>Admitted</b><span>' + (tot.admitted || 0) + '</span></div>'
    + '<div class="hot"><b>Rejected</b><span>' + (tot.rejected || 0) + '</span></div>'
    + '<div><b>Rejected share</b><span>' + (tot.rejected_pct || 0) + '%</span></div>'
    + '<div class="hot"><b>Hypotheses overturned</b><span>' + (tot.overturned || 0) + '</span></div>';

  fillSelect($('f-status'), 'status', Object.keys(n.by_status || {}).concat(['rejected']));
  fillSelect($('f-track'), 'track', ['tooling', 'quality', 'perf', 'loader']);
  $('sub-backlog').setAttribute('aria-selected', EV_TAB === 'backlog');
  $('sub-rejected').setAttribute('aria-selected', EV_TAB === 'rejected');

  if (EV_TAB === 'rejected') return renderRejected(n);

  var rows = FILTERED ? FILTERED.needs : (n.backlog || []);
  $('f-count').textContent = rows.length + ' shown';
  if (!rows.length) {
    $('e-body').innerHTML = nothing('The backlog is empty',
      'A need is admitted only when its probe <b>PASSES</b> and its acceptance <b>FAILS</b> on a freshly built main at the pinned SHA.');
    return;
  }
  $('e-body').innerHTML = '<div class="tw"><table class="t">'
    + '<thead><tr><th>#</th><th>Need</th><th>Track</th><th>Sev</th><th>Status</th>'
    + '<th>Probe (current bad)</th><th>Acceptance (desired)</th><th>Hypothesis</th>'
    + '<th class="right">Cred</th><th class="right">Inst</th><th>Rounds</th></tr></thead><tbody>'
    + rows.map(function (d, i) {
      return '<tr><td class="num dim">' + (i + 1) + '</td>'
        + '<td class="id"><button class="linkish" data-need="' + esc(d.need_id) + '">'
        + esc(d.need_id) + '</button><span class="sub">' + esc(d.title) + '</span></td>'
        + '<td>' + esc(d.track) + '</td>'
        + '<td><span class="pill ' + (d.severity === 'blocker' ? 'bad' : '') + '">'
        + esc(d.severity) + '</span></td>'
        + '<td><span class="pill ' + (d.status === 'regressed' ? 'hot'
          : d.status === 'closed' ? 'ok' : '') + '">' + esc(d.status) + '</span></td>'
        + '<td>' + probeCell(d.probe_id, d.probe_status) + '</td>'
        + '<td>' + probeCell(d.acceptance_id, d.acceptance_status) + '</td>'
        + '<td>' + hypCell(d.hypothesis_status) + '</td>'
        + '<td class="num">' + (d.credibility === null || d.credibility === undefined
          ? '—' : Number(d.credibility).toFixed(2)) + '</td>'
        + '<td class="num">' + (d.instances || 0) + '</td>'
        + '<td class="dim">' + esc((d.rounds || []).join(',') || '—') + '</td></tr>';
    }).join('') + '</tbody></table></div>';
}

function renderRejected(n) {
  var groups = n.rejected_by_reason || {};
  var keys = Object.keys(groups);
  $('f-count').textContent = (n.rejected || []).length + ' rejected';
  if (!keys.length) {
    $('e-body').innerHTML = nothing('Nothing rejected yet',
      'This pile is the honest denominator: <code>already-supported</code> means the tester was wrong, not that kuna was bad.');
    return;
  }
  var why = {
    'already-supported': 'the acceptance probe PASSED at filing — kuna already does this',
    'not-reproducible': 'the probe FAILED at filing — the symptom did not replay',
    'unprobeable': 'real friction with no machine-checkable predicate',
    'user-error': 'the tester drove kuna wrong',
    'covered-by-option': 'an existing option already closes it — a default-flip candidate'
  };
  $('e-body').innerHTML = keys.map(function (k) {
    var rows = groups[k];
    return '<details class="rgroup" open><summary><span class="c">' + rows.length + '</span>'
      + '<span>' + esc(k) + '</span><span class="why">' + esc(why[k] || '') + '</span></summary>'
      + '<div class="tw"><table class="t"><thead><tr><th>Need</th><th>Track</th>'
      + '<th>Probe</th><th>Acceptance</th><th>Covered by option</th><th>Rounds</th></tr></thead><tbody>'
      + rows.map(function (d) {
        return '<tr><td class="id"><button class="linkish" data-need="' + esc(d.need_id) + '">'
          + esc(d.need_id) + '</button><span class="sub">' + esc(d.title) + '</span></td>'
          + '<td>' + esc(d.track) + '</td><td class="mono dim">' + esc(d.probe_id || '—') + '</td>'
          + '<td class="mono dim">' + esc(d.acceptance_id || '—') + '</td>'
          + '<td>' + esc(d.covered_by_option || '—') + '</td>'
          + '<td class="dim">' + esc((d.rounds || []).join(',') || '—') + '</td></tr>';
      }).join('') + '</tbody></table></div></details>';
  }).join('');
}

function fillSelect(sel, label, values) {
  var keep = sel.value;
  sel.innerHTML = '<option value="">' + label + ': any</option>'
    + values.filter(function (v, i, a) { return v && a.indexOf(v) === i; })
      .map(function (v) { return '<option value="' + esc(v) + '">' + esc(v) + '</option>'; }).join('');
  sel.value = keep;
}

/* ── 4. acceptance matrix ─────────────────────────────────────────────── */

function renderMatrix() {
  var a = S.acceptance || {}, rounds = a.rounds || [], probes = a.probes || [];
  $('m-denom').innerHTML =
    '<div><b>Acceptance probes</b><span>' + probes.length + '</span></div>'
    + '<div><b>Passing on main</b><span>' + (a.closed || 0) + '</span></div>'
    + '<div><b>Still failing</b><span>' + (a.outstanding || 0) + '</span></div>'
    + '<div class="hot"><b>Regressions</b><span>' + (a.regressions || 0) + '</span></div>'
    + '<div><b>Never replayed</b><span>' + (a.never_run || 0) + '</span></div>';
  if (!probes.length) {
    $('m-body').innerHTML = nothing('No acceptance probes filed yet',
      'INTEGRATE writes <code>rounds/&lt;n&gt;/acceptance.json</code> from '
      + '<code>verify --acceptance-suite --all --json</code>; each column here is one of those files.');
    return;
  }
  var cols = rounds.length ? rounds : [];
  var head = '<thead><tr><th class="rowh">Acceptance probe / need</th>'
    + cols.map(function (r) {
      var t = (a.totals || {})[String(r)] || {};
      return '<th>R' + esc(r) + '<span class="pid">' + (t.pass || 0) + 'P / '
        + (t.fail || 0) + 'F</span></th>';
    }).join('') + '<th>Latest</th></tr></thead>';
  var body = probes.map(function (p) {
    var cells = cols.map(function (r) {
      var v = p.cells[String(r)];
      var cls = v === 'pass' ? 'pass' : v === 'fail' ? 'fail'
        : v === undefined || v === null ? 'none' : 'other';
      if (v === 'fail' && p.first_pass_round !== null && p.first_pass_round !== undefined
        && r > p.first_pass_round) cls = 'regress';
      return '<td><span class="cell ' + cls + '">'
        + (v ? esc(v.toUpperCase()) : '·') + '</span></td>';
    }).join('');
    var rowcls = (p.regressed ? 'reg' : '') + (Object.keys(p.cells).length ? '' : ' unrun');
    return '<tr class="' + rowcls + '"><td class="rowh">'
      + (p.need_id
        ? '<button class="linkish" data-need="' + esc(p.need_id) + '">' + esc(p.need_id) + '</button>'
        : '<span class="dim">unlinked</span>')
      + (p.title ? ' ' + esc(p.title) : '')
      + '<span class="pid"><button class="linkish" data-probe="' + esc(p.probe_id) + '">'
      + esc(p.probe_id) + '</button>'
      + (p.track ? ' · ' + esc(p.track) : '')
      + (p.regressed ? ' · <b style="color:var(--red)">REGRESSION</b>' : '') + '</span></td>'
      + cells
      + '<td><span class="cell ' + (p.latest === 'pass' ? 'pass' : p.latest === 'fail' ? 'fail' : 'none')
      + '">' + esc((p.latest || '·').toUpperCase()) + '</span></td></tr>';
  }).join('');
  $('m-body').innerHTML = '<table class="t matrix">' + head + '<tbody>' + body + '</tbody></table>';
}

/* ── drawer ───────────────────────────────────────────────────────────── */

function openDrawer(title, path, html) {
  $('d-title').textContent = title;
  $('d-path').textContent = path || '';
  $('d-body').innerHTML = html;
  $('drawer').hidden = false;
  $('scrim').hidden = false;
}
function closeDrawer() { $('drawer').hidden = true; $('scrim').hidden = true; }

function showLog(id) {
  openDrawer('log · ' + id, '', '<pre>loading…</pre>');
  fetch('/api/agent/' + encodeURIComponent(id) + '/log?tail=400')
    .then(function (r) { return r.json(); })
    .then(function (d) {
      if (d.error) return openDrawer('log · ' + id, '', '<pre>' + esc(d.error) + '</pre>');
      openDrawer('log · ' + id, d.path, '<pre>' + esc((d.lines || []).join('\n')) + '</pre>');
    })
    .catch(function (e) { openDrawer('log · ' + id, '', '<pre>' + esc(e) + '</pre>'); });
}

function showNeed(id) {
  openDrawer('need · ' + id, '', '<pre>loading…</pre>');
  fetch('/api/need/' + encodeURIComponent(id)).then(function (r) { return r.json(); })
    .then(function (d) {
      if (d.error) return openDrawer('need · ' + id, '', '<pre>' + esc(d.error) + '</pre>');
      var meta = '<div class="sec"><h4>Record</h4><pre>'
        + esc(JSON.stringify(d.raw_front_matter || {}, null, 2)) + '</pre></div>';
      var secs = Object.keys(d.sections || {}).map(function (k) {
        return '<div class="sec"><h4>' + esc(k) + '</h4><pre>' + esc(d.sections[k]) + '</pre></div>';
      }).join('');
      openDrawer('need · ' + id, d.file, meta + secs);
    });
}

function showProbe(id) {
  openDrawer('probe · ' + id, '', '<pre>loading…</pre>');
  fetch('/api/probe/' + encodeURIComponent(id)).then(function (r) { return r.json(); })
    .then(function (d) {
      if (d.error) return openDrawer('probe · ' + id, '', '<pre>' + esc(d.error) + '</pre>');
      openDrawer('probe · ' + id, d.origin || '',
        '<div class="sec"><h4>' + (d.is_acceptance ? 'Acceptance — the DESIRED behaviour'
          : 'Probe — the CURRENT BAD behaviour') + '</h4><pre>'
        + esc(JSON.stringify(d.probe, null, 2)) + '</pre></div>'
        + '<div class="sec"><h4>Replays (' + d.replay_count + ')</h4><pre>'
        + esc((d.replays || []).map(function (r) { return JSON.stringify(r); }).join('\n'))
        + '</pre></div>');
    });
}

/* ── wiring ───────────────────────────────────────────────────────────── */

function render() {
  if (!S) return;
  renderHeader();
  if (PANE === 'board') renderBoard();
  else if (PANE === 'agents') renderAgents();
  else if (PANE === 'evidence') renderEvidence();
  else renderMatrix();
}

function tickTimers() {
  if (!S) return;
  var t = now();
  var r = S.current_round;
  $('b-timer').textContent = r && r.started_at ? dur(t - r.started_at) : '—';
  var sup = S.supervisor || {};
  $('b-runtimer').textContent = sup.started_at ? dur(t - sup.started_at) : '—';
  document.querySelectorAll('[data-el]').forEach(function (td) {
    var v = parseFloat(td.getAttribute('data-el'));
    if (v) td.textContent = dur(t - v);
  });
  document.querySelectorAll('[data-st]').forEach(function (td) {
    var v = parseFloat(td.getAttribute('data-st'));
    if (!v) return;
    var s = t - v;
    td.textContent = dur(s) + (s > 120 ? '!' : '');
    td.classList.toggle('red', s > 120);
  });
}

function pull() {
  return fetch('/api/state').then(function (r) { return r.json(); }).then(function (d) {
    S = d;
    if (d.now) SKEW = d.now - Date.now() / 1000;
    LAST_SEQ = d.seq;
    $('h-seq').textContent = 'seq ' + d.seq;
    var live = $('h-live');
    live.className = 'live beat';
    setTimeout(function () { live.className = 'live on'; }, 320);
    render();
    tickTimers();
  }).catch(function () {
    $('h-live').className = 'live off';
    $('h-seq').textContent = 'disconnected';
  });
}

function applyFilters() {
  var st = $('f-status').value, tr = $('f-track').value, q = $('f-q').value.trim();
  if (!st && !tr && !q) { FILTERED = null; return renderEvidence(); }
  var u = '/api/needs?' + [st && 'status=' + encodeURIComponent(st),
    tr && 'track=' + encodeURIComponent(tr), q && 'q=' + encodeURIComponent(q)]
      .filter(Boolean).join('&');
  fetch(u).then(function (r) { return r.json(); }).then(function (d) {
    FILTERED = d; renderEvidence();
  });
}

function selectPane(name) {
  PANE = name;
  ['board', 'agents', 'evidence', 'acceptance'].forEach(function (p) {
    $('pane-' + p).hidden = p !== name;
    $('tab-' + p).setAttribute('aria-selected', String(p === name));
  });
  render();
  tickTimers();
}

document.addEventListener('click', function (e) {
  var t = e.target.closest ? e.target.closest('[data-pane],[data-log],[data-need],[data-probe]') : null;
  if (!t) return;
  if (t.dataset.pane) return selectPane(t.dataset.pane);
  if (t.dataset.log) return showLog(t.dataset.log);
  if (t.dataset.need) return showNeed(t.dataset.need);
  if (t.dataset.probe) return showProbe(t.dataset.probe);
});
$('d-close').addEventListener('click', closeDrawer);
$('scrim').addEventListener('click', closeDrawer);
document.addEventListener('keydown', function (e) { if (e.key === 'Escape') closeDrawer(); });
$('sub-backlog').addEventListener('click', function () { EV_TAB = 'backlog'; renderEvidence(); });
$('sub-rejected').addEventListener('click', function () { EV_TAB = 'rejected'; renderEvidence(); });
$('f-status').addEventListener('change', applyFilters);
$('f-track').addEventListener('change', applyFilters);
var qt = null;
$('f-q').addEventListener('input', function () {
  clearTimeout(qt); qt = setTimeout(applyFilters, 220);
});
$('h-stop').addEventListener('click', function () {
  var sup = (S && S.supervisor) || {};
  openDrawer('stop the run', '', '<div class="sec"><h4>This dashboard is read-only</h4>'
    + '<p style="font-size:.8125rem;line-height:1.8">It never posts, and it cannot stop a run. '
    + 'Graceful drain — in-flight agents finish and INTEGRATE still runs:</p>'
    + '<pre>touch ' + esc(sup.stop_path || '.kuna-repipe/STOP') + '</pre>'
    + '<p style="font-size:.8125rem;line-height:1.8">Hard stop — SIGTERM to the recorded pids, '
    + 'every worktree and arena kept intact for forensics:</p>'
    + '<pre>touch ' + esc(sup.abort_path || '.kuna-repipe/ABORT') + '</pre></div>');
});

/* SSE is the only push channel; the poll is the fallback when it will not open. */
function listen() {
  var poll = setInterval(pull, 5000);
  if (!window.EventSource) return;
  try {
    var es = new EventSource('/api/events');
    es.onmessage = function (m) {
      var d = {};
      try { d = JSON.parse(m.data); } catch (err) { return; }
      if (d.seq !== LAST_SEQ) pull();
    };
    es.onopen = function () { clearInterval(poll); poll = setInterval(pull, 30000); };
    es.onerror = function () { $('h-live').className = 'live off'; };
  } catch (err) { /* keep the poll */ }
}

pull().then(listen);
setInterval(tickTimers, 1000);
