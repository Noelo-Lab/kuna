#!/usr/bin/env python3
from pathlib import Path
import subprocess
r = Path(__file__).resolve().parent
out = r / "immutable_got_calls_x86_64"
if out.exists():
    out.chmod(0o644)
subprocess.run(["cc", "-O1", "-fPIE", "-pie", "-Wl,-z,relro,-z,now", "-o", str(out),
                str(r / "immutable_got_calls.c"), str(r / "immutable_got_calls.S")], check=True)
out.chmod(0o444)
