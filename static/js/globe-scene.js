import { el, clear, icon } from './dom.js';
import { subsolarPoint, toVector, isDaylight } from './sun.js';
import { conditionKind, formatTemp, temperatureColor, sanitize } from './weather.js';
import { isFavourite } from './favourites.js';

const IMG = 'https://cdn.jsdelivr.net/npm/three-globe@2.45.6/example/img/';
const DAY = `${IMG}earth-blue-marble.jpg`;
const NIGHT = `${IMG}earth-night.jpg`;
const WATER = `${IMG}earth-water.png`;
const BUMP = `${IMG}earth-topology.png`;
const SKY = `${IMG}night-sky.png`;

const FRAG_HEAD = `
varying vec3 vObjPos;
uniform vec3 uSunView;
uniform sampler2D uNightMap;
uniform sampler2D uWaterMask;
uniform float uTime;
uniform float uReduced;
float wgHash(vec3 p){ p = fract(p * 0.3183099 + 0.1); p *= 17.0; return fract(p.x * p.y * p.z * (p.x + p.y + p.z)); }
float wgNoise(vec3 x){
  vec3 i = floor(x); vec3 f = fract(x); f = f * f * (3.0 - 2.0 * f);
  return mix(
    mix(mix(wgHash(i), wgHash(i + vec3(1,0,0)), f.x), mix(wgHash(i + vec3(0,1,0)), wgHash(i + vec3(1,1,0)), f.x), f.y),
    mix(mix(wgHash(i + vec3(0,0,1)), wgHash(i + vec3(1,0,1)), f.x), mix(wgHash(i + vec3(0,1,1)), wgHash(i + vec3(1,1,1)), f.x), f.y),
    f.z);
}
float wgFbm(vec3 p){
  float a = 0.5; float s = 0.0;
  for (int i = 0; i < 5; i++) { s += a * wgNoise(p); p = p * 2.03 + vec3(1.7, 9.2, 3.1); a *= 0.5; }
  return s;
}
`;

const FRAG_BODY = `
{
  vec3 nrm = normalize(vNormal);
  vec3 V = normalize(vViewPosition);
  float d = dot(nrm, uSunView);
  float dayW = smoothstep(-0.10, 0.20, d);
  float nightW = 1.0 - smoothstep(-0.20, 0.05, d);
  vec3 dayCol = diffuseColor.rgb;
  vec3 nightCol = dayCol * 0.04;
  float water = 0.0;
  #ifdef USE_MAP
    nightCol += texture2D(uNightMap, vMapUv).rgb * 1.4 * nightW;
    water = texture2D(uWaterMask, vMapUv).r;
  #endif
  float lightAmt = 0.42 + 0.58 * clamp(d * 1.6, 0.0, 1.0);
  vec3 col = mix(nightCol, dayCol * lightAmt, dayW);

  #ifdef USE_BUMPMAP
    float h = texture2D(bumpMap, vBumpMapUv).r;
    col *= 1.0 + (h - 0.25) * 0.35 * dayW;
  #endif

  col += vec3(1.0, 0.541, 0.298) * 0.25 * exp(-pow(d / 0.12, 2.0));

  vec3 H = normalize(normalize(uSunView) + V);
  float spec = pow(max(dot(nrm, H), 0.0), 120.0) * 0.45 * water * dayW;
  col += vec3(1.0, 0.95, 0.85) * spec;

  vec3 cp = normalize(vObjPos);
  float ang = uTime * 0.002 * (1.0 - uReduced);
  float cs = cos(ang); float sn = sin(ang);
  cp = vec3(cp.x * cs + cp.z * sn, cp.y, -cp.x * sn + cp.z * cs);
  float clouds = smoothstep(0.55, 0.80, wgFbm(cp * 4.5)) * 0.85;
  vec3 cloudCol = mix(vec3(0.03, 0.04, 0.07), vec3(1.0) * lightAmt, dayW);
  col = mix(col, cloudCol, clouds);

  float fres = pow(1.0 - max(dot(nrm, V), 0.0), 3.0);
  col += vec3(0.353, 0.663, 1.0) * fres * mix(0.15, 0.35, dayW);

  gl_FragColor = vec4(col, 1.0);
}
`;

function loadImage(url) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => resolve(img);
    img.onerror = () => reject(new Error(`failed to load ${url}`));
    img.src = url;
  });
}

function waitFor(test, timeoutMs) {
  return new Promise((resolve, reject) => {
    const start = performance.now();
    const tick = () => {
      const value = test();
      if (value) resolve(value);
      else if (performance.now() - start > timeoutMs) reject(new Error('timeout'));
      else setTimeout(tick, 100);
    };
    tick();
  });
}

