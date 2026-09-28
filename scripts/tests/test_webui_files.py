from contextlib import ExitStack
from pathlib import Path
import tempfile
import unittest
from unittest import mock

from scripts.repipe import webui


class DashboardFileTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-webui-files-")
        self.addCleanup(temporary.cleanup)
        patches = ExitStack()
        self.addCleanup(patches.close)
        self.root = Path(temporary.name)
        self.state = self.root / "state"
        self.needs = self.root / "needs"
        self.rejected = self.needs / "rejected"
        self.site = self.root / "site"
        self.ui = self.root / "ui"
        for path in [self.state, self.needs, self.rejected, self.site, self.ui]:
            path.mkdir(parents=True, exist_ok=True)
        for function, value in [("needs_dir", self.needs), ("rejected_dir", self.rejected)]:
            patches.enter_context(mock.patch.object(webui.rconfig, function, return_value=value))
        patches.enter_context(mock.patch.object(webui, "SITE_ASSETS", self.site))
        patches.enter_context(mock.patch.object(webui, "SITE_FONTS", self.site / "fonts"))
        patches.enter_context(mock.patch.object(webui, "WEBUI_DIR", self.ui))
        patches.enter_context(mock.patch.object(webui, "_cli_probe_index", return_value={}))
        patches.enter_context(mock.patch.object(webui, "verify_mod", None))

    def write(self, path, text):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        return path

    def handler(self):
        handler = object.__new__(webui.Handler)
        handler._file = mock.Mock(return_value="served")
        handler._err = mock.Mock(return_value="refused")
        return handler

    def test_identifiers_must_match_completely(self):
        for value in ["example", "example-1", "A_b.c", "x" * 64]:
            with self.subTest(value=value):
                self.assertEqual(webui.safe_id(value), value)
        for value in [None, 1, "", "example\n", "example\r", "x" * 65]:
            with self.subTest(value=value):
                self.assertIsNone(webui.safe_id(value))

    def test_invalid_identifiers_stop_before_directory_lookup(self):
        with mock.patch.object(webui, "_listed_child") as lookup:
            self.assertIsNone(webui.read_need("example\n"))
            self.assertIsNone(webui.read_probe(self.state, "p-0123456789ab\n"))
            self.assertIsNone(webui.read_agent_report(self.state, "tester\n", []))
        lookup.assert_not_called()

    def test_unavailable_directory_has_no_matching_child(self):
        self.assertIsNone(webui._listed_child(self.root / "missing", "example"))
        with mock.patch.object(webui.os, "scandir", side_effect=PermissionError("unreadable")):
            self.assertIsNone(webui._listed_child(self.root, "example"))

    def test_need_lookup_retains_rejected_and_linked_records(self):
        self.write(self.needs / "open-example.md", "---\nneed_id: open-example\nstatus: open\n---\n## Evidence\ntext\n")
        self.write(self.rejected / "rejected-example.md", "---\nneed_id: rejected-example\nstatus: open\n---\n")
        target = self.write(self.root / "linked-record.md", "---\nneed_id: linked-example\nstatus: open\n---\n")
        (self.needs / "linked-example.md").symlink_to(target)
        self.assertEqual(webui.read_need("open-example")["sections"], {"Evidence": "text"})
        self.assertEqual(webui.read_need("rejected-example")["status"], "rejected")
        self.assertEqual(webui.read_need("linked-example")["need_id"], "linked-example")
        self.assertIsNone(webui.read_need("missing-example"))

    def test_probe_lookup_keeps_source_precedence_and_replays(self):
        probe = "p-0123456789ab"
        primary = self.write(self.state / "probes" / f"{probe}.json", '{"kind": "primary"}')
        secondary = self.write(self.needs / "probes" / f"{probe}.json", '{"kind": "secondary"}')
        self.write(self.state / "replays" / f"{probe}.jsonl", '{"status": "PASS"}\n')
        result = webui.read_probe(self.state, probe)
        self.assertEqual(result["origin"], str(primary))
        self.assertEqual(result["kind"], "primary")
        self.assertEqual(result["replays"], [{"status": "PASS"}])
        primary.unlink()
        result = webui.read_probe(self.state, probe)
        self.assertEqual(result["origin"], str(secondary))
        self.assertEqual(result["kind"], "secondary")
        self.assertIsNone(webui.read_probe(self.state, "p-aaaaaaaaaaaa"))

    def test_report_lookup_keeps_recorded_path_precedence(self):
        recorded = self.write(self.state / "arena" / "report.json", '{"source": "arena"}')
        fallback = self.write(self.state / "reports" / "tester.json", '{"source": "fallback"}')
        agents = [{"id": "tester", "report": str(recorded)}]
        result = webui.read_agent_report(self.state, "tester", agents)
        self.assertEqual(result["path"], str(recorded))
        self.assertEqual(result["report"], {"source": "arena"})
        result = webui.read_agent_report(self.state, "tester", [])
        self.assertEqual(result["path"], str(fallback))
        self.assertIsNone(webui.read_agent_report(self.state, "missing", []))

    def test_static_assets_keep_nested_paths_and_font_routes(self):
        nested = self.write(self.ui / "assets" / "css" / "theme.css", "body {}")
        font = self.write(self.site / "fonts" / "regular.woff2", "font")
        handler = self.handler()
        self.assertEqual(handler.serve_asset(["css", "theme.css"]), "served")
        handler._file.assert_called_with(nested)
        self.assertEqual(handler.serve_asset(["fonts", "regular.woff2"]), "served")
        handler._file.assert_called_with(font)
        self.assertEqual(handler.serve_site_file("fonts", "regular.woff2"), "served")
        handler._file.assert_called_with(font)
        self.assertEqual(handler.serve_asset(["missing.css"]), "refused")
        handler._err.assert_called_with(404, "not found")

    def test_asset_links_remain_confined_to_their_root(self):
        inside = self.write(self.ui / "assets" / "original.css", "body {}")
        outside = self.write(self.root / "unpublished.css", "body {}")
        (inside.parent / "alias.css").symlink_to(inside)
        (inside.parent / "unpublished.css").symlink_to(outside)
        handler = self.handler()
        self.assertEqual(handler.serve_asset(["alias.css"]), "served")
        handler._file.assert_called_once_with(inside.parent / "alias.css")
        handler._file.reset_mock()
        self.assertEqual(handler.serve_asset(["unpublished.css"]), "refused")
        handler._file.assert_not_called()


if __name__ == "__main__":
    unittest.main()
