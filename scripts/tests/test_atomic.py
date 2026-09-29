import json
import os
from pathlib import Path
import stat
import tempfile
import unittest
from unittest import mock

from scripts.atomic import atomic_text_writer
from scripts.repipe.webui import Cache


class AtomicWriterTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-atomic-test-")
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.path = self.directory / "record.json"
        self.path.write_text("previous")

    def assert_no_scratch(self):
        self.assertEqual(list(self.directory.iterdir()), [self.path])

    def test_publishes_only_after_the_writer_closes(self):
        with atomic_text_writer(self.path, encoding="utf-8") as stream:
            stream.write("complete 🚀")
            stream.flush()
            self.assertEqual(self.path.read_text(), "previous")
        self.assertTrue(stream.closed)
        self.assertEqual(self.path.read_text(encoding="utf-8"), "complete 🚀")
        self.assert_no_scratch()

    def test_interleaved_writers_never_share_a_scratch_file(self):
        with atomic_text_writer(self.path) as first:
            first.write("first")
            with atomic_text_writer(self.path) as second:
                second.write("second")
            self.assertEqual(self.path.read_text(), "second")
            first.write(" complete")
        self.assertEqual(self.path.read_text(), "first complete")
        self.assert_no_scratch()

    def test_long_destination_name_does_not_extend_the_scratch_name(self):
        destination = self.directory / ("x" * 220)
        with atomic_text_writer(destination) as stream:
            stream.write("complete")
        self.assertEqual(destination.read_text(), "complete")
        self.assertEqual(set(self.directory.iterdir()), {self.path, destination})

    def test_serialization_failure_keeps_the_old_file(self):
        with self.assertRaises(TypeError):
            with atomic_text_writer(self.path) as stream:
                json.dump({"valid": 1, "invalid": object()}, stream)
        self.assertEqual(self.path.read_text(), "previous")
        self.assert_no_scratch()

    def test_replace_failure_keeps_the_old_file(self):
        with mock.patch("scripts.atomic.os.replace", side_effect=PermissionError("denied")):
            with self.assertRaises(PermissionError):
                with atomic_text_writer(self.path) as stream:
                    stream.write("complete")
        self.assertEqual(self.path.read_text(), "previous")
        self.assert_no_scratch()

    def test_existing_temporary_file_is_neither_overwritten_nor_removed(self):
        collision = self.directory / ".kuna-fixed.tmp"
        collision.write_text("belongs to someone else")
        with mock.patch("scripts.atomic.uuid.uuid4", return_value=mock.Mock(hex="fixed")):
            with self.assertRaises(FileExistsError):
                with atomic_text_writer(self.path):
                    self.fail("opened someone else's file")
        self.assertEqual(collision.read_text(), "belongs to someone else")
        self.assertEqual(self.path.read_text(), "previous")

    @unittest.skipUnless(os.name == "posix", "POSIX permission bits")
    def test_default_permissions_match_an_ordinary_new_file(self):
        ordinary = self.directory / "ordinary"
        ordinary.write_text("value")
        with atomic_text_writer(self.path) as stream:
            stream.write("value")
        self.assertEqual(stat.S_IMODE(self.path.stat().st_mode),
                         stat.S_IMODE(ordinary.stat().st_mode))

    @unittest.skipUnless(os.name == "posix", "POSIX permission bits")
    def test_explicit_mode_is_applied_before_publication(self):
        replace = os.replace

        def checked_replace(source, destination):
            self.assertEqual(stat.S_IMODE(source.stat().st_mode), 0o755)
            replace(source, destination)

        with mock.patch("scripts.atomic.os.replace", side_effect=checked_replace):
            with atomic_text_writer(self.path, mode=0o755) as stream:
                stream.write("executable")
        self.assertEqual(stat.S_IMODE(self.path.stat().st_mode), 0o755)
        self.assert_no_scratch()

    def test_missing_parent_does_not_leave_an_unowned_file(self):
        with self.assertRaises(FileNotFoundError):
            with atomic_text_writer(self.directory / "missing" / "record.json"):
                self.fail("opened under a missing directory")
        self.assert_no_scratch()

    def test_cache_retains_its_serialization_and_best_effort_failure_policy(self):
        cache = Cache(self.directory)
        cache_path = self.directory / "webui-cache.json"
        cache._write_through({"path": Path("example")})
        previous = '{"path": "example"}'
        self.assertEqual(cache_path.read_text(), previous)
        with mock.patch("scripts.atomic.os.replace", side_effect=PermissionError("denied")):
            cache._write_through({"new": True})
        self.assertEqual(cache_path.read_text(), previous)
        self.assertEqual(set(self.directory.iterdir()), {self.path, cache_path})


if __name__ == "__main__":
    unittest.main()
