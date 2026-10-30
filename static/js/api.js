const TIMEOUT_MS = 10_000;

export class ApiError extends Error {}
export class ApiNotFound extends Error {}

async function getJson(url, signal) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  const onAbort = () => controller.abort();
  signal?.addEventListener('abort', onAbort);
  try {
    const response = await fetch(url, {
      signal: controller.signal,
      headers: { Accept: 'application/json' },
    });
    if (response.status === 404) throw new ApiNotFound(url);
    if (!response.ok) throw new ApiError(`HTTP ${response.status}`);
    return await response.json();
  } catch (error) {
    if (error instanceof ApiNotFound || error instanceof ApiError) throw error;
    if (signal?.aborted) throw error;
    throw new ApiError(error?.name === 'AbortError' ? 'timeout' : 'network');
  } finally {
    clearTimeout(timer);
    signal?.removeEventListener('abort', onAbort);
  }
}

export async function getWeather(signal) {
  const data = await getJson('/api/weather', signal);
  if (!Array.isArray(data)) throw new ApiError('unexpected response');
  return data;
}

export async function getAlerts(signal) {
  const data = await getJson('/api/alerts', signal);
  if (!Array.isArray(data)) throw new ApiError('unexpected response');
  return data;
}

export async function getForecast(city, signal) {
  const data = await getJson(`/api/weather/forecast/${encodeURIComponent(city)}`, signal);
  if (!Array.isArray(data)) throw new ApiError('unexpected response');
  return data;
}

async function sendJson(url, method, body) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  try {
    const response = await fetch(url, {
      method,
      signal: controller.signal,
      headers: { Accept: 'application/json', ...(body ? { 'Content-Type': 'application/json' } : {}) },
      body: body ? JSON.stringify(body) : undefined,
    });
    if (response.status === 404) throw new ApiNotFound(url);
    if (!response.ok) throw new ApiError(`HTTP ${response.status}`);
    return response.status === 204 ? null : await response.json();
  } catch (error) {
    if (error instanceof ApiNotFound || error instanceof ApiError) throw error;
    throw new ApiError(error?.name === 'AbortError' ? 'timeout' : 'network');
  } finally {
    clearTimeout(timer);
  }
}

export async function getFavourites(signal) {
  const data = await getJson('/api/favourites', signal);
  if (!Array.isArray(data)) throw new ApiError('unexpected response');
  return data;
}

export function addFavourite(city) {
  return sendJson('/api/favourites', 'POST', { city });
}

export function removeFavourite(city) {
  return sendJson(`/api/favourites/${encodeURIComponent(city)}`, 'DELETE');
}
