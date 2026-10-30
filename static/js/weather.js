const CONTROL = /[\u0000-\u001f\u007f-\u009f]/g;

export function sanitize(value, max = 60) {
  if (value === null || value === undefined) return '';
  const text = String(value).replace(CONTROL, ' ').replace(/\s+/g, ' ').trim();
  return text.length > max ? `${text.slice(0, max - 1)}…` : text;
}

export const NO_DATA_COLOR = '#94a3b8';

export const TEMP_STOPS = [
  [-20, '#7c5cff'],
  [-10, '#3b82f6'],
  [0, '#22d3ee'],
  [10, '#34d399'],
  [20, '#facc15'],
  [30, '#fb923c'],
  [40, '#ef4444'],
];

function hexToRgb(hex) {
  const n = parseInt(hex.slice(1), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

export function temperatureColor(celsius) {
  if (!Number.isFinite(celsius)) return NO_DATA_COLOR;
  const first = TEMP_STOPS[0];
  const last = TEMP_STOPS[TEMP_STOPS.length - 1];
  if (celsius <= first[0]) return first[1];
  if (celsius >= last[0]) return last[1];
  for (let i = 0; i < TEMP_STOPS.length - 1; i++) {
    const [t0, c0] = TEMP_STOPS[i];
    const [t1, c1] = TEMP_STOPS[i + 1];
    if (celsius >= t0 && celsius <= t1) {
      const f = (celsius - t0) / (t1 - t0);
      const a = hexToRgb(c0);
      const b = hexToRgb(c1);
      const rgb = a.map((v, k) => Math.round(v + (b[k] - v) * f));
      return `#${rgb.map((v) => v.toString(16).padStart(2, '0')).join('')}`;
    }
  }
  return NO_DATA_COLOR;
}

const RULES = [
  [/storm|thunder/, 'storm'],
  [/snow|sleet|blizzard|flurr/, 'snow'],
  [/rain|drizzle|shower/, 'rain'],
  [/fog|mist|haze|smog/, 'fog'],
  [/partly|scattered|mostly sunny/, 'partly'],
  [/overcast|cloud/, 'cloud'],
  [/wind|breez|gust/, 'wind'],
  [/humid|muggy/, 'humid'],
];

export function conditionKind(text, isDay = true) {
  if (!text) return 'unknown';
  const t = String(text).toLowerCase();
  for (const [re, kind] of RULES) if (re.test(t)) return kind;
  if (/clear/.test(t)) return isDay ? 'sun' : 'moon';
  if (/sun|warm|hot|fair/.test(t)) return 'sun';
  return 'unknown';
}

export function toFahrenheit(c) {
  return (c * 9) / 5 + 32;
}

export function formatTemp(celsius, unit, digits = 0) {
  if (!Number.isFinite(celsius)) return '—';
  const v = unit === 'F' ? toFahrenheit(celsius) : celsius;
  let text = v.toFixed(digits);
  if (/^-0(\.0+)?$/.test(text)) text = text.slice(1);
  return `${text}°`;
}

export function formatNumber(value, digits, suffix) {
  return Number.isFinite(value) ? `${value.toFixed(digits)}${suffix}` : '—';
}

const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export function formatRecordedAt(value) {
  const m = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2})/.exec(String(value ?? ''));
  if (!m) return sanitize(value, 40);
  return `${Number(m[3])} ${MONTHS[Number(m[2]) - 1] ?? ''} ${m[1]}, ${m[4]}:${m[5]}`;
}

const DAY_FMT = new Intl.DateTimeFormat('en-GB', {
  weekday: 'short',
  day: 'numeric',
  month: 'short',
  timeZone: 'UTC',
});

export function formatForecastDate(isoDate) {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(String(isoDate ?? ''));
  if (!m) return { weekday: sanitize(isoDate, 12) || '—', rest: '' };
  const d = new Date(Date.UTC(Number(m[1]), Number(m[2]) - 1, Number(m[3])));
  const parts = DAY_FMT.formatToParts(d);
  const pick = (type) => parts.find((p) => p.type === type)?.value ?? '';
  const todayIso = new Date().toISOString().slice(0, 10);
  return {
    weekday: isoDate === todayIso ? 'Today' : pick('weekday'),
    rest: `${pick('day')} ${pick('month')}`,
  };
}

export function clamp(v, lo, hi) {
  return Math.min(hi, Math.max(lo, v));
}
