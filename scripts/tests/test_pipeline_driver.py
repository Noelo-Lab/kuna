import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest


ROOT = Path(__file__).resolve().parents[2]
DRIVER = ROOT / "tools/pipeline/run.sh"
STUB = r'''
import os
from pathlib import Path
import sys

root = Path(os.environ["KUNA_REPO"])
scenario = os.environ["DRIVER_TEST_SCENARIO"]
command = Path(sys.argv[0]).name
arguments = sys.argv[1:]
with (root / "commands.log").open("a") as output:
    print(command, *arguments, file=output)

def invocation(name):
    path = root / (name + ".count")
    count = int(path.read_text()) + 1 if path.exists() else 1
    path.write_text(str(count))
    return count

if command == "git":
    if arguments not in [["-C", str(root), "worktree", "list", "--porcelain"],
                         ["-C", str(root), "worktree", "prune"]]:
        (root / "unexpected-git").write_text(repr(arguments))
        sys.exit(99)
    sys.exit(0)
if command == "gh":
    (root / "unexpected-gh").write_text(repr(arguments))
    sys.exit(99)
if command == "df":
    print("Avail")
    print("1G" if invocation("disk") == 1 else "100G")
    sys.exit(0)
if arguments == ["-m", "scripts.pipeline.status"]:
    sys.exit(0)
if arguments == ["-m", "scripts.pipeline.state", "reap"]:
    if scenario == "reap_error":
        print("reaper diagnostic", file=sys.stderr)
        sys.exit(2)
    sys.exit(0)
if arguments[:3] == ["-m", "scripts.pipeline.state", "claim"]:
    count = invocation("claim")
    if scenario == "claim_error":
        print("claim diagnostic", file=sys.stderr)
        sys.exit(2)
    sys.exit(1 if scenario == "claim_race" and count == 1 else 0)
if arguments == ["-m", "scripts.pipeline.select", "--shell"]:
    count = invocation("select")
    if scenario == "real_corrupt":
        os.execv(sys.executable, [sys.executable, *arguments])
    if scenario in ["empty", "reap_error"]:
        sys.exit(1)
    if scenario == "select_error" or (scenario == "drain_error" and count > 1):
        (root / "dispatch-error").touch()
        print("selector diagnostic", file=sys.stderr)
        sys.exit(2)
    print("OPP_ID='opportunity'; TEST_NAME='case'; BINARY='fixture'; "
          "SELECTOR='main'; ARCH='test'; SLUG='case'; SCORE='4'; KINDS='structural'")
    sys.exit(0)
(root / "unexpected-command").write_text(repr(arguments))
sys.exit(99)
'''


class DriverTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-driver-test-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        commands = self.root / "bin"
        commands.mkdir()
        for name in ["git", "gh", "df", "python"]:
            path = commands / name
            path.write_text(f"#!{sys.executable}\n" + STUB)
            path.chmod(0o755)
        worker = self.root / "worker.sh"
        worker.write_text(
            '#!/usr/bin/env bash\n'
            'touch "$KUNA_REPO/worker-started"\n'
            'if [ "$DRIVER_TEST_SCENARIO" = drain_error ]; then\n'
            '  for ((attempt=0; attempt<1000; attempt++)); do\n'
            '    [ -f "$KUNA_REPO/release-worker" ] && break\n'
            '    sleep 0.01\n'
            '  done\n'
            '  [ -f "$KUNA_REPO/release-worker" ] || exit 2\n'
            'fi\n'
            'touch "$KUNA_REPO/worker-finished"\n'
        )
        self.environment = dict(os.environ, **{
            "PATH": str(commands) + os.pathsep + os.environ["PATH"],
            "KUNA_REPO": str(self.root),
            "KUNA_ROOT": str(self.root),
            "KUNA_PY": str(commands / "python"),
            "PYTHONPATH": str(ROOT),
            "KUNA_PIPELINE_DOCS_DIR": str(self.root / "docs"),
            "PIPELINE_STATE_DIRNAME": "state",
            "PIPELINE_SELECT_MOD": "scripts.pipeline.select",
            "PIPELINE_WORKER_SCRIPT": "worker.sh",
            "PIPELINE_WORKTREE_MATCH": "/state/worktrees/",
            "PIPELINE_BRANCH_MATCH": "test/",
            "PIPELINE_WORKERS": "2",
            "PIPELINE_HOURS": "0",
            "PIPELINE_POLL": "0.01",
            "PIPELINE_MIN_FREE_GB": "0",
        })

    def run_driver(self, scenario, *, once=True):
        environment = dict(self.environment, DRIVER_TEST_SCENARIO=scenario)
        if scenario == "disk_brake":
            environment["PIPELINE_MIN_FREE_GB"] = "2"
        result = subprocess.run(
            ["bash", str(DRIVER), *(["--once"] if once else [])],
            env=environment, cwd=self.root, capture_output=True, text=True,
            timeout=10, check=False,
        )
        for marker in ["unexpected-git", "unexpected-gh", "unexpected-command"]:
            self.assertFalse((self.root / marker).exists(), marker)
        return result

    def test_an_empty_selector_is_the_only_drained_backlog(self):
        result = self.run_driver("empty")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("backlog drained and no active workers", result.stdout)
        self.assertFalse((self.root / "worker-started").exists())

    def test_selector_errors_preserve_diagnostics_and_fail(self):
        result = self.run_driver("select_error")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("selector diagnostic", result.stderr)
        self.assertNotIn("backlog drained", result.stdout)
        self.assertFalse((self.root / "worker-started").exists())

    def test_claim_errors_are_not_claim_races(self):
        result = self.run_driver("claim_error")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("claim diagnostic", result.stderr)
        self.assertNotIn("backlog drained", result.stdout)
        self.assertNotIn("already claimed", result.stdout)
        self.assertFalse((self.root / "worker-started").exists())

    def test_real_selector_corruption_reaches_the_driver_exit_status(self):
        directory = self.root / "docs"
        directory.mkdir()
        backlog = directory / "opportunities.json"
        backlog.write_text("not JSON")
        result = self.run_driver("real_corrupt")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("cannot read backlog", result.stderr)
        self.assertNotIn("Traceback", result.stderr)
        self.assertNotIn("backlog drained", result.stdout)
        self.assertFalse((self.root / "worker-started").exists())
        self.assertEqual(backlog.read_text(), "not JSON")

    def test_reaper_errors_stop_dispatch(self):
        result = self.run_driver("reap_error")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("reaper diagnostic", result.stderr)
        self.assertNotIn("backlog drained", result.stdout)
        self.assertFalse((self.root / "select.count").exists())

    def test_claim_races_retry_instead_of_reporting_a_drained_backlog(self):
        result = self.run_driver("claim_race")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual((self.root / "claim.count").read_text(), "2")
        self.assertTrue((self.root / "worker-finished").exists())
        self.assertNotIn("backlog drained", result.stdout)

    def test_disk_brakes_retry_instead_of_reporting_a_drained_backlog(self):
        result = self.run_driver("disk_brake")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual((self.root / "disk.count").read_text(), "2")
        self.assertTrue((self.root / "worker-finished").exists())
        self.assertIn("disk brake", result.stdout)
        self.assertNotIn("backlog drained", result.stdout)

    def test_dispatch_errors_wait_for_in_flight_workers(self):
        environment = dict(self.environment, DRIVER_TEST_SCENARIO="drain_error")
        process = subprocess.Popen(
            ["bash", str(DRIVER)], env=environment, cwd=self.root,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        try:
            deadline = time.monotonic() + 5
            while not all((self.root / marker).exists()
                          for marker in ["dispatch-error", "worker-started"]):
                self.assertIsNone(process.poll(), "driver exited before the dispatch error")
                self.assertLess(time.monotonic(), deadline, "driver never reached the error")
                time.sleep(0.01)
            self.assertTrue((self.root / "worker-started").exists())
            self.assertFalse((self.root / "worker-finished").exists())
            with self.assertRaises(subprocess.TimeoutExpired):
                process.wait(timeout=0.3)
        finally:
            (self.root / "release-worker").touch()
            stdout, stderr = process.communicate(timeout=10)
        self.assertEqual(process.returncode, 2, stdout + stderr)
        self.assertTrue((self.root / "worker-finished").exists())
        self.assertIn("waiting for in-flight workers", stdout)
        self.assertNotIn("backlog drained", stdout)
        for marker in ["unexpected-git", "unexpected-gh", "unexpected-command"]:
            self.assertFalse((self.root / marker).exists(), marker)


if __name__ == "__main__":
    unittest.main()
