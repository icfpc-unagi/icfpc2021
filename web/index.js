import * as wasm from "icfpc2021";

(async function () {
  let el_problem_id = document.getElementById('_problem_id')
  let el_container = document.getElementById('_container')
  let el_pose = document.getElementById('_pose')
  let el_message = document.getElementById('_message')
  let el_morph = document.getElementById('_morph')
  let el_chokudai = document.getElementById('_chokudai')

  // TODO: class
  var state = {
    problem: '',
    pose: '',
  }

  async function render_for_hash(hash) {
    if (!hash.startsWith('#')) return
    let params = Object.fromEntries(hash.substr(1).split('&').map(e => e.split('=', 2).map(e => decodeURIComponent(e))))
    if (params['problem_id']) {
      el_problem_id.value = params['problem_id']
      params['problem_url'] = `/static/problems/${params['problem_id']}.json`
    }
    let p_problem = params['problem_url'] && fetch(params['problem_url']).then(resp => resp.ok && resp.text())
    let p_pose = params['pose_url'] && fetch(params['pose_url']).then(resp => resp.ok && resp.text())
    state.problem = p_problem && await p_problem
    state.pose = p_pose && await p_pose || state.problem && JSON.stringify({ vertices: JSON.parse(state.problem).figure.vertices })
    render(state.problem, state.pose)
  }

  function render(problem, pose) {
    if (pose && problem) {
      el_container.innerHTML = wasm.render_pose(problem, pose)
      el_message.textContent = `score: ${wasm.calculate_score(problem, pose)}`
      el_pose.value = pose
      el_pose.style.boxShadow = ''
    } else if (problem) {
      el_container.innerHTML = wasm.render_problem(problem)
      el_message.textContent = ''
      el_pose.value = ''
      el_pose.style.boxShadow = ''
    } else {
      throw arguments
    }
    let handleKeyDown = ev => {
      let i = parseInt(ev.target.getAttribute('i'))
      let a = (ev.shiftKey ? 10 : 1)
      switch (ev.key) {
        case 'ArrowUp': apply(i, v => [v[0], v[1] - a], false); return false;
        case 'ArrowDown': apply(i, v => [v[0], v[1] + a], false); return false;
        case 'ArrowLeft': apply(i, v => [v[0] - a, v[1]], false); return false;
        case 'ArrowRight': apply(i, v => [v[0] + a, v[1]], false); return false;
        case 'h': apply(i, (v, p) => [2 * p[0] - v[0], v[1]], true); return false;
        case 'v': apply(i, (v, p) => [v[0], 2 * p[1] - v[1]], true); return false;
        case '-': apply(i, (v, p) => [Math.round((v[0] - p[0]) * .9 + p[0]), Math.round((v[1] - p[1]) * .9 + p[1])], true); return false;
        case '+': apply(i, (v, p) => [Math.round((v[0] - p[0]) / .9 + p[0]), Math.round((v[1] - p[1]) / .9 + p[1])], true); return false;
      }
    }
    document.querySelectorAll('circle[i]').forEach(el => el.addEventListener('keydown', handleKeyDown))
  }

  function apply(i, f, all) {
    let json = JSON.parse(state.pose)
    for (let j of all ? json.vertices.keys() : [i]) {
      json.vertices[j] = f(json.vertices[j], json.vertices[i])
    }
    state.pose = JSON.stringify(json)
    render(state.problem, state.pose)
    document.querySelector(`circle[i="${i}"]`).focus()
  }

  addEventListener('hashchange', () => render_for_hash(location.href), false)
  el_problem_id.addEventListener('change', e => {
    let newhash = `#problem_url=%2Fstatic%2Fproblems%2F${e.target.value}.json`
    history.replaceState(null, '', newhash)
    render_for_hash(newhash)
  })
  await render_for_hash(location.hash)

  el_pose.addEventListener('change', el => {
    try {
      state.pose = JSON.stringify(JSON.parse(el.target.value))
      render(state.problem, state.pose)
      el.target.style.boxShadow = ''
    } catch (e) {
      console.error(e)
      el_message.textContent = e.toString()
      el.target.style.boxShadow = '0 0 2px 2px red'
    }
  })

  el_container.addEventListener('click', e => {
    let closest = Infinity
    let closest_target = null
    el_container.querySelectorAll('circle[tabindex]').forEach(el => {
      let rect = el.getBoundingClientRect()
      let dx = (rect.left + rect.right) / 2 - e.clientX
      let dy = (rect.top + rect.bottom) / 2 - e.clientY
      let d2 = dx * dx + dy * dy
      if (d2 < closest) {
        closest = d2
        closest_target = el
      }
    })
    if (closest_target) closest_target.focus()
  })

  el_morph.addEventListener('click', async function () {
    if (state.problem && state.pose && !this.disabled) {
      this.disabled = true
      for (let i = 0; i < 10 && this.disabled; i++) {
        state.pose = wasm.morph(state.problem, state.pose, 1000)
        render(state.problem, state.pose)
        await new Promise(resolve => setTimeout(resolve, 10))
      }
      this.disabled = false
    }
  })

  el_chokudai.addEventListener('click', async function () {
    if (state.problem && state.pose && !this.disabled) {
      this.disabled = true
      state.pose = wasm.chokudai(state.problem, state.pose, 1.0, true, true)
      render(state.problem, state.pose)
      this.disabled = false
    }
  })
})()
