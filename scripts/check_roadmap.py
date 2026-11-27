#!/usr/bin/env python3
"""Validate Sanpei's only backlog and derive its deterministic dependency graph."""
from __future__ import annotations

import argparse
from dataclasses import asdict, dataclass
from datetime import datetime
import json
import os
from pathlib import Path
import re
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent
ID = r"[A-Z][A-Z0-9]*-[0-9]{3,}"
TASK = re.compile(rf"- \[([ x])\] \*\*(Not started|In progress|Done)\*\* \*\*({ID})\*\* (\S.*)")
RETIRED = re.compile(rf"- \*\*({ID})\*\* Retired\. Reason: (\S.*?)\. Replacement: (none|`{ID}`)\.")
REQUIRED = {"Depends on", "Responsible", "Owner", "Scope", "Completion"}
ALLOWED = REQUIRED | {"Note", "Verification"}


class RoadmapError(ValueError):
    pass


@dataclass
class Task:
    id: str
    title: str
    status: str
    dependencies: list[str]
    responsible: str
    owner: str
    scope: str
    completion: str
    note: str | None = None
    verification: str | None = None


def visible_lines(text: str) -> list[tuple[int, str]]:
    """Ignore fenced examples, rejecting unmatched opening fences."""
    result = []
    fence = None
    for number, line in enumerate(text.splitlines(), 1):
        match = re.match(r"^\s{0,3}(`{3,}|~{3,})(.*)$", line)
        if fence:
            if match and match[1][0] == fence[0] and len(match[1]) >= len(fence) and not match[2].strip():
                fence = None
            continue
        if match:
            fence = match[1]
        else:
            result.append((number, line))
    if fence:
        raise RoadmapError("Unclosed code fence")
    return result


def parse(text: str) -> tuple[list[Task], list[dict[str, str | None]]]:
    lines = visible_lines(text)
    tasks: list[Task] = []
    retired: list[dict[str, str | None]] = []
    seen: set[str] = set()
    current = None
    fields: dict[str, str] = {}
    retired_section = False
    has_retired_section = False

    def finish():
        nonlocal current, fields
        if current is None:
            return
        checkbox, status, task_id, title = current
        missing = REQUIRED - fields.keys()
        if missing:
            raise RoadmapError(f"{task_id}: missing fields: {', '.join(sorted(missing))}")
        if (checkbox == "x") != (status == "Done"):
            raise RoadmapError(f"{task_id}: checkbox/status mismatch")
        if fields["Responsible"] not in {"Agent", "Owner"}:
            raise RoadmapError(f"{task_id}: Responsible must be Agent or Owner")
        owner = fields["Owner"]
        if status != "Not started" and owner == "unassigned":
            raise RoadmapError(f"{task_id}: claimed Owner required")
        if status == "In progress":
            if not re.fullmatch(r"\(owner: " + re.escape(owner) + r", \S.*\)", fields.get("Note", "")):
                raise RoadmapError(f"{task_id}: In progress requires Note matching Owner")
        verification = fields.get("Verification")
        if status == "Done":
            match = re.fullmatch(r"(\d{4}-\d{2}-\d{2}(?: \d{2}:\d{2}(?::\d{2})?)?) UTC (\S.*)", verification or "")
            if not match:
                raise RoadmapError(f"{task_id}: Done requires dated UTC Verification and evidence")
            try:
                datetime.fromisoformat(match[1])
            except ValueError as exc:
                raise RoadmapError(f"{task_id}: invalid Verification date") from exc
        elif verification is not None:
            raise RoadmapError(f"{task_id}: Verification forbidden unless Done")
        deps = fields["Depends on"]
        if deps == "none.":
            dependencies = []
        elif re.fullmatch(rf"`{ID}`(?:, `{ID}`)*\.", deps):
            dependencies = re.findall(ID, deps)
            if len(dependencies) != len(set(dependencies)):
                raise RoadmapError(f"{task_id}: duplicate dependency")
        else:
            raise RoadmapError(f"{task_id}: invalid Depends on format")
        tasks.append(Task(task_id, title, status, dependencies, fields["Responsible"], owner,
                          fields["Scope"], fields["Completion"], fields.get("Note"), verification))
        current = None
        fields = {}

    for number, line in lines:
        if not line.strip():
            finish()
            continue
        if current:
            field = re.fullmatch(r"  ([A-Za-z ]+): (\S.*)", line)
            if not field:
                raise RoadmapError(f"Line {number}: task fields must be single lines; blank line ends task")
            name, value = field.groups()
            if name not in ALLOWED:
                raise RoadmapError(f"Line {number}: unsupported field {name}")
            if name in fields:
                raise RoadmapError(f"Line {number}: duplicate field {name}")
            fields[name] = value
            continue
        if line.startswith("## "):
            retired_section = line == "## Retired IDs"
            has_retired_section |= retired_section
        match = TASK.fullmatch(line)
        old = RETIRED.fullmatch(line)
        if match or old:
            task_id = match[3] if match else old[1]
            if task_id in seen:
                raise RoadmapError(f"Duplicate ID: {task_id}")
            seen.add(task_id)
            if match:
                if retired_section:
                    raise RoadmapError(f"{task_id}: active task inside Retired IDs")
                current = match.groups()
            else:
                if not retired_section:
                    raise RoadmapError(f"{task_id}: retired ID outside Retired IDs")
                retired.append({"id": task_id, "reason": old[2],
                                "replacement": None if old[3] == "none" else old[3][1:-1]})
        elif re.match(r"\s*- \[", line) or re.match(r"\s*- \*\*[A-Z][A-Z0-9]*-", line):
            raise RoadmapError(f"Line {number}: malformed task or retired ID")
    finish()
    if not tasks or not has_retired_section:
        raise RoadmapError("At least one task and a Retired IDs section are required")
    active = {task.id: task for task in tasks}
    old_ids = {item["id"] for item in retired}
    for task in tasks:
        for dep in task.dependencies:
            if dep in old_ids:
                raise RoadmapError(f"{task.id}: dependency on retired ID {dep}")
            if dep not in active:
                raise RoadmapError(f"{task.id}: unknown dependency {dep}")
    for item in retired:
        if item["replacement"] is not None and item["replacement"] not in active:
            raise RoadmapError(f"{item['id']}: replacement must be an active ID")
    for number, line in lines:
        for ref in re.findall(rf"`({ID})`", line):
            if ref not in seen:
                raise RoadmapError(f"Line {number}: unknown ID reference {ref}")
    # DFS uses the active stack to report exactly the cycle, excluding its ancestors.
    visited: set[str] = set()
    stack: list[str] = []

    def visit(task_id: str):
        if task_id in stack:
            start = stack.index(task_id)
            raise RoadmapError("Dependency cycle: " + " -> ".join(stack[start:] + [task_id]))
        if task_id in visited:
            return
        stack.append(task_id)
        for dep in active[task_id].dependencies:
            visit(dep)
        stack.pop()
        visited.add(task_id)

    for task in tasks:
        visit(task.id)
    for task in tasks:
        if task.status != "Not started" and any(active[dep].status != "Done" for dep in task.dependencies):
            raise RoadmapError(f"{task.id}: claimed or Done task has unfinished dependencies")
    return tasks, retired


