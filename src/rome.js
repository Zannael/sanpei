/** Convert a local wall time in Europe/Rome into a UTC instant. Ambiguous
 * and nonexistent civil times are rejected instead of silently guessing.
 * @param {string} value
 * @returns {string}
 */
export function romeToUtc(value) {
  if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/.test(value)) throw new Error('Inserisci una data e un’ora valide.');
  const [day, clock] = value.split('T');
  const [year, month, date] = day.split('-').map(Number);
  const [hour, minute] = clock.split(':').map(Number);
  if (year < 100 || month < 1 || month > 12 || date < 1 || date > 31 || hour > 23 || minute > 59) throw new Error('Inserisci una data e un’ora valide.');
  const reference = Date.UTC(year, month - 1, date, hour, minute);
  const format = new Intl.DateTimeFormat('sv-SE', {timeZone:'Europe/Rome', year:'numeric',month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit',hourCycle:'h23'});
  const candidates = [1, 2].map(offset => new Date(reference - offset * 3600000)).filter(candidate => format.format(candidate).replace(' ', 'T') === value);
  if (candidates.length !== 1) throw new Error('L’ora scelta è inesistente o ambigua per il cambio dell’ora a Roma. Scegli un orario diverso.');
  return candidates[0].toISOString().replace('.000Z', 'Z');
}
