// Real-browser smoke check for the globe UI (Node 22+, Google Chrome).
// Usage: node scripts/browser-check.mjs http://localhost:<port>/ [screenshot.png]
// Prints NET (response status), EXC (uncaught or caught JS exceptions) and TEXT lines.
// Exits 1 on any exception, non-2xx local response, or literal "null"/"undefined" in page text.
import { spawn } from 'node:child_process';
import { writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const [url, shot] = process.argv.slice(2);
if (!url) { console.error('usage: browser-check.mjs <url> [screenshot.png]'); process.exit(2); }
const chrome = process.env.CHROME ?? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const port = 9300 + Math.floor(Math.random() * 600);
const proc = spawn(chrome, ['--headless=new', '--use-angle=swiftshader', '--enable-unsafe-swiftshader',
  '--window-size=1600,1000', `--remote-debugging-port=${port}`,
  `--user-data-dir=${mkdtempSync(join(tmpdir(), 'browser-check-'))}`, 'about:blank'], { stdio: 'ignore' });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
await sleep(2500);
const page = (await (await fetch(`http://127.0.0.1:${port}/json`)).json()).find((t) => t.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
let id = 0; const pending = {}; let failed = false;
const call = (method, params = {}) => new Promise((r) => { pending[++id] = r; ws.send(JSON.stringify({ id, method, params })); });
const origin = new URL(url).origin;
ws.onmessage = (m) => {
  const d = JSON.parse(m.data);
  if (d.id && pending[d.id]) return pending[d.id](d.result);
  if (d.method === 'Network.responseReceived') {
    const { status, url: u } = d.params.response;
    console.log('NET', status, u);
    if (u.startsWith(origin) && (status < 200 || status >= 300)) failed = true;
  }
  if (d.method === 'Network.loadingFailed') { console.log('NETFAIL', d.params.errorText); failed = true; }
  if (d.method === 'Debugger.paused') {
    const f = d.params.callFrames[0];
    console.log('EXC', d.params.data?.description?.split('\n')[0], '@', f?.url, (f?.location.lineNumber ?? 0) + 1);
    if (f?.url.startsWith(origin)) failed = true;
    call('Debugger.resume');
  }
};
await new Promise((r) => (ws.onopen = r));
for (const m of ['Runtime.enable', 'Debugger.enable', 'Page.enable', 'Network.enable']) await call(m);
await call('Debugger.setPauseOnExceptions', { state: 'all' });
await call('Page.navigate', { url });
await sleep(9000); // real time: never use --virtual-time-budget, it fast-forwards fetch timeouts
const text = (await call('Runtime.evaluate', { expression: 'document.body.innerText' })).result.value ?? '';
console.log('TEXT', JSON.stringify(text.slice(0, 600)));
if (/\b(null|undefined|NaN)\b/.test(text)) { console.log('FAIL literal null/undefined/NaN in page text'); failed = true; }
if (shot) writeFileSync(shot, Buffer.from((await call('Page.captureScreenshot', { format: 'png' })).data, 'base64'));
ws.close(); proc.kill();
console.log(failed ? 'RESULT: FAIL' : 'RESULT: PASS');
process.exit(failed ? 1 : 0);
