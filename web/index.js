import * as wasm from "icfpc2021";

async function render_problem_for_hash(hash) {
  if (!hash.startsWith('#')) return
  let params = Object.fromEntries(hash.substr(1).split('&').map(e => e.split('=', 2).map(e => decodeURIComponent(e))))
  if (!params['problem_url']) return
  wasm.render_problem(await fetch(params['problem_url']).then(resp => resp.text()));
}

(async function () {
  addEventListener('hashchange', e => render_problem_for_hash(e.newURL.hash), false);
  await render_problem_for_hash(location.hash)
})()
