const REPO = 'https://github.com/Noelo-Lab/kuna';
const number = new Intl.NumberFormat('en-US');
const shortDate = new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric', timeZone: 'UTC' });
const longDate = new Intl.DateTimeFormat('en-US', { month: 'long', day: 'numeric', year: 'numeric', timeZone: 'UTC' });

const $ = (id) => document.getElementById(id);
const esc = (value) => String(value).replace(/[&<>"']/g, (c) => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const title = (slug) => slug.replace(/[-_]/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
const sourceKey = (source) => source.toLowerCase() === 'ghidra-upstream' ? 'ghidra' : source.toLowerCase();
const sourceLabel = (source) => ({'ghidra':'Ghidra','angr':'angr','ida':'IDA','binja':'Binary Ninja','kuna':'Kuna','oxidizer':'Oxidizer'}[sourceKey(source)] || title(source));
const kilo = (lines) => lines < 1000 ? String(lines) : `${(lines / 1000).toFixed(lines < 10000 ? 1 : 0)}k`;
const iso = (date) => new Date(`${date}T00:00:00Z`);
const repoLink = (path, line = '') => `${REPO}/blob/main/${path}${line ? `#L${line}` : ''}`;

function daysBetween(start, end) {
  const out = [];
  for (let cursor = iso(start); cursor <= iso(end); cursor = new Date(cursor.getTime() + 86400000)) {
    out.push(cursor.toISOString().slice(0, 10));
  }
  return out;
}

function monday(date) {
  const d = iso(date);
  const delta = (d.getUTCDay() + 6) % 7;
  d.setUTCDate(d.getUTCDate() - delta);
  return d.toISOString().slice(0, 10);
}

function summarize(data) {
  const commits = data.commits.length;
  const agent = data.commits.filter((c) => c.agent).length;
  const upstream = data.baselines.upstream;
  const stages = data.baselines.stages;
  $('stat-commits').textContent = number.format(commits);
  $('stat-agent').textContent = `${Math.round(agent / commits * 100)}%`;
  $('stat-options').textContent = number.format(data.options.length);
  $('stat-tests').textContent = number.format(upstream[1] + stages[1]);
  $('snapshot').innerHTML = `Repository snapshot <a href="${REPO}/commit/${data.meta.sha}">${data.meta.sha.slice(0, 8)}</a> · ${longDate.format(iso(data.meta.end))} UTC`;
  $('data-note').innerHTML = `Generated from Kuna commit <a href="${REPO}/commit/${data.meta.sha}">${data.meta.sha.slice(0, 8)}</a>. Commit attribution counts explicit <code>[AUTOMATED]</code> markers or agent co-author trailers; source churn is added plus removed Rust/C/C++ lines in non-merge commits.`;
  $('decbench-options').textContent = number.format(data.options.filter((o) => o.decbench).length);
  $('novel-cases').textContent = data.novelPool.cases == null ? '—' : number.format(data.novelPool.cases);
  $('novel-features').textContent = number.format(data.records.filter((r) => r.novel).length);
  $('oracle-count').textContent = number.format(upstream[0]);
  $('oracle-total').textContent = number.format(upstream[1]);
  $('stage-count').textContent = number.format(stages[0]);
  $('stage-total').textContent = number.format(stages[1]);
  $('triage-count').textContent = number.format(data.triage.length);
  const statuses = Object.entries(data.triage.reduce((a, r) => ((a[r.status] ||= []).push(r), a), {}))
    .map(([status, rows]) => `${rows.length} ${status.replaceAll('-', ' ')}`);
  $('triage-summary').textContent = statuses.join(' · ') + '.';
}

function renderMilestones(data) {
  $('milestones').innerHTML = data.milestones.map((m) => `<li>
    <time datetime="${m.date}">${shortDate.format(iso(m.date))}</time>
    <h3><a href="${esc(m.url || `${REPO}/commit/${m.commit}`)}">${esc(m.title)}</a></h3>
    <p>${esc(m.detail)}</p>
  </li>`).join('');
}

function dailyRows(data) {
  const grouped = new Map();
  for (const commit of data.commits) {
    if (!grouped.has(commit.date)) grouped.set(commit.date, {date:commit.date, commits:0, agent:0, added:0, removed:0});
    const row = grouped.get(commit.date);
    row.commits++;
    row.agent += commit.agent ? 1 : 0;
    row.added += commit.added;
    row.removed += commit.removed;
  }
  return [...grouped.values()].sort((a, b) => a.date.localeCompare(b.date)).map((row, i, rows) => ({
    ...row,
    skipped: i ? Math.round((iso(row.date) - iso(rows[i - 1].date)) / 86400000) - 1 : 0,
  }));
}

function renderPace(data) {
  const canvas = $('pace-chart');
  const shell = $('pace-scroll');
  const tip = $('pace-tip');
  const rows = dailyRows(data);
  const cssWidth = Math.max(820, shell.clientWidth);
  const cssHeight = 340;
  const ratio = Math.min(window.devicePixelRatio || 1, 2);
  canvas.style.width = `${cssWidth}px`;
  canvas.style.height = `${cssHeight}px`;
  canvas.width = cssWidth * ratio;
  canvas.height = cssHeight * ratio;
  const ctx = canvas.getContext('2d');
  ctx.scale(ratio, ratio);
  const pad = {left:44,right:48,top:30,bottom:62};
  const width = cssWidth - pad.left - pad.right;
  const height = cssHeight - pad.top - pad.bottom;
  const step = width / rows.length;
  const maxCommit = Math.max(...rows.map((r) => r.commits));
  const maxChurn = Math.max(...rows.map((r) => r.added + r.removed));
  const portDays = rows.map((r, i) => r.date >= '2026-06-10' && r.date <= data.meta.portEnd ? i : -1).filter((i) => i >= 0);
  ctx.font = '10px "Roboto Mono", monospace';
  if (portDays.length) {
    ctx.fillStyle = '#F7F4F3';
    ctx.fillRect(pad.left + portDays[0] * step, pad.top, portDays.length * step, height);
    ctx.fillStyle = '#6E6663';
    ctx.fillText('RUST PORT', pad.left + portDays[0] * step + 6, pad.top + 13);
  }
  ctx.strokeStyle = '#E4DDDB'; ctx.lineWidth = 1;
  for (let i = 0; i <= 4; i++) {
    const y = pad.top + height * i / 4;
    ctx.beginPath(); ctx.moveTo(pad.left, y + .5); ctx.lineTo(cssWidth - pad.right, y + .5); ctx.stroke();
    ctx.fillStyle = '#6E6663';
    ctx.fillText(String(Math.round(maxCommit * (4 - i) / 4)), 8, y + 4);
  }
  rows.forEach((row, i) => {
    const barHeight = row.commits / maxCommit * height;
    ctx.fillStyle = '#141110';
    ctx.fillRect(pad.left + i * step + Math.max(1, step * .12), pad.top + height - barHeight, Math.max(1, step * .72), barHeight);
  });
  ctx.beginPath();
  rows.forEach((row, i) => {
    const churn = row.added + row.removed;
    const scaled = maxChurn ? Math.log1p(churn) / Math.log1p(maxChurn) : 0;
    const x = pad.left + (i + .5) * step;
    const y = pad.top + height - scaled * height;
    i ? ctx.lineTo(x, y) : ctx.moveTo(x, y);
  });
  ctx.strokeStyle = '#B80D1E'; ctx.lineWidth = 2; ctx.stroke();
  const tickStride = Math.max(1, Math.ceil(72 / step));
  const ticks = rows.filter((_, i) => i % tickStride === 0 && i < rows.length - tickStride || i === rows.length - 1);
  ctx.fillStyle = '#6E6663'; ctx.font = '10px "Roboto Mono", monospace';
  for (const row of ticks) {
    const i = rows.indexOf(row), x = pad.left + (i + .5) * step;
    ctx.fillText(shortDate.format(iso(row.date)), Math.min(x, cssWidth - 80), cssHeight - 32);
  }
  const breaks = rows.filter((row) => row.skipped);
  ctx.fillStyle = '#B80D1E';
  for (const row of breaks) {
    const x = pad.left + rows.indexOf(row) * step;
    ctx.fillText('//', x - 6, pad.top + height + 13);
  }
  ctx.fillStyle = '#6E6663';
  ctx.fillText('Active days (UTC) · // date jump', pad.left, cssHeight - 9);
  $('pace-breaks').textContent = breaks.length ? 'Date jumps: ' + breaks.map((row) => {
    const previous = rows[rows.indexOf(row) - 1];
    return `${shortDate.format(iso(previous.date))} → ${shortDate.format(iso(row.date))} (${row.skipped} ${row.skipped === 1 ? 'day' : 'days'} without commits)`;
  }).join(' · ') + '.' : 'No days without commits in this snapshot.';
  const peak = rows.reduce((a, b) => b.commits > a.commits ? b : a);
  const summary = `${number.format(data.commits.length)} non-merge commits across ${rows.length} active UTC days. Peak: ${peak.commits} commits on ${longDate.format(iso(peak.date))}. Lines changed includes both additions and removals.`;
  $('pace-summary').textContent = summary;
  let selected = rows.length - 1;

  function inspect(index) {
    selected = index;
    const row = rows[index];
    tip.hidden = false;
    const description = `${longDate.format(iso(row.date))} UTC: ${row.commits} commits; ${row.agent} marked as agent-assisted; ${number.format(row.added + row.removed)} source lines changed.${row.skipped ? ` ${row.skipped} days without commits omitted before this date.` : ''}`;
    tip.innerHTML = `<b>${longDate.format(iso(row.date))} UTC</b><br>${row.commits} commits · ${row.agent} agent-assisted<br><em>${number.format(row.added + row.removed)} source lines changed</em>${row.skipped ? `<br>${row.skipped} days without commits omitted` : ''}`;
    const left = Math.max(8, Math.min(cssWidth - 205, pad.left + index * step - shell.scrollLeft));
    tip.style.left = `${left + shell.scrollLeft}px`; tip.style.top = '38px';
    $('pace-summary').textContent = description;
  }
  canvas.onpointermove = (event) => inspect(Math.max(0, Math.min(rows.length - 1, Math.floor((event.clientX - canvas.getBoundingClientRect().left - pad.left) / step))));
  const dismiss = () => { tip.hidden = true; $('pace-summary').textContent = summary; };
  canvas.onpointerleave = dismiss;
  canvas.onblur = dismiss;
  canvas.onfocus = () => inspect(selected);
  canvas.onkeydown = (event) => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const next = event.key === 'Home' ? 0 : event.key === 'End' ? rows.length - 1 : selected + (event.key === 'ArrowLeft' ? -1 : 1);
    inspect(Math.max(0, Math.min(rows.length - 1, next)));
    shell.scrollLeft = Math.max(0, pad.left + (selected + .5) * step - shell.clientWidth / 2);
  };
}

function weeklyPhase(data) {
  const weeks = [...new Set(daysBetween(monday(data.meta.start), data.meta.end).map(monday))];
  const values = new Map(data.phases.flatMap((p) => weeks.map((w) => [`${p.id}:${w}`, {churn:0, commits:new Set()}])));
  for (const commit of data.commits) {
    const week = monday(commit.date);
    for (const [phase, stats] of Object.entries(commit.phases)) {
      const row = values.get(`${phase}:${week}`);
      if (!row) continue;
      row.churn += stats.added + stats.removed;
      row.commits.add(commit.sha);
    }
  }
  return {weeks, values};
}

function renderHeatmap(data) {
  const root = $('phase-heatmap');
  const detail = $('phase-detail');
  const {weeks, values} = weeklyPhase(data);
  root.style.gridTemplateColumns = `124px repeat(${weeks.length}, minmax(27px, 1fr))`;
  root.innerHTML = '<span></span>' + weeks.map((w, i) => `<span class="heat-week">${i % 2 === 0 ? shortDate.format(iso(w)) : ''}</span>`).join('');
  const max = Math.max(...[...values.values()].map((v) => v.churn));
  let selected = null;
  function show(phase, week, value, button) {
    selected?.classList.remove('selected'); button.classList.add('selected'); selected = button;
    const today = data.architecture.phases[phase.id];
    detail.innerHTML = `<span class="phase-id">${phase.id}</span><div><h3>${esc(phase.name)}</h3><p>${esc(phase.description)} · week of ${longDate.format(iso(week))}</p></div><p class="phase-metric"><strong>${number.format(value.churn)}</strong> source lines changed in ${value.commits.size} commit${value.commits.size === 1 ? '' : 's'} · ${phase.options} current options · ${kilo(today.loc)} lines today</p>`;
  }
  for (const phase of data.phases) {
    root.insertAdjacentHTML('beforeend', `<span class="heat-label"><b>${phase.id}</b><span>${esc(phase.name)}</span></span>`);
    for (const week of weeks) {
      const value = values.get(`${phase.id}:${week}`);
      const intensity = value.churn && max ? .1 + .9 * Math.log1p(value.churn) / Math.log1p(max) : 0;
      const button = document.createElement('button');
      button.className = 'heat-cell'; button.type = 'button';
      button.style.background = value.churn ? `rgba(184,13,30,${intensity.toFixed(3)})` : '#F8F4F3';
      button.title = `${phase.id} · week of ${week}: ${number.format(value.churn)} source lines changed in ${value.commits.size} commits`;
      button.setAttribute('aria-label', button.title);
      button.addEventListener('click', () => show(phase, week, value, button));
      root.append(button);
    }
  }
  const busiest = [...values.entries()].reduce((a, b) => b[1].churn > a[1].churn ? b : a);
  const [phaseId, week] = busiest[0].split(':');
  const phase = data.phases.find((p) => p.id === phaseId);
  const index = 1 + weeks.length + data.phases.indexOf(phase) * (weeks.length + 1) + weeks.indexOf(week) + 1;
  show(phase, week, busiest[1], root.children[index]);
}

function renderArchitecture(data) {
  const arch = data.architecture;
  const passes = arch.schedule.passes.reduce((a, p) => ((a[p.phase] = (a[p.phase] || 0) + 1), a), {});
  const rows = [
    ...data.phases.map((p) => ({...arch.phases[p.id], id: p.id, name: p.name, options: p.options, passes: passes[p.id] || 0})),
    ...arch.shared.map((f) => ({...f, id: '', name: f.folder === 'substrate' ? 'Shared IR' : 'Framework', shared: true})),
  ];
  const max = Math.max(...rows.map((r) => r.loc));
  const module = (m) => `<li><a href="${repoLink(`decompiler/crates/kuna-decomp/src/${m.folder}/${m.file}`)}">${esc(m.file)}</a> <span>${number.format(m.loc)}</span>${m.kuna ? ' <em>Kuna</em>' : ''}</li>`;
  $('arch-table').innerHTML = `<div class="arch-head" aria-hidden="true"><span>phase</span><span>lines of Rust</span><span>in kuna_*</span><span>options</span><span>passes</span></div>` + rows.map((r) => {
    const kunaShare = r.kunaLoc != null ? Math.round(r.kunaLoc / r.loc * 100) : null;
    const facts = r.shared ? '<span>—</span><span>—</span><span>—</span>'
      : `<span>${kunaShare}%</span><span>${r.options}</span><span>${r.passes}</span>`;
    const body = r.shared ? `<p>${esc(r.role)}. ${r.files} files in <code>${esc(r.folder)}/</code>.</p>`
      : `<p>${esc(r.role)}. ${r.files} files in <code>${esc(r.folder)}/</code>, ${r.subphases} named decision points.</p>
        <ul>${r.modules.map((m) => module({...m, folder: r.folder})).join('')}</ul>`;
    return `<details class="arch-row${r.shared ? ' shared' : ''}"><summary>
      <span class="arch-name"><b>${r.id}</b>${esc(r.name)}</span>
      <span class="arch-bar" title="${number.format(r.loc)} lines${kunaShare != null ? `; ${number.format(r.kunaLoc)} in kuna_* modules` : ''}"><i style="width:${r.loc / max * 100}%">${r.kunaLoc != null ? `<i style="width:${r.kunaLoc / r.loc * 100}%"></i>` : ''}</i></span>
      <span class="arch-loc">${kilo(r.loc)}</span>${facts}</summary>
      <div class="arch-body">${body}<a href="${repoLink(r.spec)}">Read the specification: ${esc(r.spec.split('/').pop())}</a></div></details>`;
  }).join('');
  const total = arch.crates.reduce((sum, c) => sum + c.loc, 0);
  const engine = arch.crates.find((c) => c.name === 'kuna-decomp');
  $('arch-note').innerHTML = `Line counts include tests kept beside the code. The engine crate (<code>kuna-decomp</code>) holds ${kilo(engine.loc)} of the ${kilo(total)} lines of Rust in ${arch.crates.length} workspace crates; the rest are the loader and analyzer, the SLEIGH compiler and runtime, the command line, and the Ghidra and browser front ends. Options and passes are counted from <a href="${repoLink('decompiler/crates/kuna-decomp/phases.toml')}">the option catalog</a> and <a href="${repoLink(arch.schedule.source)}">the recorded default schedule</a>.`;
}

function renderOrder(data) {
  const {passes, loops} = data.architecture.schedule;
  const chart = $('order-chart');
  const idle = data.phases.filter((p) => !passes.some((x) => x.phase === p.id)).map((p) => p.id);
  const summary = `${passes.length} passes in the default schedule. Brackets mark the loops that repeat until nothing changes.${idle.length ? ` ${idle.join(', ')} schedules no passes; the other phases read its configuration.` : ''}`;
  const at = (i) => `${(i + .5) / passes.length * 100}%`;
  const lanes = Math.max(0, ...loops.map((l) => l.depth));
  const phaseRows = data.phases.filter((p) => !idle.includes(p.id));
  chart.style.setProperty('--lanes', lanes);
  chart.innerHTML = `<div class="order-loops" aria-hidden="true">${loops.map((l) => `<span class="order-loop" style="left:${l.start / passes.length * 100}%;width:${(l.end - l.start) / passes.length * 100}%;top:${(l.depth - 1) * 22}px">${esc(l.name)}</span>`).join('')}</div>`
    + phaseRows.map((p) => `<div class="order-row"><span class="heat-label"><b>${p.id}</b><span>${esc(p.name)}</span></span><div class="order-track" aria-hidden="true">${passes.map((x, i) => x.phase === p.id ? `<i class="${x.kind}" style="left:${at(i)}"></i>` : '').join('')}</div></div>`).join('')
    + '<div class="order-cursor" hidden></div>';
  const cursor = chart.querySelector('.order-cursor');
  const track = () => chart.querySelector('.order-track').getBoundingClientRect();
  let selected = 0;
  function inspect(index) {
    selected = index;
    const pass = passes[index];
    const box = track(), base = chart.getBoundingClientRect();
    cursor.hidden = false;
    cursor.style.left = `${box.left - base.left + (index + .5) / passes.length * box.width}px`;
    $('order-summary').innerHTML = `#${index + 1} <b>${esc(pass.name)}</b> · ${pass.kind} · ${pass.phase}${pass.group ? ` · group ${esc(pass.group)}` : ''}${pass.file ? ` · <a href="${repoLink(pass.file)}">${esc(pass.file.split('/').slice(-2).join('/'))}</a>` : ''}`;
  }
  const dismiss = () => { cursor.hidden = true; $('order-summary').textContent = summary; };
  chart.onpointermove = (event) => {
    const box = track();
    if (event.clientX < box.left) return;
    inspect(Math.max(0, Math.min(passes.length - 1, Math.floor((event.clientX - box.left) / box.width * passes.length))));
  };
  chart.onpointerleave = dismiss;
  chart.onblur = dismiss;
  chart.onfocus = () => inspect(selected);
  chart.onkeydown = (event) => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const next = event.key === 'Home' ? 0 : event.key === 'End' ? passes.length - 1 : selected + (event.key === 'ArrowLeft' ? -1 : 1);
    inspect(Math.max(0, Math.min(passes.length - 1, next)));
  };
  dismiss();
}

