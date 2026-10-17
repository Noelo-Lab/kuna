import signal
import sys
import tempfile
import unittest
from unittest.mock import patch
from pathlib import Path

from scripts.huge_function_bench import measure


class MeasurementTests(unittest.TestCase):
    def test_failed_child_keeps_diagnostics_and_status(self):
        with tempfile.TemporaryDirectory() as directory:
            stem = Path(directory) / "fixture with spaces"
            result = measure([sys.executable, "-c",
                              "import sys; print('diagnostic', file=sys.stderr); sys.exit(7)"],
                             stem, 5)
            self.assertEqual(result["exit_code"], 7)
            self.assertFalse(result["timed_out"])
            self.assertIn("diagnostic", stem.with_suffix(".stderr").read_text())
            self.assertGreater(result["peak_rss_kib"], 0)

    def test_deadline_kills_a_stalled_child(self):
        with tempfile.TemporaryDirectory() as directory:
            result = measure([sys.executable, "-c", "import time; time.sleep(60)"],
                             Path(directory) / "fixture", 0.1)
            self.assertTrue(result["timed_out"])
            self.assertEqual(result["exit_code"], -signal.SIGKILL)
            self.assertLess(result["wall_seconds"], 5)

    def test_interruption_reaps_the_child(self):
        import subprocess

        processes = []
        popen = subprocess.Popen

        def capture(*args, **kwargs):
            process = popen(*args, **kwargs)
            processes.append(process)
            return process

        with tempfile.TemporaryDirectory() as directory, \
                patch("scripts.huge_function_bench.subprocess.Popen", side_effect=capture), \
                patch("scripts.huge_function_bench.time.sleep", side_effect=KeyboardInterrupt):
            with self.assertRaises(KeyboardInterrupt):
                measure([sys.executable, "-c", "import time; time.sleep(60)"],
                        Path(directory) / "fixture", 5)
        self.assertEqual(processes[0].returncode, -signal.SIGKILL)
