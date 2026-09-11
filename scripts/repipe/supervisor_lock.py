#!/usr/bin/env python3
"""Run one supervisor command while holding its process-lifetime ownership lock."""

import argparse
import ctypes
import fcntl
import os
from pathlib import Path
import signal
import subprocess
import sys


CONFLICT_EXIT = 1
PR_SET_PDEATHSIG = 1


def _kill_if_owner_dies(expected_parent):
    """Keep a SIGKILLed lock runner from leaving its supervisor loop orphaned on Linux."""
    if not sys.platform.startswith("linux"):
        return
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(PR_SET_PDEATHSIG, signal.SIGKILL, 0, 0, 0) != 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))
    # Close the fork-to-prctl race: if ownership already disappeared, do not exec the loop.
    if os.getppid() != expected_parent:
        os.kill(os.getpid(), signal.SIGKILL)


def run_locked(lock_path, command):
    """Run command while exclusively owning lock_path, or fail without spawning it."""
    path = Path(lock_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "a+") as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            print("another repipe supervisor already owns the lifetime lock", file=sys.stderr)
            return CONFLICT_EXIT

        child = None
        pending = []

        def forward(signum, _frame):
            if child is None:
                pending.append(signum)
            elif child.poll() is None:
                try:
                    child.send_signal(signum)
                except ProcessLookupError:
                    pass

        previous = {
            signum: signal.getsignal(signum) for signum in (signal.SIGINT, signal.SIGTERM)
        }
        for signum in previous:
            signal.signal(signum, forward)
        try:
            # Python lock descriptors are non-inheritable. close_fds also makes the ownership
            # boundary explicit: captain/worker descendants cannot accidentally retain it.
            owner_pid = os.getpid()
            child = subprocess.Popen(
                command, close_fds=True,
                preexec_fn=lambda: _kill_if_owner_dies(owner_pid),
            )
            for signum in pending:
                if child.poll() is None:
                    try:
                        child.send_signal(signum)
                    except ProcessLookupError:
                        pass
            returncode = child.wait()
            return returncode if returncode >= 0 else 128 - returncode
        finally:
            for signum, handler in previous.items():
                signal.signal(signum, handler)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lock", required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command:
        parser.error("a command is required after --")
    return run_locked(args.lock, command)


if __name__ == "__main__":
    raise SystemExit(main())
