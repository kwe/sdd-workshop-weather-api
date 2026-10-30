import { getState, setState, subscribe } from './state.js';
import { ApiNotFound, getForecast, getWeather } from './api.js';
import { locate, normalize, normalizeCountry } from './gazetteer.js';
import { createGlobeScene } from './globe-scene.js';
import { createUi } from './ui.js';
import { sanitize } from './weather.js';

const REFRESH_MS = 60_000;
const NONE_TTL_MS = 60_000;

let scene = null;
let weatherInFlight = false;
let forecastController = null;

function buildCities(records) {
  const byKey = new Map();
  for (const record of records) {
    const name = sanitize(record?.city, 60);
    if (!name) continue;
    const key = `${normalize(name)}|${normalizeCountry(record.country)}`;
    const prev = byKey.get(key);
    const newer =
      !prev ||
      String(record.recordedAt) > String(prev.recordedAt) ||
      (String(record.recordedAt) === String(prev.recordedAt) && record.id > prev.id);
    if (newer) byKey.set(key, record);
  }
  const cities = [];
  for (const [key, record] of byKey) {
    const pos = locate(record);
    cities.push({
      key,
      name: sanitize(record.city, 60),
      country: sanitize(record.country, 60),
      record,
      lat: pos?.lat ?? 0,
      lng: pos?.lng ?? 0,
      onGlobe: !!pos,
    });
  }
  return cities.sort((a, b) => a.name.localeCompare(b.name));
}

function hhmm(date) {
  return `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
}

async function refreshWeather(ui) {
  if (weatherInFlight) return;
  weatherInFlight = true;
  try {
    const records = await getWeather();
    const cities = buildCities(records);
    const selected = getState().selectedKey;
    setState({
      cities,
      lastSelected: cities.find((c) => c.key === selected) ?? getState().lastSelected,
      status: {
        phase: 'ready',
        stale: false,
        lastOk: hhmm(new Date()),
        message: cities.length ? '' : 'No weather records yet',
      },
    });
    if (!getState().announcedLoad) {
      setState({ announcedLoad: true });
      ui.announce(`Loaded ${cities.length} cities`);
    }
  } catch {
    const { cities, status } = getState();
    if (cities.length) {
      setState({ status: { ...status, phase: 'ready', stale: true } });
      ui.announce('Offline');
    } else {
      setState({ status: { phase: 'error', stale: false, lastOk: null, message: 'Could not load weather data.' } });
    }
  } finally {
    weatherInFlight = false;
  }
}

async function loadForecast(key, ui, force = false) {
  const state = getState();
  const city = state.cities.find((c) => c.key === key);
  if (!city) return;
  const cached = state.forecasts.get(key);
  if (!force && cached) {
    if (cached.status === 'ok') return;
    if (cached.status === 'none' && Date.now() - cached.at < NONE_TTL_MS) return;
  }
  forecastController?.abort();
  forecastController = new AbortController();
  const { signal } = forecastController;
  const set = (entry) => {
    const forecasts = new Map(getState().forecasts);
    if (entry) forecasts.set(key, entry);
    else forecasts.delete(key);
    setState({ forecasts });
  };
  set({ status: 'loading', at: Date.now() });
  try {
    const data = await getForecast(String(city.record.city).trim(), signal);
    set({ status: 'ok', data, at: Date.now() });
    if (getState().selectedKey === key) ui.announce(`Forecast loaded for ${city.name}`);
  } catch (error) {
    if (signal.aborted) {
      set(null);
      return;
    }
    set({ status: error instanceof ApiNotFound ? 'none' : 'error', at: Date.now() });
    ui.announce('Forecast unavailable');
  }
}

function boot() {
  const ui = createUi({
    getSun: () => scene?.sun() ?? { lat: 0, lng: 0 },
    actions: {
      onSelect: (key) => select(key),
      onClose: () => {
        forecastController?.abort();
        setState({ selectedKey: null, lastSelected: null });
        scene?.clearSelection();
      },
      onReset: () => {
        forecastController?.abort();
        setState({ selectedKey: null, lastSelected: null });
        scene?.reset();
      },
      onUnit: (unit) => setState({ unit }),
      onQuery: (query) => setState({ query }),
      onRetryWeather: () => refreshWeather(ui),
      onRetryForecast: (key) => loadForecast(key, ui, true),
    },
  });

  function select(key) {
    const city = getState().cities.find((c) => c.key === key);
    if (!city) return;
    setState({ selectedKey: key, lastSelected: city });
    scene?.select(key, city);
    ui.announce(`Selected ${city.name}`);
    document.body.classList.remove('list-open');
    loadForecast(key, ui);
  }

  subscribe((state, patch) => {
    ui.render(state);
    if (scene && ('cities' in patch || 'unit' in patch)) scene.setCities(state.cities, state.unit);
  });

  try {
    scene = createGlobeScene({
      container: document.getElementById('globe'),
      onSelect: select,
      onFail: () => setState({ globeFailed: true }),
    });
  } catch (error) {
    console.warn('3D globe unavailable', error);
    setState({ globeFailed: true });
  }

  ui.render(getState());
  refreshWeather(ui);
  setInterval(() => {
    if (document.hidden) return;
    refreshWeather(ui);
    scene?.refreshSun(getState().cities);
  }, REFRESH_MS);
  document.addEventListener('visibilitychange', () => {
    if (!document.hidden) refreshWeather(ui);
  });
}

boot();
