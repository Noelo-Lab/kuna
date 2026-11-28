"""Shared helpers for the versioned function-oracle JSON protocol."""

import hashlib
import json

MAX_MAPPED = 16 * 1024 * 1024
MAX_REQUEST = 6 * MAX_MAPPED + 2 * 1024 * 1024
DEFAULT_LIMITS = {"instructions": 10_000, "timeout_us": 1_000_000}


def case_digest(spec):
    canonical = json.dumps(spec, sort_keys=True, separators=(",", ":"), ensure_ascii=True)
    return hashlib.sha256(canonical.encode("ascii")).hexdigest()


def load_request(file):
    data = file.read(MAX_REQUEST + 1)
    if len(data) > MAX_REQUEST:
        raise ValueError("request exceeds the byte limit")
    decoded = json.loads(data)
    if not isinstance(decoded, dict):
        raise ValueError("request must be a JSON object")
    return decoded
