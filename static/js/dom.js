const SVG_NS = 'http://www.w3.org/2000/svg';

function apply(node, attrs, children) {
  for (const [name, value] of Object.entries(attrs ?? {})) {
    if (value === null || value === undefined || value === false) continue;
    if (name === 'class') node.setAttribute('class', String(value));
    else if (name === 'vars') {
      for (const [k, v] of Object.entries(value)) node.style.setProperty(k, String(v));
    } else if (name === 'on') {
      for (const [evt, fn] of Object.entries(value)) node.addEventListener(evt, fn);
    } else if (/^(on|style$)/i.test(name)) {
      throw new Error(`attribute not allowed: ${name}`);
    } else node.setAttribute(name, value === true ? '' : String(value));
  }
  for (const child of children.flat()) {
    if (child === null || child === undefined || child === false) continue;
    node.append(child instanceof Node ? child : document.createTextNode(String(child)));
  }
  return node;
}

/** Strings become text nodes; there is intentionally no HTML-string API. */
export function el(tag, attrs, ...children) {
  return apply(document.createElement(tag), attrs, children);
}

export function svgEl(tag, attrs, ...children) {
  return apply(document.createElementNS(SVG_NS, tag), attrs, children);
}

export function clear(node) {
  node.replaceChildren();
}

const ICONS = {
  sun: () => [
    svgEl('circle', { cx: 12, cy: 12, r: 4, class: 'fill' }),
    ...[0, 45, 90, 135, 180, 225, 270, 315].map((a) => {
      const r = (a * Math.PI) / 180;
      const p = (d) => (12 + Math.sin(r) * d).toFixed(2);
      const q = (d) => (12 - Math.cos(r) * d).toFixed(2);
      return svgEl('line', { x1: p(7), y1: q(7), x2: p(10), y2: q(10) });
    }),
  ],
  moon: () => [svgEl('path', { d: 'M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5z', class: 'fill' })],
  cloud: () => [svgEl('path', { d: 'M7 18h10a4 4 0 0 0 .5-8A5.5 5.5 0 0 0 7 9.5 4.3 4.3 0 0 0 7 18z' })],
  partly: () => [
    svgEl('circle', { cx: 8, cy: 8, r: 3, class: 'fill', opacity: 0.85 }),
    svgEl('path', { d: 'M8 19h9a3.5 3.5 0 0 0 .4-7A5 5 0 0 0 8 12.5 3.3 3.3 0 0 0 8 19z' }),
  ],
  rain: () => [
    svgEl('path', { d: 'M7 14h10a4 4 0 0 0 .5-8A5.5 5.5 0 0 0 7 5.5 4.3 4.3 0 0 0 7 14z' }),
    svgEl('path', { d: 'M8 17l-1 3M12 17l-1 3M16 17l-1 3' }),
  ],
  storm: () => [
    svgEl('path', { d: 'M7 14h10a4 4 0 0 0 .5-8A5.5 5.5 0 0 0 7 5.5 4.3 4.3 0 0 0 7 14z' }),
    svgEl('path', { d: 'M12 14l-2 4h3l-2 4', class: 'bolt' }),
  ],
  snow: () => [
    svgEl('path', { d: 'M12 3v18M4.2 7.5l15.6 9M4.2 16.5l15.6-9' }),
  ],
  fog: () => [svgEl('path', { d: 'M4 8h16M6 12h14M4 16h16M8 20h10' })],
  wind: () => [
    svgEl('path', { d: 'M3 9h11a3 3 0 1 0-3-3M3 15h15a3 3 0 1 1-3 3M3 12h8' }),
  ],
  humid: () => [svgEl('path', { d: 'M12 3s6 6.5 6 11a6 6 0 0 1-12 0c0-4.5 6-11 6-11z' })],
  unknown: () => [svgEl('circle', { cx: 12, cy: 12, r: 8 }), svgEl('path', { d: 'M12 8v5M12 16v.5' })],
};

export function icon(kind) {
  const key = ICONS[kind] ? kind : 'unknown';
  return svgEl(
    'svg',
    { class: `ico k-${key}`, viewBox: '0 0 24 24', 'aria-hidden': 'true', focusable: 'false' },
    ...ICONS[key](),
  );
}