export function createGlobeScene({ container, onSelect, onFail }) {
  if (typeof window.Globe !== 'function') throw new Error('globe.gl not loaded');

  const motion = window.matchMedia('(prefers-reduced-motion: reduce)');
  let reduced = motion.matches;
  let sun = subsolarPoint();
  let selectedKey = null;
  let idleTimer = null;
  let shaded = false;
  const pins = new Map();
  let pinList = [];
  const uniforms = {};

  const globe = window.Globe({ animateIn: false, rendererConfig: { antialias: true, alpha: true } })(container)
    .globeImageUrl(DAY)
    .bumpImageUrl(BUMP)
    .backgroundImageUrl(SKY)
    .atmosphereColor('#4da6ff')
    .atmosphereAltitude(0.2)
    .htmlLat((d) => d.lat)
    .htmlLng((d) => d.lng)
    .htmlAltitude(0.012)
    .htmlElement((d) => d.el)
    .htmlTransitionDuration(0)
    .ringLat((d) => d.lat)
    .ringLng((d) => d.lng)
    .ringColor(() => (t) => `rgba(90, 169, 255, ${(1 - t) * 0.9})`)
    .ringMaxRadius(4)
    .ringPropagationSpeed(2.4)
    .ringRepeatPeriod(1600)
    .onZoom(({ altitude }) => container.classList.toggle('zoomed', altitude < 1.4));

  if (typeof globe.htmlElementVisibilityModifier === 'function') {
    globe.htmlElementVisibilityModifier((node, visible) => node.classList.toggle('hidden', !visible));
  }

  const renderer = globe.renderer();
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.domElement.addEventListener('webglcontextlost', (e) => {
    e.preventDefault();
    onFail?.(new Error('webgl context lost'));
  });

  const controls = globe.controls();
  controls.minDistance = 130;
  controls.maxDistance = 600;
  controls.rotateSpeed = 0.6;
  controls.zoomSpeed = 0.8;
  controls.enableDamping = true;
  controls.enablePan = false;
  controls.autoRotateSpeed = 0.25;

  function applyAutoRotate() {
    controls.autoRotate = !reduced && !selectedKey;
  }

  function onInteract() {
    controls.autoRotate = false;
    clearTimeout(idleTimer);
    idleTimer = setTimeout(applyAutoRotate, 8000);
  }
  controls.addEventListener('start', onInteract);
  renderer.domElement.addEventListener('wheel', onInteract, { passive: true });
  window.addEventListener('keydown', onInteract);

  const resize = () => globe.width(container.clientWidth).height(container.clientHeight);
  new ResizeObserver(resize).observe(container);
  resize();

  document.addEventListener('visibilitychange', () => {
    if (document.hidden) globe.pauseAnimation();
    else globe.resumeAnimation();
  });

  motion.addEventListener('change', (e) => {
    reduced = e.matches;
    uniforms.uReduced && (uniforms.uReduced.value = reduced ? 1 : 0);
    applyAutoRotate();
    syncRings();
  });

  function pov(lat, lng, altitude, ms) {
    globe.pointOfView({ lat, lng, altitude }, reduced ? 0 : ms);
  }

  pov(20, sun.lng, reduced ? 2.5 : 6, 0);
  if (!reduced) setTimeout(() => pov(20, sun.lng, 2.5, 2500), 50);
  applyAutoRotate();

  async function installShader() {
    const material = globe.globeMaterial();
    const map = await waitFor(() => material.map, 15000);
    const [nightImg, waterImg] = await Promise.all([loadImage(NIGHT), loadImage(WATER)]);
    const Texture = map.constructor;
    const Vector3 = globe.camera().position.constructor;
    const night = new Texture(nightImg);
    night.colorSpace = map.colorSpace;
    night.needsUpdate = true;
    const water = new Texture(waterImg);
    water.needsUpdate = true;

    Object.assign(uniforms, {
      uSunView: { value: new Vector3(1, 0, 0) },
      uNightMap: { value: night },
      uWaterMask: { value: water },
      uTime: { value: 0 },
      uReduced: { value: reduced ? 1 : 0 },
    });

    material.onBeforeCompile = (shader) => {
      if (!shader.fragmentShader.includes('#include <opaque_fragment>')) {
        throw new Error('unexpected shader layout');
      }
      Object.assign(shader.uniforms, uniforms);
      shader.vertexShader = shader.vertexShader
        .replace('#include <common>', '#include <common>\nvarying vec3 vObjPos;')
        .replace('#include <begin_vertex>', '#include <begin_vertex>\nvObjPos = position;');
      shader.fragmentShader = shader.fragmentShader
        .replace('#include <common>', `#include <common>\n${FRAG_HEAD}`)
        .replace('#include <opaque_fragment>', FRAG_BODY);
    };
    material.customProgramCacheKey = () => 'weather-globe-v1';
    material.needsUpdate = true;

    const mesh = globe.scene().getObjectByProperty('material', material) ?? findMesh(globe.scene(), material);
    if (!mesh) throw new Error('globe mesh not found');
    const sunVec = new Vector3();
    mesh.onBeforeRender = (_r, _s, camera) => {
      const s = toVector(sun.lat, sun.lng);
      sunVec.set(s.x, s.y, s.z).transformDirection(camera.matrixWorldInverse);
      uniforms.uSunView.value.copy(sunVec);
      uniforms.uTime.value = reduced ? 0 : performance.now() / 1000;
    };
    shaded = true;
  }

  function findMesh(root, material) {
    let found = null;
    root.traverse((o) => {
      if (!found && o.material === material) found = o;
    });
    return found;
  }

  const shaderReady = installShader().catch((error) => {
    console.warn('globe shader unavailable, using plain textures', error);
  });

  function buildPin(pin) {
    // globe.gl positions the root with an inline transform, so styling lives on the inner button
    const node = el('button', { type: 'button', class: 'pin' });
    node.addEventListener('click', () => onSelect?.(pin.key));
    pin.el = el('div', { class: 'pin-root' }, node);
    return node;
  }

  function updatePin(pin, unit) {
    const { city, node } = pin;
    const celsius = city.record?.temperatureCelsius;
    const kind = conditionKind(city.record?.description, isDaylight(city.lat, city.lng, sun));
    const favourite = isFavourite(lastFavourites, city);
    clear(node);
    node.append(
      icon(kind),
      el('span', { class: 'pin-temp' }, formatTemp(celsius, unit)),
      el('span', { class: 'pin-name' }, sanitize(city.name)),
    );
    if (favourite) node.append(el('span', { class: 'pin-star', 'aria-hidden': 'true' }, '★'));
    node.style.setProperty('--pc', temperatureColor(celsius));
    node.setAttribute(
      'aria-label',
      `${sanitize(city.name)}, ${formatTemp(celsius, unit)}, ${sanitize(city.record?.description) || 'no conditions'}${favourite ? ', favourite' : ''}`,
    );
    node.classList.toggle('favourite', favourite);
    node.classList.toggle('selected', city.key === selectedKey);
  }

  function syncRings() {
    const target = pinList.find((p) => p.key === selectedKey);
    globe.ringsData(target && !reduced ? [target] : []);
  }

  let lastUnit = 'C';
  let lastFavourites = [];

  return {
    get shaded() {
      return shaded;
    },
    shaderReady,
    sun: () => sun,
    setCities(cities, unit, favourites) {
      lastUnit = unit;
      if (favourites) lastFavourites = favourites;
      const seen = new Set();
      for (const city of cities) {
        if (!city.onGlobe) continue;
        seen.add(city.key);
        let pin = pins.get(city.key);
        if (!pin) {
          pin = { key: city.key, lat: city.lat, lng: city.lng, node: null, el: null, city };
          pin.node = buildPin(pin);
          pins.set(city.key, pin);
        }
        pin.city = city;
        updatePin(pin, unit);
      }
      for (const key of [...pins.keys()]) if (!seen.has(key)) pins.delete(key);
      pinList = [...pins.values()];
      globe.htmlElementsData(pinList);
      syncRings();
    },
    setUnit(unit, cities) {
      this.setCities(cities, unit);
    },
    refreshSun(cities) {
      sun = subsolarPoint();
      this.setCities(cities, lastUnit);
    },
    select(key, city) {
      selectedKey = key;
      for (const pin of pins.values()) pin.node.classList.toggle('selected', pin.key === key);
      syncRings();
      controls.autoRotate = false;
      clearTimeout(idleTimer);
      if (key && city?.onGlobe) pov(city.lat, city.lng, 0.9, 1600);
    },
    reset() {
      selectedKey = null;
      for (const pin of pins.values()) pin.node.classList.remove('selected');
      syncRings();
      const current = globe.pointOfView();
      pov(20, current.lng, 2.5, 1600);
      clearTimeout(idleTimer);
      idleTimer = setTimeout(applyAutoRotate, 8000);
    },
    clearSelection() {
      selectedKey = null;
      for (const pin of pins.values()) pin.node.classList.remove('selected');
      syncRings();
      applyAutoRotate();
    },
  };
}
