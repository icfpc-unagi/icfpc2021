import * as wasm from "icfpc2021";

(async function () {
  let el_container = document.getElementById('_container')
  let el_message = document.getElementById('_message')
  let el_pose = document.getElementById('_pose')
  let el_morph = document.getElementById('_morph')

  var problem = '';
  var pose = ''

  async function render_for_hash(hash) {
    if (!hash.startsWith('#')) return
    let params = Object.fromEntries(hash.substr(1).split('&').map(e => e.split('=', 2).map(e => decodeURIComponent(e))))
    let p_problem = params['problem_url'] && fetch(params['problem_url']).then(resp => resp.text())
    let p_pose = params['pose_url'] && fetch(params['pose_url']).then(resp => resp.text())
    problem = p_problem && await p_problem
    pose = p_pose && await p_pose
    if (render(problem, pose)) {
      el_morph.disabled = false
    }
  }

  function render(problem, pose) {
    if (pose && problem) {
      el_container.innerHTML = wasm.render_pose(problem, pose)
      el_message.textContent = `score: ${wasm.calculate_score(problem, pose)}`
      el_pose.value = pose
      return true
    } else if (problem) {
      el_container.innerHTML = wasm.render(problem)
      el_message.textContent = ''
      el_pose.value = ''
    } else {
      throw arguments
    }
    return false
  }

  addEventListener('hashchange', _ => render_for_hash(location.href), false)
  await render_for_hash(location.hash)

  el_pose.addEventListener('change', e => {
    try {
      let validated = JSON.stringify(JSON.parse(e.target.value))
      pose = validated
      render(problem, pose)
      e.target.style.boxShadow = ''
    } catch {
      e.target.style.boxShadow = '0 0 2px 2px red'
    }
  })

  el_morph.addEventListener('click', async function () {
    if (problem && pose && !this.disabled) {
      this.disabled = true
      for (let i = 0; i < 10 && this.disabled; i++) {
        pose = wasm.morph(problem, pose, 1000)
        render(problem, pose)
        await new Promise(resolve => setTimeout(resolve, 10))
      }
      this.disabled = false
    }
  })
})()
