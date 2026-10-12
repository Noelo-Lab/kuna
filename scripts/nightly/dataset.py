"""Pick the nightly sample from the published decbench dataset and fetch only its files.

The dataset lives on Hugging Face (`noelo-lab/decbench-dataset`); every read is pinned
to one dataset revision so a dataset refresh can never move the nightly numbers on its
own. The sample is a deterministic, project-stratified set of binaries from one config
(default `optimized`, the O2-noinline functions): projects take turns contributing
their next binary, in an order fixed by the seed, until the sample is full. A binary is
the unit because kuna decompiles a whole binary at once; every function the config
scores in a sampled binary is scored.

    python3 -m scripts.nightly.dataset --revision <sha> --binaries 96 \
        --data data/ --sample-out sample.json

Stdlib only, so it runs before the decbench environment exists.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import json
import os
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

REPO_ID = "noelo-lab/decbench-dataset"
HF_BASE = "https://huggingface.co/datasets"


def resolve_url(repo_id: str, revision: str, path: str) -> str:
    return f"{HF_BASE}/{repo_id}/resolve/{revision}/{path}"


def fetch(url: str, dest: Path, sha256: str | None = None, size: int | None = None,
          attempts: int = 5) -> Path:
    """Download `url` to `dest` (atomic), skipping a present file that already verifies."""
    if dest.is_file() and _verifies(dest, sha256, size):
        return dest
    dest.parent.mkdir(parents=True, exist_ok=True)
    headers = {"User-Agent": "kuna-nightly"}
    token = os.environ.get("HF_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    tmp = dest.with_name(dest.name + ".part")
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(urllib.request.Request(url, headers=headers),
                                        timeout=120) as resp, open(tmp, "wb") as out:
                while chunk := resp.read(1 << 20):
                    out.write(chunk)
            if not _verifies(tmp, sha256, size):
                raise OSError(f"checksum mismatch for {url}")
            tmp.replace(dest)
            return dest
        except (urllib.error.URLError, OSError, TimeoutError) as e:
            if attempt == attempts:
                raise RuntimeError(f"download failed after {attempts} attempts: {url}: {e}") from e
            retry_after = getattr(e, "headers", None) and e.headers.get("Retry-After")
            time.sleep(int(retry_after) if str(retry_after).isdigit() else 2 ** attempt)
    raise AssertionError("unreachable")


def _verifies(path: Path, sha256: str | None, size: int | None) -> bool:
    if size is not None and path.stat().st_size != size:
        return False
    if sha256 is None:
        return True
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(1 << 20):
            h.update(chunk)
    return h.hexdigest() == sha256


def _rank(seed: str, key: str) -> str:
    return hashlib.sha256(f"{seed}:{key}".encode()).hexdigest()


def select(manifest: dict, count: int, seed: str) -> list[dict]:
    """Project-stratified round robin over the config's binaries, ordered by `seed`."""
    by_project: dict[str, list[dict]] = {}
    for entry in manifest["binaries"]:
        if entry.get("functions") and entry.get("source_cfg_path"):
            by_project.setdefault(entry["project"], []).append(entry)
    queues = {
        project: sorted(entries, key=lambda e: _rank(seed, e["binary_path"]))
        for project, entries in by_project.items()
    }
    order = sorted(queues, key=lambda p: _rank(seed, p))
    picked: list[dict] = []
    while len(picked) < count and any(queues.values()):
        for project in order:
            if queues[project] and len(picked) < count:
                picked.append(queues[project].pop(0))
    return sorted(picked, key=lambda e: e["binary_path"])


def sample_digest(revision: str, config: str, binaries: list[dict]) -> str:
    """Identity of a sample: the same digest on two nights means a like-for-like comparison."""
    h = hashlib.sha256(f"{REPO_ID}@{revision}/{config}".encode())
    for entry in binaries:
        h.update(f"\n{entry['binary_path']}:{entry['sha256']}:{','.join(entry['functions'])}".encode())
    return h.hexdigest()[:16]


def build_sample(manifest: dict, revision: str, config: str, count: int, seed: str) -> dict:
    binaries = select(manifest, count, seed)
    projects = sorted({b["project"] for b in binaries})
    return {
        "dataset": REPO_ID,
        "revision": revision,
        "config": config,
        "seed": seed,
        "requested": count,
        "digest": sample_digest(revision, config, binaries),
        "function_count": sum(len(b["functions"]) for b in binaries),
        "binaries": [
            {k: b[k] for k in ("project", "opt", "binary", "binary_path", "sha256", "size",
                               "source_cfg_path", "functions")}
            for b in binaries
        ],
        "sources": {p: manifest["projects"].get(p, {}).get("sources", []) for p in projects},
    }


def download(sample: dict, data: Path, jobs: int = 8) -> None:
    """Fetch every binary and source CFG the sample names.

    The project sources are deliberately not fetched: they are thousands of small
    files (Hugging Face answers a full set of anonymous reads with HTTP 429), and
    scoring needs only their names, which the manifest carries.
    """
    rev = sample["revision"]
    work = []
    for b in sample["binaries"]:
        work.append((b["binary_path"], b["sha256"], b["size"]))
        work.append((b["source_cfg_path"], None, None))
    with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
        futures = [
            pool.submit(fetch, resolve_url(sample["dataset"], rev, path), data / path, sha, size)
            for path, sha, size in work
        ]
        for f in concurrent.futures.as_completed(futures):
            f.result()


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--revision", required=True, help="pinned dataset commit sha")
    ap.add_argument("--config", default="optimized")
    ap.add_argument("--binaries", type=int, default=96, help="sample size, in binaries")
    ap.add_argument("--seed", default="kuna-nightly-v1")
    ap.add_argument("--data", type=Path, required=True, help="download root (mirrors the repo)")
    ap.add_argument("--sample-out", type=Path, required=True)
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args(argv)

    manifest_path = f"configs/{args.config}/manifest.json"
    local = fetch(resolve_url(REPO_ID, args.revision, manifest_path), args.data / manifest_path)
    manifest = json.loads(local.read_text())
    sample = build_sample(manifest, args.revision, args.config, args.binaries, args.seed)
    download(sample, args.data, args.jobs)
    args.sample_out.parent.mkdir(parents=True, exist_ok=True)
    args.sample_out.write_text(json.dumps(sample, indent=1) + "\n")
    print(f"sample {sample['digest']}: {len(sample['binaries'])} binaries, "
          f"{sample['function_count']} functions, {len(sample['sources'])} projects "
          f"from {args.config}@{args.revision[:10]}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
