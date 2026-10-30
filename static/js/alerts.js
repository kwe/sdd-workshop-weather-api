import { el, clear } from './dom.js';
import { normalize, normalizeCountry } from './gazetteer.js';
import { formatNumber, formatTemp, sanitize } from './weather.js';

const NEUTRAL = { rank: 0, label: 'Alert', color: '#94a3b8', rgb: '148,163,184' };

export const SEVERITIES = {
  severe: { rank: 3, label: 'Severe', color: '#ff2d2d', rgb: '255,45,45' },
  warning: { rank: 2, label: 'Warning', color: '#ff8c00', rgb: '255,140,0' },
  watch: { rank: 1, label: 'Watch', color: '#ffd400', rgb: '255,212,0' },
};

const TYPES = {
  heat: { label: 'Heat', kind: 'temp' },
  cold: { label: 'Cold', kind: 'temp' },
  wind: { label: 'High wind', kind: 'wind' },
  humidity: { label: 'High humidity', kind: 'humidity' },
};

const SOURCES = { current: 'now', forecast: 'forecast' };

export function severityInfo(severity) {
  return Object.hasOwn(SEVERITIES, severity) ? SEVERITIES[severity] : NEUTRAL;
}

function typeInfo(type) {
  return Object.hasOwn(TYPES, type) ? TYPES[type] : { label: 'Weather', kind: 'raw' };
}

/** Same key shape as buildCities() in main.js, so alerts line up with globe pins. */
export function cityKeyOf(alert) {
  return `${normalize(sanitize(alert?.city, 60))}|${normalizeCountry(alert?.country)}`;
}

export function groupByCity(alerts) {
  const groups = new Map();
  for (const alert of Array.isArray(alerts) ? alerts : []) {
    if (!alert || typeof alert.city !== 'string') continue;
    const key = cityKeyOf(alert);
    let group = groups.get(key);
    if (!group) {
      group = { key, worst: alert.severity, alerts: [] };
      groups.set(key, group);
    }
    group.alerts.push(alert);
    if (severityInfo(alert.severity).rank > severityInfo(group.worst).rank) group.worst = alert.severity;
  }
  return groups;
}

export function countBySeverity(alerts) {
  const counts = { severe: 0, warning: 0, watch: 0 };
  for (const alert of Array.isArray(alerts) ? alerts : []) {
    if (Object.hasOwn(counts, alert?.severity)) counts[alert.severity] += 1;
  }
  return counts;
}

/** Ring data for the globe: one entry per alerting city, coloured by its worst severity. */
export function ringList(groups) {
  return [...groups.values()].map((group) => {
    const info = severityInfo(group.worst);
    return { key: group.key, severity: group.worst, color: info.color, rgb: info.rgb };
  });
}

function formatMeasure(kind, value, unit) {
  if (!Number.isFinite(value)) return '—';
  if (kind === 'temp') return formatTemp(value, unit, 1);
  if (kind === 'wind') return formatNumber(value, 1, ' km/h');
  if (kind === 'humidity') return formatNumber(value, 0, '%');
  return formatNumber(value, 1, '');
}

function formatDate(isoDate) {
  return /^\d{4}-\d{2}-\d{2}$/.test(String(isoDate)) ? isoDate : '';
}

export function describeAlert(alert, unit) {
  const type = typeInfo(alert.type);
  const severity = severityInfo(alert.severity);
  const value = formatMeasure(type.kind, alert.value, unit);
  const threshold = formatMeasure(type.kind, alert.threshold, unit);
  const source = Object.hasOwn(SOURCES, alert.source) ? SOURCES[alert.source] : '';
  return {
    type: type.label,
    severity: severity.label,
    color: severity.color,
    text: `${value} (threshold ${threshold})`,
    when: [source, formatDate(alert.date)].filter(Boolean).join(', '),
  };
}

function severityChip(severity) {
  const info = severityInfo(severity);
  const chip = el('span', { class: 'alert-chip', vars: { '--alert-color': info.color } }, info.label);
  return chip;
}

let expanded = false;
let bannerSig = '';
let lastState = null;
let lastSelect = () => {};

/** Compact alerts banner: count pill that expands into a clickable list of alerts. */
export function renderAlertsBanner(root, state, onSelect) {
  lastState = state;
  lastSelect = onSelect;
  draw(root);
}

function draw(root) {
  const { alerts, alertsStatus, unit } = lastState;
  const sig = JSON.stringify([alertsStatus, alerts, unit, expanded]);
  if (sig === bannerSig) return;
  bannerSig = sig;

  const hadFocus = root.contains(document.activeElement);
  clear(root);
  root.classList.toggle('is-error', alertsStatus === 'error');
  if (alertsStatus === 'loading') {
    root.hidden = true;
    return;
  }
  root.hidden = false;
  const list = (Array.isArray(alerts) ? alerts : []).filter((a) => a && typeof a.city === 'string');

  if (alertsStatus === 'error') {
    root.append(el('p', { class: 'alerts-note' }, 'Alerts unavailable'));
    return;
  }
  if (!list.length) {
    root.append(el('p', { class: 'alerts-note' }, 'No active alerts'));
    return;
  }

  const counts = countBySeverity(list);
  const summary = ['severe', 'warning', 'watch']
    .filter((name) => counts[name])
    .map((name) => `${counts[name]} ${SEVERITIES[name].label.toLowerCase()}`)
    .join(', ');
  const toggle = el(
    'button',
    {
      type: 'button',
      class: 'alerts-toggle',
      'aria-expanded': String(expanded),
      'aria-controls': 'alerts-list',
      on: {
        click: () => {
          expanded = !expanded;
          draw(root);
        },
      },
    },
    el('span', { class: 'alerts-dot', 'aria-hidden': 'true' }),
    el('b', null, `${list.length} ${list.length === 1 ? 'alert' : 'alerts'}`),
    el('span', { class: 'alerts-summary' }, summary),
  );
  const items = el('ul', { class: 'alerts-list', id: 'alerts-list' });
  items.hidden = !expanded;
  for (const alert of list) {
    const info = describeAlert(alert, unit);
    const key = cityKeyOf(alert);
    items.append(
      el(
        'li',
        null,
        el(
          'button',
          {
            type: 'button',
            class: 'alert-item',
            on: {
              click: () => {
                lastSelect(key);
                expanded = false;
                draw(root);
              },
            },
          },
          severityChip(alert.severity),
          el(
            'span',
            { class: 'alert-text' },
            el('span', { class: 'alert-city' }, sanitize(alert.city)),
            el('span', { class: 'alert-meta' }, `${info.type}: ${info.text}`),
          ),
        ),
      ),
    );
  }
  root.append(toggle, items);
  if (hadFocus) toggle.focus();
}

export function collapseBanner(root) {
  if (!expanded || !lastState) return false;
  expanded = false;
  draw(root);
  return true;
}

/** Detail-panel list of one city's alerts. */
export function renderCityAlerts(root, alerts, unit) {
  clear(root);
  const list = Array.isArray(alerts) ? alerts : [];
  root.hidden = !list.length;
  if (!list.length) return;
  root.append(el('h3', null, 'Active alerts'));
  const rows = el('ul', { class: 'city-alert-list' });
  for (const alert of list) {
    const info = describeAlert(alert, unit);
    rows.append(
      el(
        'li',
        { class: 'city-alert' },
        severityChip(alert.severity),
        el(
          'span',
          { class: 'alert-text' },
          el('span', { class: 'alert-city' }, `${info.type}: ${info.text}`),
          el('span', { class: 'alert-meta' }, info.when),
        ),
      ),
    );
  }
  root.append(rows);
}
