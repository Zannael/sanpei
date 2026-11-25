#!/usr/bin/env python3
"""Check project text and filenames for common accidental disclosures."""
from pathlib import Path
import re
import subprocess
import sys

root = Path(__file__).resolve().parent.parent
try:
    names = subprocess.check_output(['git', 'ls-files', '-co', '--exclude-standard', '-z'], cwd=root, stderr=subprocess.DEVNULL).decode().split('\0')
except (subprocess.CalledProcessError, FileNotFoundError):
    names = [str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and not any(part in {'.git', 'node_modules', 'target', 'dist', '.agents', '.codex', '.aws'} for part in p.parts)]
patterns = [re.compile(r'/home/' + r'[^/\s]+/'), re.compile(r'/Users/' + r'[^/\s]+/'), re.compile(r'\bgh' + r'p_[A-Za-z0-9]{20,}'), re.compile(r'\bgithub_' + r'pat_[A-Za-z0-9_]{20,}'), re.compile(r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b')]
failures = []
for name in filter(None, names):
    path = root / name
    if not path.is_file() or path.is_symlink():
        continue
    if path.suffix.lower() in {'.sqlite', '.sqlite3', '.db', '.pdf', '.jpg', '.jpeg', '.png'} and name != 'src-tauri/icons/icon.png':
        failures.append(f'{name}: file type requires review')
        continue
    try:
        content = path.read_text(encoding='utf-8')
    except (UnicodeDecodeError, OSError):
        continue
    for i, line in enumerate(content.splitlines(), 1):
        if any(p.search(line) for p in patterns):
            failures.append(f'{name}:{i}: possible personal data or credential')
if failures:
    print('\n'.join(failures))
    sys.exit(1)
print('Privacy check: no common indicators found; manual review is still required.')