function renderProvenance(data) {
  const groups = new Map();
  for (const option of data.options) {
    const source = sourceKey(option.source);
    groups.set(source, (groups.get(source) || 0) + 1);
  }
  const ordered = [...groups].sort((a, b) => b[1] - a[1]);
  const max = ordered[0][1];
  $('prov-bars').innerHTML = ordered.map(([source, count]) => `<div class="prov-row"><span>${esc(sourceLabel(source))}</span><div class="prov-track"><div class="prov-fill" style="width:${count / max * 100}%"></div></div><strong>${count}</strong></div>`).join('');
  $('novel-list').innerHTML = data.records.filter((r) => r.novel).map((r) => `<a href="${repoLink(r.path)}" title="Recorded source: ${esc(sourceLabel(r.source))}">${esc(title(r.slug))}</a>`).join('');
  const select = $('catalog-source');
  select.insertAdjacentHTML('beforeend', ordered.map(([source]) => `<option value="${esc(source)}">${esc(sourceLabel(source))}</option>`).join(''));
  let limit = 12;
  const search = $('catalog-search');
  const more = $('catalog-more');
  function render() {
    const query = search.value.trim().toLowerCase();
    const source = select.value;
    const filtered = data.options.filter((o) => (!source || sourceKey(o.source) === source) && (!query || [o.name,o.phase,o.source,o.inspiration,o.summary].some((v) => String(v).toLowerCase().includes(query))));
    $('catalog-list').innerHTML = filtered.length ? filtered.slice(0, limit).map((o) => `<article class="catalog-item"><header><a href="${repoLink('decompiler/crates/kuna-decomp/phases.toml', o.line)}">${esc(o.name)}</a><span class="tag">${o.phase}</span><span class="tag">${esc(sourceLabel(o.source))}</span>${o.decbench ? '<span class="tag">decbench</span>' : ''}</header><p>${esc(o.inspiration)}</p></article>`).join('') : '<p class="catalog-empty">No options match that filter.</p>';
    more.hidden = filtered.length <= limit;
    more.textContent = `Show ${Math.min(24, filtered.length - limit)} more of ${filtered.length}`;
  }
  search.addEventListener('input', () => { limit = 12; render(); });
  select.addEventListener('change', () => { limit = 12; render(); });
  more.addEventListener('click', () => { limit += 24; render(); });
  render();
}

