/* Shared observatory helpers: inlined-data access, element building, nav. */
"use strict";
function dataOf(name) {
  const node = document.getElementById("data-" + name);
  return node ? JSON.parse(node.textContent) : null;
}
function esc(text) {
  return String(text).replace(/[&<>"]/g, (ch) =>
    ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[ch]));
}
function el(tag, attrs, ...children) {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(attrs || {})) {
    if (key === "html") node.innerHTML = value;
    else node.setAttribute(key, value);
  }
  for (const child of children) {
    node.append(child instanceof Node ? child : document.createTextNode(child));
  }
  return node;
}
function fmt(number) { return Number(number).toLocaleString("en-US"); }
function phaseColor(phase) {
  const known = /^(P\d|substrate|infra)$/.test(phase || "");
  return known ? `var(--ph-${phase})` : "var(--baseline)";
}
function phaseChip(phase, label) {
  const text = label || phase || "—";
  const dot = `<span class="dot" style="background:${phaseColor(phase)}"></span>`;
  return `<span class="chip">${dot}${esc(text)}</span>`;
}
function renderTopbar(active) {
  const status = dataOf("status") || { datasets: {} };
  const links = Object.entries(status.datasets).map(([name, entry]) => {
    const label = (entry.title || name).split(" ")[0].replace(/^./, (ch) => ch.toUpperCase());
    const here = active === name ? " here" : "";
    const off = entry.available ? "" : " off";
    const title = entry.available ? "" : ` title="not generated yet — run: ${esc(entry.regen || "make observatory")}"`;
    return `<a class="${here}${off}" href="./${name}.html"${title}>${esc(label)}${entry.available ? "" : " ·"}</a>`;
  }).join("");
  const atlasMeta = (status.datasets.atlas || {}).meta || {};
  const sha = atlasMeta.sha ? atlasMeta.sha.slice(0, 10) : "";
  document.body.insertAdjacentHTML(
    "afterbegin",
    `<header class="topbar"><div class="wrap">
       <a class="brand" href="./index.html">kuna <span>observatory</span></a>
       <nav>${links}</nav>
       <span class="meta">${esc(sha)} · ${esc(status.generated || "")}</span>
     </div></header>`);
}
function missingNotice(name) {
  const status = dataOf("status") || { datasets: {} };
  const entry = status.datasets[name] || { regen: "make observatory" };
  return `<div class="missing">This page's data has not been generated yet.
    Run <code>${esc(entry.regen)}</code> from the repo root, then reload.</div>`;
}
