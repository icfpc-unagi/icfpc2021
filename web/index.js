import * as wasm from "icfpc2021";

async function render_problem_for_hash(hash) {
  if (!hash.startsWith('#')) return
  let params = Object.fromEntries(hash.substr(1).split('&').map(e => e.split('=', 2).map(e => decodeURIComponent(e))))
  if (params['pose_url'] && params['problem_url']) {
    let problem = fetch(params['problem_url']).then(resp => resp.text())
    let pose = fetch(params['pose_url']).then(resp => resp.text())
    wasm.render_pose(await problem, await pose);
  } else if (params['problem_url']) {
    let problem = fetch(params['problem_url']).then(resp => resp.text())
    wasm.render_problem(await problem);
  } else {
    throw params
  }
}

(async function () {
  addEventListener('hashchange', e => render_problem_for_hash(location.href), false);
  await render_problem_for_hash(location.hash)
})()