def graph_bytes(tasks: list[Task], retired: list[dict[str, str | None]]) -> bytes:
    payload = {"schema_version": 1, "tasks": [asdict(task) for task in tasks], "retired": retired}
    return (json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2) + "\n").encode("utf-8")


def ready(tasks: list[Task]) -> list[Task]:
    done = {task.id for task in tasks if task.status == "Done"}
    return [task for task in tasks if task.status == "Not started" and task.owner == "unassigned"
            and set(task.dependencies) <= done]


def layers(tasks: list[Task]) -> list[list[str]]:
    done = {task.id for task in tasks if task.status == "Done"}
    remaining = [task for task in tasks if task.id not in done]
    waves = []
    while remaining:
        wave = [task.id for task in remaining if set(task.dependencies) <= done]
        if not wave:
            raise RoadmapError("Cannot layer cyclic or unknown dependencies")
        waves.append(wave)
        done.update(wave)
        remaining = [task for task in remaining if task.id not in done]
    return waves


def atomic_write(path: Path, data: bytes):
    """Publish in the same directory, never leaving a partially written graph."""
    with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".task-graph-", delete=False) as stream:
        temporary = Path(stream.name)
        try:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        except BaseException:
            temporary.unlink(missing_ok=True)
            raise
    try:
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)


def main(argv=None, root: Path = ROOT) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write-graph", action="store_true")
    mode.add_argument("--ready", action="store_true")
    mode.add_argument("--layers", action="store_true")
    args = parser.parse_args(argv)
    try:
        tasks, retired = parse((root / "roadmap/TODO.md").read_text(encoding="utf-8"))
        path = root / "roadmap/task-graph.json"
        expected = graph_bytes(tasks, retired)
        if args.write_graph:
            atomic_write(path, expected)
        elif not path.exists() or path.read_bytes() != expected:
            raise RoadmapError("task-graph.json missing or stale; run --write-graph")
        if args.ready:
            for task in ready(tasks):
                print(f"{task.id} {task.title}")
        elif args.layers:
            for index, wave in enumerate(layers(tasks), 1):
                print(f"{index}: {', '.join(wave)}")
        else:
            print(f"Roadmap OK: {len(tasks)} tasks, {len(retired)} retired IDs; graph up to date.")
        return 0
    except (OSError, UnicodeError, RoadmapError) as exc:
        print(f"Roadmap error: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
