/** @param {{starts_at:string, ends_at:string, pause_minutes:number|null, ineffective_minutes:number|null}[]} intervals */
export function summarizeEffort(intervals) {
  let nominalMinutes = 0;
  let effectiveMinutes = 0;
  let complete = intervals.length > 0;
  for (const interval of intervals) {
    const duration = (Date.parse(interval.ends_at) - Date.parse(interval.starts_at)) / 60000;
    nominalMinutes += duration;
    if (interval.pause_minutes === null || interval.ineffective_minutes === null) {
      complete = false;
    } else {
      effectiveMinutes += duration - interval.pause_minutes - interval.ineffective_minutes;
    }
  }
  return { nominalMinutes, effectiveMinutes: complete ? effectiveMinutes : null };
}
