#!/usr/bin/env python3
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parent
output = root / "startup"
if output.exists():
    output.chmod(0o644)
subprocess.run(["cc", "-O1", "-fno-inline", "-fno-omit-frame-pointer", "-fno-pie", "-no-pie",
                "-Wl,-z,relro,-z,now", "-o", str(output), str(root / "startup.c")], check=True)
output.chmod(0o444)
