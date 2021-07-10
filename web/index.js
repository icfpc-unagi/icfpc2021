import * as wasm from "icfpc2021";

(async function () {
  let el_container = document.getElementById('_container')
  let el_message = document.getElementById('_message')
  let el_pose = document.getElementById('_pose')

  async function render_problem_for_hash(hash) {
    if (!hash.startsWith('#')) return
    let params = Object.fromEntries(hash.substr(1).split('&').map(e => e.split('=', 2).map(e => decodeURIComponent(e))))
    if (params['pose_url'] && params['problem_url']) {
      let p_problem = fetch(params['problem_url']).then(resp => resp.text())
      let p_pose = fetch(params['pose_url']).then(resp => resp.text())
      let problem = await p_problem
      let pose = await p_pose
      el_container.innerHTML = wasm.render_pose(problem, pose)
      el_message.textContent = `score: ${wasm.calculate_score(problem, pose)}`
      el_pose.textContent = pose
    } else if (params['problem_url']) {
      el_container.innerHTML = wasm.render_problem(await fetch(params['problem_url']).then(resp => resp.text()))
      el_message.textContent = ''
      el_pose.textContent = ''
    } else {
      throw params
    }
  }

  addEventListener('hashchange', _ => render_problem_for_hash(location.href), false)
  await render_problem_for_hash(location.hash)
})()
