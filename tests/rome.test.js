import { test } from 'node:test';
import assert from 'node:assert/strict';
import { romeToUtc } from '../src/rome.js';
test('converts Rome time independently of system timezone', () => {
  assert.equal(romeToUtc('2026-01-15T12:00'), '2026-01-15T11:00:00Z');
  assert.equal(romeToUtc('2026-07-15T12:00'), '2026-07-15T10:00:00Z');
});
test('rejects nonexistent, ambiguous and invalid civil times', () => {
  assert.throws(() => romeToUtc('2026-03-29T02:30'));
  assert.throws(() => romeToUtc('2026-10-25T02:30'));
  assert.throws(() => romeToUtc('2026-02-30T12:00'));
});
