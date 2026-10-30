const RAD = Math.PI / 180;

function dayOfYear(date) {
  const start = Date.UTC(date.getUTCFullYear(), 0, 0);
  return Math.floor((date.getTime() - start) / 86_400_000);
}

/** Subsolar point in degrees; accurate to about 1 degree (visual use only). */
export function subsolarPoint(date = new Date()) {
  const n = dayOfYear(date);
  const declination = -23.44 * Math.cos(((360 / 365) * (n + 10)) * RAD);
  const b = ((360 / 365) * (n - 81)) * RAD;
  const eot = 9.87 * Math.sin(2 * b) - 7.53 * Math.cos(b) - 1.5 * Math.sin(b);
  const hours = date.getUTCHours() + date.getUTCMinutes() / 60 + date.getUTCSeconds() / 3600;
  let lng = -15 * (hours - 12 + eot / 60);
  lng = ((((lng + 180) % 360) + 360) % 360) - 180;
  return { lat: declination, lng };
}

/** Unit vector in three-globe world coordinates. */
export function toVector(lat, lng) {
  const phi = (90 - lat) * RAD;
  const theta = (90 - lng) * RAD;
  return {
    x: Math.sin(phi) * Math.cos(theta),
    y: Math.cos(phi),
    z: Math.sin(phi) * Math.sin(theta),
  };
}

export function isDaylight(lat, lng, sun) {
  const c = toVector(lat, lng);
  const s = toVector(sun.lat, sun.lng);
  return c.x * s.x + c.y * s.y + c.z * s.z > 0;
}
