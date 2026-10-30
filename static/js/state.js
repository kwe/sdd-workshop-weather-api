const listeners = new Set();

const state = {
  cities: [],
  selectedKey: null,
  unit: 'C',
  forecasts: new Map(),
  status: { phase: 'loading', stale: false, lastOk: null, message: '' },
  query: '',
  favourites: [],
  alerts: [],
  alertsStatus: 'loading',
};

try {
  const saved = localStorage.getItem('weather-globe-unit');
  if (saved === 'C' || saved === 'F') state.unit = saved;
} catch {
  // storage unavailable; keep default
}

export function getState() {
  return state;
}

export function setState(patch) {
  Object.assign(state, patch);
  if (patch.unit) {
    try {
      localStorage.setItem('weather-globe-unit', state.unit);
    } catch {
      // ignore
    }
  }
  for (const fn of listeners) fn(state, patch);
}

export function subscribe(fn) {
  listeners.add(fn);
  return () => listeners.delete(fn);
}
