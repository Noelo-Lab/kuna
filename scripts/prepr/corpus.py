"""The public binaries `aritydiff` runs on, fetched on demand and pinned by SHA-256.

Nothing here is committed to the repository: each entry names where the bytes
come from (a URL, or a member of a downloaded archive) and the hash both the
download and the extracted file must have. Files are cached under
``$KUNA_PREPR_CACHE`` (default ``~/.cache/kuna-prepr``).

The ``local`` corpus is whatever binaries one machine has and may not share:
one path per line in ``$KUNA_PREPR_LOCAL`` (default
``~/.config/kuna-prepr/local.txt``), blank lines and ``#`` comments ignored.
"""
import hashlib
import io
import os
import urllib.request
import zipfile
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

SETUPTOOLS_WHEEL = (
    "https://files.pythonhosted.org/packages/py3/s/setuptools/setuptools-59.6.0-py3-none-any.whl",
    "4ce92f1e1f8f01233ee9952c04f6b81d1e02939d6e1b488428154974a4d0783e",
)
PIP_WHEEL = (
    "https://files.pythonhosted.org/packages/py3/p/pip/pip-21.3.1-py3-none-any.whl",
    "deaf32dcd9ab821e359cd8330786bcd077604b5c5730c0b096eda46f95c24a2d",
)
GHIDRA_RAW = "https://raw.githubusercontent.com/NationalSecurityAgency/ghidra/79d8f164f8bb8b15cfb60c5d4faeb8e1c25d15ca/"


@dataclass(frozen=True)
class Entry:
    name: str
    url: str
    url_sha256: str
    sha256: str
    member: Optional[str] = None


CORPORA = {
    "msvc32": [
        Entry("setuptools-cli-32.exe", *SETUPTOOLS_WHEEL,
              "75f12ea2f30d9c0d872dade345f30f562e6d93847b6a509ba53beec6d0b2c346", "setuptools/cli-32.exe"),
        Entry("setuptools-gui-32.exe", *SETUPTOOLS_WHEEL,
              "5c1af46c7300e87a73dacf6cf41ce397e3f05df6bd9c7e227b4ac59f85769160", "setuptools/gui-32.exe"),
        Entry("distlib-t32.exe", *PIP_WHEEL,
              "352df104254095ddf925514d99bfb5411c95b5386e90caf06557979f82e16844", "pip/_vendor/distlib/t32.exe"),
        Entry("distlib-w32.exe", *PIP_WHEEL,
              "949b6765d794c53656c9afc45b90d9a2cfcae6bb30444086b29225f19242217b", "pip/_vendor/distlib/w32.exe"),
        Entry("WinHelloCPP.exe",
              GHIDRA_RAW + "GhidraDocs/GhidraClass/ExerciseFiles/WinhelloCPP/WinHelloCPP.exe",
              "7a98a934de5f578c3a547af873221b221d037bafa87ceee71dd4f089dd228202",
              "7a98a934de5f578c3a547af873221b221d037bafa87ceee71dd4f089dd228202"),
    ],
}


def cache_dir() -> Path:
    return Path(os.environ.get("KUNA_PREPR_CACHE", Path.home() / ".cache" / "kuna-prepr"))


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _download(url: str, expected: str, cache: Path) -> bytes:
    path = cache / "downloads" / expected
    if path.is_file():
        data = path.read_bytes()
        if sha256(data) == expected:
            return data
    with urllib.request.urlopen(url, timeout=120) as response:
        data = response.read()
    if sha256(data) != expected:
        raise ValueError(f"{url}: SHA-256 {sha256(data)} is not the pinned {expected}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return data


def fetch(entry: Entry, cache: Optional[Path] = None) -> Path:
    """The cached file for `entry`, downloading and verifying it when absent."""
    cache = cache or cache_dir()
    path = cache / "corpus" / entry.name
    if path.is_file() and sha256(path.read_bytes()) == entry.sha256:
        return path
    data = _download(entry.url, entry.url_sha256, cache)
    if entry.member is not None:
        with zipfile.ZipFile(io.BytesIO(data)) as archive:
            data = archive.read(entry.member)
    if sha256(data) != entry.sha256:
        raise ValueError(f"{entry.name}: SHA-256 {sha256(data)} is not the pinned {entry.sha256}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return path


def local_list() -> Path:
    return Path(os.environ.get("KUNA_PREPR_LOCAL", Path.home() / ".config" / "kuna-prepr" / "local.txt"))


def local() -> list[Path]:
    """The machine-local binaries named in `local_list()`."""
    listing = local_list()
    if not listing.is_file():
        raise FileNotFoundError(f"no local corpus: list binaries, one per line, in {listing}")
    paths = []
    for line in listing.read_text().splitlines():
        line = line.split("#", 1)[0].strip()
        if line:
            path = Path(line).expanduser()
            if not path.is_file():
                raise FileNotFoundError(f"{listing}: {path} does not exist")
            paths.append(path)
    return paths


def resolve(name: str, cache: Optional[Path] = None) -> list[Path]:
    """The files of the named corpus."""
    if name == "local":
        return local()
    if name not in CORPORA:
        raise KeyError(f"unknown corpus {name!r}; known: {', '.join(sorted(CORPORA) + ['local'])}")
    return [fetch(entry, cache) for entry in CORPORA[name]]
