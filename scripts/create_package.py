#!/usr/bin/env python3
"""Emit a draft package template; review and version it before importing."""
import json
from pathlib import Path
root = Path(__file__).resolve().parent.parent
read = lambda name: json.loads((root / 'content' / name).read_text(encoding='utf-8'))
package = {
    'schema_version': 1,
    'package_id': 'lazio-nuova-ricerca',
    'content_version': '0.1.0',
    'region': 'Lazio',
    'review_status': 'draft',
    'rules': read('rules.v1.json'),
    'rigs': read('lazio-rigs.v1.json'),
    'sectors': read('lazio-sectors.v1.json'),
}
print(json.dumps(package, ensure_ascii=False, indent=2))
