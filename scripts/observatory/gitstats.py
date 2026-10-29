"""One-pass git history reader for the churn page: numstat chunks with rename
resolution and agent attribution. The parsing rules are adapted from
integrations/web/dev-viz/generate.py (kept separate on purpose: that script
belongs to the public site build), with unit tests this copy owns."""
import re
import subprocess
from pathlib import Path

LOG_FORMAT = "%x1e%H%x1f%cI%x1f%an%x1f%B%x1f"
_RENAME = re.compile(r"\{[^{}]* => ([^{}]*)\}")
_COAUTHOR = re.compile(r"^co-authored-by:", re.I)
_AGENT = re.compile(r"\b(claude|codex|chatgpt|copilot)\b", re.I)


def destination(path: str) -> str:
    """Resolve numstat rename syntax (``a/{b => c}/d``, ``a => b``)."""
    path = _RENAME.sub(r"\1", path)
    return path.rsplit(" => ", 1)[-1]


def has_agent_attribution(message: str) -> bool:
    if "[AUTOMATED]" in message.upper():
        return True
    return any(
        _AGENT.search(line) for line in message.splitlines() if _COAUTHOR.match(line)
    )


def read_log(root: Path) -> str:
    return subprocess.run(
        ["git", "log", "HEAD", "--no-merges", "--find-renames", "--numstat",
         f"--format={LOG_FORMAT}"],
        cwd=root,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=True,
    ).stdout


def parse_commits(log: str) -> list:
    """Yield {sha, date(ISO day), author, agent, files:[(path, added, removed)]}."""
    commits = []
    for chunk in log.split("\x1e"):
        if not chunk.strip():
            continue
        sha, stamp, author, message, stats = chunk.split("\x1f", 4)
        files = []
        for line in stats.splitlines():
            fields = line.split("\t", 2)
            if len(fields) != 3 or fields[0] == "-" or fields[1] == "-":
                continue
            files.append((destination(fields[2]), int(fields[0]), int(fields[1])))
        commits.append(
            {
                "sha": sha,
                "date": stamp[:10],
                "author": author,
                "agent": has_agent_attribution(message),
                "files": files,
            }
        )
    return commits
