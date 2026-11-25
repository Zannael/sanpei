import { test } from 'node:test';
import assert from 'node:assert/strict';
import { summarizeEffort } from '../src/effort.js';

test('sums concurrent rods and elapsed UTC time across daylight saving changes', () => {
  const intervals = [
    { starts_at: '2026-10-25T00:30:00Z', ends_at: '2026-10-25T02:30:00Z', pause_minutes: 15, ineffective_minutes: 5 },
    { starts_at: '2026-10-25T00:30:00Z', ends_at: '2026-10-25T01:30:00Z', pause_minutes: 0, ineffective_minutes: 10 },
  ];
  assert.deepEqual(summarizeEffort(intervals), { nominalMinutes: 180, effectiveMinutes: 150 });
  assert.deepEqual(summarizeEffort([...intervals, {...intervals[0], pause_minutes: null}]), { nominalMinutes: 300, effectiveMinutes: null });
  assert.deepEqual(summarizeEffort([]), { nominalMinutes: 0, effectiveMinutes: null });
});