function renderContributions(data) {
  const {changes, checkedThrough} = data.community;
  const authored = changes.filter((c) => c.externalPr).length;
  $('community-count').textContent = number.format(changes.length);
  $('community-summary').textContent = `${authored} PRs submitted by outside contributors and ${changes.length - authored} other PRs prompted by their issue reports.`;
  $('community-list').innerHTML = changes.map((c) => `<li><a href="${REPO}/pull/${c.pr}">#${c.pr}: ${esc(c.title)}</a> <span>— ${c.externalPr ? 'PR by' : 'reported by'} <a href="https://github.com/${esc(c.contributor)}">${esc(c.contributor)}</a>${c.issues.length ? ` · ${c.issues.map((id) => `<a href="${REPO}/issues/${id}">issue #${id}</a>`).join(', ')}` : ''}</span></li>`).join('');
  $('community-note').innerHTML = `Reviewed through ${longDate.format(iso(checkedThrough))}. Each merged PR counts once, including agent-assisted submissions and partial fixes. This is a documented minimum based on the <a href="${repoLink('integrations/web/dev-viz/community.json')}">linked PRs and reports</a>.`;
}

function renderGed(data) {
  const rows = data.records.filter((r) => r.ged && Number.isFinite(r.ged.before) && Number.isFinite(r.ged.after))
    .sort((a, b) => (b.ged.before - b.ged.after) - (a.ged.before - a.ged.after));
  const max = Math.max(...rows.flatMap((r) => [r.ged.before, r.ged.after]));
  $('ged-chart').innerHTML = rows.map((r) => {
    const delta = r.ged.after - r.ged.before;
    const result = delta < 0 ? `<span class="down">${Math.abs(delta)} lower</span>` : delta === 0 ? 'unchanged' : `${delta} higher`;
    return `<div class="ged-row"><div class="ged-name"><a href="${repoLink(r.path)}">${esc(title(r.slug))}</a><small>${esc(r.ged.case)}</small></div><div class="ged-track" title="before ${r.ged.before}; after ${r.ged.after}"><span class="ged-before" style="width:${r.ged.before / max * 100}%"></span><span class="ged-after" style="width:${r.ged.after / max * 100}%"></span></div><p class="ged-value"><strong>${r.ged.before} → ${r.ged.after}</strong><br>${result}</p></div>`;
  }).join('');
}

async function main() {
  try {
    const response = await fetch('./data.json');
    if (!response.ok) throw new Error(`snapshot request failed: ${response.status}`);
    const data = await response.json();
    summarize(data); renderMilestones(data); renderPace(data); renderHeatmap(data); renderArchitecture(data); renderOrder(data); renderProvenance(data); renderContributions(data); renderGed(data);
    let timer;
    addEventListener('resize', () => { clearTimeout(timer); timer = setTimeout(() => renderPace(data), 100); });
  } catch (error) {
    $('snapshot').textContent = 'The repository snapshot could not be loaded.';
    $('data-note').textContent = String(error);
    console.error(error);
  }
}

main();
