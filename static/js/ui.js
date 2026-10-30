import { el, clear, icon } from './dom.js';
import { normalize } from './gazetteer.js';
import {
  TEMP_STOPS,
  clamp,
  conditionKind,
  formatForecastDate,
  formatNumber,
  formatRecordedAt,
  formatTemp,
  sanitize,
  temperatureColor,
} from './weather.js';
import { isDaylight } from './sun.js';

const $ = (id) => document.getElementById(id);

export function createUi({ getSun, actions }) {
  const nodes = {
    list: $('city-list'),
    search: $('search'),
    chip: $('status-chip'),
    detail: $('detail'),
    title: $('detail-title'),
    sub: $('detail-sub'),
    note: $('detail-note'),
    current: $('current'),
    forecast: $('forecast-body'),
    banner: $('banner'),
    live: $('live'),
    veil: $('veil'),
    legendBar: $('legend-bar'),
    legendLabels: $('legend-labels'),
  };
  let focusReturn = null;
  let listSig = '';
  let detailSig = '';
  let searchTimer = null;

  nodes.search.addEventListener('input', () => {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => {
      const cleaned = sanitize(nodes.search.value, 60);
      actions.onQuery(cleaned);
    }, 80);
  });
  $('detail-close').addEventListener('click', () => actions.onClose());
  $('reset-view').addEventListener('click', () => actions.onReset());
  $('list-toggle').addEventListener('click', () => {
    const open = document.body.classList.toggle('list-open');
    $('list-toggle').setAttribute('aria-expanded', String(open));
  });
  for (const button of document.querySelectorAll('.unit-toggle button')) {
    button.addEventListener('click', () => actions.onUnit(button.dataset.unit));
  }

  document.addEventListener('keydown', (e) => {
    const inInput = e.target === nodes.search;
    if (e.key === '/' && !inInput && !e.metaKey && !e.ctrlKey) {
      e.preventDefault();
      nodes.search.focus();
    } else if (e.key === 'Escape') {
      if (inInput && nodes.search.value) {
        nodes.search.value = '';
        actions.onQuery('');
      } else if (!nodes.detail.hidden) {
        actions.onClose();
        focusReturn?.focus?.();
      }
    } else if ((e.key === 'ArrowDown' || e.key === 'ArrowUp') && nodes.list.contains(document.activeElement)) {
      e.preventDefault();
      const items = [...nodes.list.querySelectorAll('.city-item')];
      const i = items.indexOf(document.activeElement);
      items[clamp(i + (e.key === 'ArrowDown' ? 1 : -1), 0, items.length - 1)]?.focus();
    } else if (e.key === 'ArrowDown' && inInput) {
      nodes.list.querySelector('.city-item')?.focus();
    }
  });

  function announce(text) {
    nodes.live.textContent = '';
    setTimeout(() => {
      nodes.live.textContent = text;
    }, 30);
  }

  function renderLegend(unit) {
    const stops = TEMP_STOPS.map(([t, c]) => `${c} ${((t + 20) / 60) * 100}%`).join(', ');
    nodes.legendBar.style.setProperty('background', `linear-gradient(90deg, ${stops})`);
    clear(nodes.legendLabels);
    for (const t of [-20, 0, 20, 40]) nodes.legendLabels.append(el('span', null, formatTemp(t, unit)));
  }

  function matches(city, query) {
    const q = normalize(query);
    return !q || normalize(city.name).includes(q) || normalize(city.country).includes(q);
  }

  function renderList(state) {
    const visible = state.cities.filter((c) => matches(c, state.query));
    const sig = JSON.stringify([
      state.query,
      state.unit,
      state.selectedKey,
      visible.map((c) => [c.key, c.record?.temperatureCelsius, c.record?.description, c.onGlobe]),
    ]);
    if (sig === listSig) return;
    listSig = sig;
    const focusedKey = document.activeElement?.dataset?.key;
    clear(nodes.list);
    if (!visible.length) {
      nodes.list.append(el('li', { class: 'empty' }, state.cities.length ? 'No cities match your search' : 'No weather records yet'));
      return;
    }
    const sun = getSun();
    for (const city of visible) {
      const day = city.onGlobe ? isDaylight(city.lat, city.lng, sun) : true;
      const kind = conditionKind(city.record?.description, day);
      const button = el(
        'button',
        {
          type: 'button',
          class: 'city-item',
          'data-key': city.key,
          'aria-current': city.key === state.selectedKey ? 'true' : null,
          on: {
            click: (e) => {
              focusReturn = e.currentTarget;
              actions.onSelect(city.key);
            },
          },
        },
        icon(kind),
        el(
          'span',
          null,
          el('span', { class: 'city-name' }, sanitize(city.name)),
          el(
            'span',
            { class: 'city-meta' },
            sanitize(city.country) || '—',
            city.onGlobe ? null : el('span', { class: 'tag' }, 'not on globe'),
          ),
        ),
        el('span', { class: 'city-temp' }, formatTemp(city.record?.temperatureCelsius, state.unit)),
      );
      button.querySelector('.city-temp').style.setProperty('color', temperatureColor(city.record?.temperatureCelsius));
      nodes.list.append(el('li', null, button));
    }
    if (focusedKey) nodes.list.querySelector(`[data-key="${CSS.escape(focusedKey)}"]`)?.focus();
  }

  function renderStatus(state) {
    const { phase, stale, lastOk } = state.status;
    const message = state.status.message || (state.globeFailed ? '3D globe unavailable - the city list still works' : '');
    nodes.chip.hidden = !stale;
    if (stale) nodes.chip.textContent = `Offline - showing data from ${lastOk ?? '--:--'}`;

    clear(nodes.banner);
    if (message) {
      nodes.banner.hidden = false;
      nodes.banner.append(el('span', null, message));
      if (phase === 'error') {
        nodes.banner.append(el('button', { type: 'button', class: 'retry', on: { click: () => actions.onRetryWeather() } }, 'Retry'));
      }
    } else {
      nodes.banner.hidden = true;
    }
    if (phase !== 'loading') nodes.veil.classList.add('done');
  }

  function statBlock(label, value, wide) {
    return el('div', { class: wide ? 'stat wide' : 'stat' }, label, el('b', null, value));
  }

  function renderForecast(entry, unit, city) {
    clear(nodes.forecast);
    if (!entry || entry.status === 'loading') {
      for (let i = 0; i < 5; i++) nodes.forecast.append(el('div', { class: 'skeleton' }));
      return;
    }
    if (entry.status === 'none') {
      nodes.forecast.append(el('p', { class: 'state-msg' }, 'No forecast available for this city'));
      return;
    }
    if (entry.status === 'error') {
      nodes.forecast.append(
        el('p', { class: 'state-msg' }, 'The forecast could not be loaded.'),
        el('button', { type: 'button', class: 'retry', on: { click: () => actions.onRetryForecast(city.key) } }, 'Retry'),
      );
      return;
    }
    const days = entry.data;
    const finite = (v) => Number.isFinite(v);
    const highs = days.map((d) => d.highTemperatureCelsius).filter(finite);
    const lows = days.map((d) => d.lowTemperatureCelsius).filter(finite);
    const min = Math.min(...lows, ...highs);
    const max = Math.max(...lows, ...highs);
    const span = max - min || 1;
    days.forEach((d, i) => {
      const when = formatForecastDate(d.date);
      const hi = d.highTemperatureCelsius;
      const lo = d.lowTemperatureCelsius;
      const chance = Number.isFinite(d.precipitationChance) ? clamp(d.precipitationChance, 0, 100) : 0;
      const fill = el('span', { class: 'range-fill' });
      if (finite(hi) && finite(lo)) {
        fill.style.setProperty('--l', (((lo - min) / span) * 100).toFixed(1));
        fill.style.setProperty('--w', Math.max(4, ((hi - lo) / span) * 100).toFixed(1));
        fill.style.setProperty('--c1', temperatureColor(lo));
        fill.style.setProperty('--c2', temperatureColor(hi));
      }
      const rain = el('span', { class: 'rain-fill' });
      rain.style.setProperty('--p', chance);
      const row = el(
        'article',
        { class: 'day' },
        el('div', { class: 'day-name' }, when.weekday, el('small', null, when.rest)),
        icon(conditionKind(d.conditions, true)),
        el(
          'div',
          { class: 'range' },
          el('span', { class: 'lo' }, formatTemp(lo, unit)),
          el('span', { class: 'range-track' }, fill),
          el('span', null, formatTemp(hi, unit)),
        ),
        el('div', { class: 'cond' }, sanitize(d.conditions, 120) || '—'),
        el('div', { class: 'rain' }, el('span', { class: 'rain-track' }, rain), `${Number.isFinite(d.precipitationChance) ? chance : '—'}%`),
      );
      row.style.setProperty('--i', i);
      nodes.forecast.append(row);
    });
  }

  function renderDetail(state) {
    const city = state.cities.find((c) => c.key === state.selectedKey);
    const stale = state.selectedKey && !city ? state.lastSelected : null;
    const shown = city ?? stale;
    if (!shown) {
      nodes.detail.hidden = true;
      detailSig = '';
      return;
    }
    const entry = state.forecasts.get(shown.key);
    const sig = JSON.stringify([shown.key, state.unit, entry?.status, entry?.data, shown.record, !city]);
    if (sig === detailSig && !nodes.detail.hidden) return;
    const reopen = nodes.detail.hidden;
    detailSig = sig;
    nodes.detail.hidden = false;
    const r = shown.record ?? {};
    nodes.title.textContent = sanitize(shown.name);
    nodes.sub.textContent = sanitize(shown.country) || '';
    nodes.note.hidden = !!city;
    nodes.note.textContent = city ? '' : 'This record is no longer available';
    const day = shown.onGlobe ? isDaylight(shown.lat, shown.lng, getSun()) : true;
    clear(nodes.current);
    nodes.current.append(
      el(
        'div',
        { class: 'current-main' },
        icon(conditionKind(r.description, day)),
        el('div', null, el('div', { class: 'big-temp' }, formatTemp(r.temperatureCelsius, state.unit, 1)), el('div', { class: 'desc' }, sanitize(r.description, 120) || 'No description')),
      ),
      el(
        'div',
        { class: 'stats' },
        statBlock('Humidity', formatNumber(r.humidity, 0, '%')),
        statBlock('Wind', formatNumber(r.windSpeedKmh, 1, ' km/h')),
        statBlock('Recorded', formatRecordedAt(r.recordedAt), true),
      ),
    );
    renderForecast(entry, state.unit, shown);
    if (reopen) nodes.detail.scrollTop = 0;
  }

  return {
    announce,
    render(state) {
      renderLegend(state.unit);
      for (const b of document.querySelectorAll('.unit-toggle button')) {
        b.setAttribute('aria-pressed', String(b.dataset.unit === state.unit));
      }
      renderStatus(state);
      renderList(state);
      renderDetail(state);
    },
    showFallbackNote(message) {
      nodes.banner.hidden = false;
      nodes.banner.textContent = message;
    },
  };
}
