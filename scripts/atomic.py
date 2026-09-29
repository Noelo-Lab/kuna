"""Publish complete text files with a unique sibling per writer.

This prevents partial reads, not lost read-modify-write updates: callers still
need their existing locks. Replacement does not promise crash durability.
"""

from contextlib import contextmanager
import os
from pathlib import Path
import uuid


@contextmanager
def atomic_text_writer(path, *, encoding=None, mode=None):
    """Replace path on successful exit; otherwise retain it and remove our scratch file."""
    path = Path(path)
    temporary = path.with_name(f".kuna-{uuid.uuid4().hex}.tmp")
    stream = temporary.open("x", encoding=encoding)
    try:
        with stream:
            yield stream
        if mode is not None:
            temporary.chmod(mode)
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)
