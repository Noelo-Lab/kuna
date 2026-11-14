"""Lint a branch's diff for what review keeps sending back.

    python -m scripts.prepr.difflint [BASE]      # BASE defaults to origin/main

Checks the changes from the merge base of BASE and HEAD to the working tree
(committed or not). Errors fail the run (exit status 1); warnings only print.

* error `fixture-readme`: a binary added under a crate's `tests/fixtures/`
  whose name the fixtures README does not mention (each row gives the recipe and
  the SHA-256).
* error `local-path`: an added binary, or an added line under a `tests/`
  directory, carrying a local path (`/home/<user>/`, `/Users/<user>/`,
  `C:\\Users\\`, `.claude/jobs/`): build intermediates and recorded output leak
  where they were made.
* error `intermediate`: an added `.obj`, `.o` or `.lib` that no test, stage
  test or script names. A compiler intermediate belongs in the build recipe,
  not the tree.
* error `assertion-edit`: a `<stringmatch>` line removed or changed in a stage
  test that already exists on BASE. An expectation that moves is a behaviour
  change to justify, not a test to adjust: add a commit-message trailer
  `Assertion-change: <test name>: <why the new expectation is right>`.
  `kuna-catalog.xml` is exempt: every new option bumps its registry counts.
* warning `option-policy`: a new `kuna_*.rs` module with no `phases.toml`
  change. That is right only for a strict correctness fix (docs/agents.md,
  "Adding features").
"""
import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Optional

LOCAL_PATH = re.compile(rb"/home/[A-Za-z0-9._-]+/|/Users/[A-Za-z0-9._-]+/|[A-Za-z]:\\Users\\|\.claude/jobs/")
INTERMEDIATE = {".obj", ".o", ".lib"}
TRAILER = re.compile(r"^Assertion-change:\s*(.+)$", re.M | re.I)
COUNTED_REGISTRY = {"kuna-catalog"}


@dataclass
class Finding:
    level: str
    check: str
    path: str
    message: str

    def __str__(self) -> str:
        return f"{self.level}: {self.check}: {self.path}: {self.message}"


def git(root: Path, *args: str) -> str:
    return subprocess.run(["git", "-C", str(root), *args], capture_output=True, text=True, check=True).stdout


def is_binary(data: bytes) -> bool:
    return b"\0" in data[:8192]


def changed_files(root: Path, base: str) -> dict[str, str]:
    """Each path changed since the merge base, mapped to its status letter (A, M, D, R...)."""
    merge_base = git(root, "merge-base", base, "HEAD").strip()
    out = {}
    for line in git(root, "diff", "--name-status", "--no-renames", merge_base).splitlines():
        status, path = line.split("\t", 1)
        out[path] = status[0]
    for path in git(root, "ls-files", "--others", "--exclude-standard").splitlines():
        out[path] = "A"
    return out


def removed_lines(root: Path, base: str, path: str) -> list[str]:
    merge_base = git(root, "merge-base", base, "HEAD").strip()
    diff = git(root, "diff", "-U0", merge_base, "--", path)
    return [line[1:] for line in diff.splitlines() if line.startswith("-") and not line.startswith("---")]


def added_lines(root: Path, base: str, path: str) -> list[str]:
    merge_base = git(root, "merge-base", base, "HEAD").strip()
    diff = git(root, "diff", "-U0", merge_base, "--", path)
    lines = [line[1:] for line in diff.splitlines() if line.startswith("+") and not line.startswith("+++")]
    if not lines and (root / path).is_file() and not git(root, "ls-files", path).strip():
        lines = (root / path).read_text(errors="replace").splitlines()
    return lines


def referenced(root: Path, name: str) -> bool:
    """Whether a test, stage test or script mentions `name`."""
    hits = subprocess.run(
        ["git", "-C", str(root), "grep", "-l", "--untracked", "-F", name, "--",
         "tests", "*.rs", "*.py", "*.xml", "*.s", "*.asm"],
        capture_output=True, text=True,
    ).stdout.split()
    return any(not hit.endswith(name) for hit in hits)


def lint(root: Path, base: str) -> list[Finding]:
    findings = []
    changes = changed_files(root, base)
    commit_messages = git(root, "log", "--format=%B", f"{git(root, 'merge-base', base, 'HEAD').strip()}..HEAD")
    justified = TRAILER.findall(commit_messages)
    readme_cache: dict[Path, str] = {}

    for path, status in sorted(changes.items()):
        file = root / path
        posix = PurePosixPath(path)
        if status == "A" and file.is_file():
            data = file.read_bytes()
            binary = is_binary(data)
            if binary and "fixtures" in posix.parts and "tests" in posix.parts:
                fixtures = Path(*posix.parts[: posix.parts.index("fixtures") + 1])
                readme = root / fixtures / "README.md"
                if readme not in readme_cache:
                    readme_cache[readme] = readme.read_text(errors="replace") if readme.is_file() else ""
                if posix.name not in readme_cache[readme]:
                    findings.append(Finding("error", "fixture-readme", path,
                                            f"not listed in {fixtures}/README.md (recipe and SHA-256)"))
            if binary and LOCAL_PATH.search(data):
                findings.append(Finding("error", "local-path", path,
                                        f"embeds {LOCAL_PATH.search(data).group().decode(errors='replace')!r}"))
            if posix.suffix.lower() in INTERMEDIATE and not referenced(root, posix.name):
                findings.append(Finding("error", "intermediate", path,
                                        "a build intermediate nothing reads; keep it in the build recipe"))
        if status in "AM" and file.is_file() and "tests" in posix.parts and not is_binary(file.read_bytes()):
            for line in added_lines(root, base, path):
                hit = LOCAL_PATH.search(line.encode(errors="replace"))
                if hit:
                    findings.append(Finding("error", "local-path", path,
                                            f"added line embeds {hit.group().decode(errors='replace')!r}"))
                    break
        if status == "M" and posix.parent == PurePosixPath("tests/stages") and posix.suffix == ".xml":
            edited = [line for line in removed_lines(root, base, path) if "<stringmatch" in line]
            stem = posix.stem
            if edited and stem not in COUNTED_REGISTRY and not any(stem in reason for reason in justified):
                findings.append(Finding("error", "assertion-edit", path,
                                        f"{len(edited)} existing assertion(s) changed; justify with a commit trailer "
                                        f"'Assertion-change: {stem}: <reason>'"))

    new_modules = [p for p, s in changes.items()
                   if s == "A" and re.search(r"(^|/)kuna_[a-z0-9_]+\.rs$", p) and "/tests" not in p]
    if new_modules and not any(p.endswith("phases.toml") for p in changes):
        for path in new_modules:
            findings.append(Finding("warning", "option-policy", path,
                                    "new module without a phases.toml row: right only for a strict correctness fix"))
    return findings


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("base", nargs="?", default="origin/main")
    parser.add_argument("--root", type=Path, default=Path.cwd())
    args = parser.parse_args(argv)
    findings = lint(args.root.resolve(), args.base)
    for finding in findings:
        print(finding)
    errors = [f for f in findings if f.level == "error"]
    if errors:
        print(f"FAIL: {len(errors)} error(s)", file=sys.stderr)
        return 1
    print(f"OK ({len(findings)} warning(s))")
    return 0


if __name__ == "__main__":
    sys.exit(main())
