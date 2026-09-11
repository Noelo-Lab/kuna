#!/usr/bin/env python3
"""Acquire or verify the inherited process-lifetime repipe supervisor lock."""

import argparse
import fcntl
import os
from pathlib import Path
import sys


CONFLICT_EXIT = 1
FD_ENV = "REPIPE_SUPERVISOR_LOCK_FD"
OWNER_ENV = "REPIPE_SUPERVISOR_LOCK_OWNER_PID"


def verify_inherited(lock_path, owner_pid):
    """Verify that owner_pid is this caller's parent and holds lock_path via its inherited FD."""
    try:
        fd = int(os.environ[FD_ENV])
        recorded_owner = int(os.environ[OWNER_ENV])
    except (KeyError, TypeError, ValueError):
        return False
    if fd < 3 or recorded_owner != owner_pid or os.getppid() != owner_pid:
        return False
    try:
        held = os.fstat(fd)
        expected = Path(lock_path).stat()
        if (held.st_dev, held.st_ino) != (expected.st_dev, expected.st_ino):
            return False
        # An inherited duplicate shares the owner's open-file description, so re-locking is a
        # no-op. A forged descriptor opened independently conflicts with the real owner.
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except (BlockingIOError, OSError):
        return False
    return True


def acquire_and_exec(lock_path, command):
    """Acquire lock_path and exec command as its owner, or fail without spawning command."""
    path = Path(lock_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    lock = open(path, "a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        lock.close()
        print("another repipe supervisor already owns the lifetime lock", file=sys.stderr)
        return CONFLICT_EXIT

    owner_pid = os.getpid()
    lock.seek(0)
    lock.truncate()
    lock.write("%s\n" % owner_pid)
    lock.flush()
    # Use a high descriptor so Bash cannot reuse the capability for script/input plumbing.
    capability_fd = fcntl.fcntl(lock.fileno(), fcntl.F_DUPFD, 100)
    os.set_inheritable(capability_fd, True)
    lock.close()
    env = os.environ.copy()
    env[FD_ENV] = str(capability_fd)
    env[OWNER_ENV] = str(owner_pid)
    try:
        os.execvpe(command[0], command, env)
    except OSError as exc:
        print("could not exec repipe supervisor: %s" % exc, file=sys.stderr)
        os.close(capability_fd)
        return 126


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lock", required=True)
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--owner-pid", type=int)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)

    if args.verify:
        if args.owner_pid is None or args.command:
            parser.error("--verify requires --owner-pid and no command")
        return 0 if verify_inherited(args.lock, args.owner_pid) else 1

    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if args.owner_pid is not None or not command:
        parser.error("a command is required after --")
    return acquire_and_exec(args.lock, command)


if __name__ == "__main__":
    raise SystemExit(main())
