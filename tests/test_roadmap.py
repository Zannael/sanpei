"""Behavioral tests for the roadmap's format, state transitions and generated files."""
from contextlib import redirect_stderr, redirect_stdout
import io
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from scripts.check_roadmap import RoadmapError, graph_bytes, layers, main, parse, ready


def task(task_id="CORE-001", status="Not started", depends="none.", owner="unassigned"):
    checkbox = "x" if status == "Done" else " "
    text = f"""- [{checkbox}] **{status}** **{task_id}** Concrete activity.
  Depends on: {depends}
  Responsible: Agent
  Owner: {owner}
  Scope: Change one bounded behavior.
  Completion: Demonstrate the behavior with evidence.
"""
    if status == "In progress":
        text += f"  Note: (owner: {owner}, verifying the behavior)\n"
    if status == "Done":
        text += "  Verification: 2026-11-27 UTC checks executed; fixture only.\n"
    return text


def backlog(*tasks, retired=""):
    return "# Fixture backlog\n\n" + "\n\n".join(text.rstrip() for text in tasks) + "\n\n## Retired IDs\n\n" + retired


class ParsingTests(unittest.TestCase):
    def invalid(self, text, message):
        with self.assertRaisesRegex(RoadmapError, message):
            parse(text)

    def test_valid_all_statuses_and_retired(self):
        text = backlog(task(status="Done", owner="Codex"),
                       task("CORE-002", "In progress", "`CORE-001`.", "Codex"),
                       task("CORE-003", depends="`CORE-002`."),
                       retired="- **OLD-001** Retired. Reason: Replaced scope. Replacement: `CORE-001`.\n")
        tasks, retired = parse(text)
        self.assertEqual(tasks[1].dependencies, ["CORE-001"])
        self.assertEqual(tasks[1].owner, "Codex")
        self.assertEqual(retired[0]["replacement"], "CORE-001")
        self.assertEqual(layers(tasks), [["CORE-002"], ["CORE-003"]])

    def test_code_examples_are_ignored_in_both_fence_styles(self):
        examples = "```markdown\n" + task() + "\n```\n\n~~~text\n`UNKNOWN-001`\n~~~\n\n"
        tasks, _ = parse(examples + backlog(task()))
        self.assertEqual(len(tasks), 1)
        self.invalid(backlog(task()) + "\n```\nexample", "Unclosed")
        # A shorter fence and one with a suffix cannot close a longer fence.
        self.invalid("````\n```\n````suffix\n" + backlog(task()), "Unclosed")

    def test_checkbox_and_required_fields(self):
        self.invalid(backlog(task().replace("- [ ]", "- [x]")), "checkbox")
        self.invalid(backlog(task(status="Done", owner="Codex").replace("- [x]", "- [ ]")), "checkbox")
        for field in ["Depends on", "Responsible", "Owner", "Scope", "Completion"]:
            with self.subTest(field=field):
                text = "\n".join(line for line in task().splitlines() if not line.startswith(f"  {field}:"))
                self.invalid(backlog(text), "missing fields")

    def test_duplicates_unknown_fields_and_prose(self):
        for extra, message in [("  Scope: duplicate\n", "duplicate field"),
                               ("  Risk: unsupported\n", "unsupported field"),
                               ("  Some free-form prose\n", "single lines"),
                               ("  Scope: \n", "single lines")]:
            with self.subTest(extra=extra):
                self.invalid(backlog(task() + extra), message)
        self.invalid(backlog(task(), task()), "Duplicate ID")
        self.invalid(backlog(task(), retired="- **CORE-001** Retired. Reason: Old. Replacement: none.\n"), "Duplicate ID")
        self.invalid(backlog(task().replace("Responsible: Agent", "Responsible: Anyone")), "Responsible")
        self.invalid(backlog(task().replace("**CORE-001**", "**CORE-01**")), "malformed")
        self.invalid(backlog(task().replace("[ ]", "[X]")), "malformed")

    def test_in_progress_owner_and_note(self):
        self.invalid(backlog(task(status="In progress")), "claimed Owner")
        text = task(status="In progress", owner="Codex")
        self.invalid(backlog("\n".join(line for line in text.splitlines() if "Note:" not in line)), "requires Note")
        self.invalid(backlog(text.replace("(owner: Codex", "(owner: Other")), "matching Owner")
        self.invalid(backlog(text.replace("verifying the behavior", "")), "requires Note")

    def test_verification_state_and_dates(self):
        text = task(status="Done", owner="Codex")
        self.invalid(backlog(task(status="Done")), "claimed Owner")
        self.invalid(backlog("\n".join(line for line in text.splitlines() if "Verification:" not in line)), "Done requires")
        self.invalid(backlog(task() + "  Verification: 2026-11-27 UTC claimed.\n"), "forbidden")
        for invalid in ["2026-02-30 UTC invalid", "2026-11-27 25:00 UTC invalid", "2026-11-27 missing UTC", "2026-11-27 UTC "]:
            with self.subTest(invalid=invalid):
                self.invalid(backlog(text.replace("2026-11-27 UTC checks executed; fixture only.", invalid)), "Verification|single lines")
        for valid in ["2026-11-27", "2026-11-27 12:34", "2026-11-27 12:34:56"]:
            parse(backlog(text.replace("2026-11-27 UTC", valid + " UTC")))

    def test_dependency_and_reference_validation(self):
        self.invalid(backlog(task(depends="`UNKNOWN-001`.")), "unknown dependency")
        retired = "- **OLD-001** Retired. Reason: Removed. Replacement: none.\n"
        self.invalid(backlog(task(depends="`OLD-001`."), retired=retired), "retired ID")
        self.invalid(backlog(task()) + "\nSee `UNKNOWN-001`.\n", "unknown ID reference")
        self.invalid(backlog(task(), retired=retired.replace("none", "`UNKNOWN-001`")), "replacement")
        self.invalid(backlog(task(), retired="- **OLD-001** Retired. Reason: Removed. Replacement: `OLD-001`.\n"), "replacement")
        for deps in ["none", "CORE-002.", "`CORE-002`", "`CORE-002`, `CORE-002`."]:
            with self.subTest(deps=deps):
                self.invalid(backlog(task(depends=deps), task("CORE-002")), "Depends on|duplicate dependency")
        self.invalid(backlog(task(), task("CORE-002", "In progress", "`CORE-001`.", "Codex")), "unfinished dependencies")
        self.invalid(backlog(task(), task("CORE-002", "Done", "`CORE-001`.", "Codex")), "unfinished dependencies")

    def test_exact_cycle_and_self_cycle(self):
        text = backlog(task(depends="`CORE-002`."), task("CORE-002", depends="`CORE-003`."), task("CORE-003", depends="`CORE-002`."))
        with self.assertRaises(RoadmapError) as result:
            parse(text)
        self.assertEqual(str(result.exception), "Dependency cycle: CORE-002 -> CORE-003 -> CORE-002")
        self.invalid(backlog(task(depends="`CORE-001`.")), "CORE-001 -> CORE-001")

    def test_ready_and_layers_respect_claims_and_all_dependencies(self):
        tasks, _ = parse(backlog(task(status="Done", owner="Codex"),
                                task("CORE-002", depends="`CORE-001`."),
                                task("CORE-003", depends="`CORE-001`.", owner="Reserved"),
                                task("CORE-004", depends="`CORE-002`, `CORE-003`.")))
        self.assertEqual([item.id for item in ready(tasks)], ["CORE-002"])
        self.assertEqual(layers(tasks), [["CORE-002", "CORE-003"], ["CORE-004"]])
        self.assertEqual(ready(parse(backlog(task(status="Done", owner="Codex")))[0]), [])

    def test_format_boundaries(self):
        self.invalid(task(), "Retired IDs")
        self.invalid("## Retired IDs\n", "At least one")
        self.invalid(backlog(task().rstrip() + "\n" + task("CORE-002")), "single lines")
        self.invalid(backlog(task()) + "\n" + task("CORE-002"), "inside Retired")
        self.invalid("- **OLD-001** Retired. Reason: Old. Replacement: none.\n\n" + backlog(task()), "outside Retired")


class CommandTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / "roadmap").mkdir()
        (self.root / "scripts").mkdir()
        self.script = self.root / "scripts/check_roadmap.py"
        self.script.write_bytes((Path(__file__).resolve().parents[1] / "scripts/check_roadmap.py").read_bytes())
        self.todo = self.root / "roadmap/TODO.md"
        self.graph = self.root / "roadmap/task-graph.json"
        self.todo.write_text(backlog(task(status="Done", owner="Codex"), task("CORE-002", depends="`CORE-001`.")), encoding="utf-8")

    def cli(self, *args):
        return subprocess.run([sys.executable, str(self.script), *args], cwd=self.root,
                              capture_output=True, text=True, check=False)

    def test_cli_deterministic_write_validation_ready_and_layers(self):
        self.assertNotEqual(self.cli().returncode, 0)
        self.assertEqual(self.cli("--write-graph").returncode, 0)
        first = self.graph.read_bytes()
        self.assertEqual(self.cli("--write-graph").returncode, 0)
        self.assertEqual(self.graph.read_bytes(), first)
        tasks, retired = parse(self.todo.read_text(encoding="utf-8"))
        self.assertEqual(first, graph_bytes(tasks, retired))
        self.assertEqual(self.cli().returncode, 0)
        self.assertEqual(self.cli("--ready").stdout, "CORE-002 Concrete activity.\n")
        self.assertEqual(self.cli("--layers").stdout, "1: CORE-002\n")
        self.assertEqual(sorted(path.name for path in self.graph.parent.iterdir()), ["TODO.md", "task-graph.json"])

    def test_stale_graph_including_title_and_whitespace(self):
        self.cli("--write-graph")
        self.graph.write_bytes(self.graph.read_bytes() + b"\n")
        self.assertIn("stale", self.cli().stderr)
        self.assertNotEqual(self.cli("--ready").returncode, 0)
        self.cli("--write-graph")
        self.todo.write_text(self.todo.read_text(encoding="utf-8").replace("Concrete activity.", "Changed activity."), encoding="utf-8")
        self.assertNotEqual(self.cli().returncode, 0)

    def test_invalid_input_does_not_overwrite_graph(self):
        self.cli("--write-graph")
        original = self.graph.read_bytes()
        self.todo.write_text(backlog(task(depends="`UNKNOWN-001`.")), encoding="utf-8")
        self.assertNotEqual(self.cli("--write-graph").returncode, 0)
        self.assertEqual(self.graph.read_bytes(), original)

    def test_invalid_utf8_and_missing_backlog_are_reported(self):
        self.todo.write_bytes(b"\xff")
        with redirect_stderr(io.StringIO()), redirect_stdout(io.StringIO()):
            self.assertEqual(main([], root=self.root), 1)
            self.todo.unlink()
            self.assertEqual(main([], root=self.root), 1)


if __name__ == "__main__":
    unittest.main()
