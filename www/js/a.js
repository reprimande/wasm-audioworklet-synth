const ctx = new AudioContext()
ctx.audioWorklet.addModule('js/p.js?t=' + new Date().getTime()).then(() => {
  const n = new AudioWorkletNode(ctx, 'my-processor')
  n.connect(ctx.destination)

  fetch('wasm/wasm_audioworklet.wasm?t=' + new Date().getTime())
    .then(r => r.arrayBuffer())
    .then(r => n.port.postMessage({ type: 'loadWasm', data: r }))

  const key = document.getElementById('key')
  key.addEventListener('change', e => {
    n.parameters.get('freq').value =
      440.0 * Math.pow(2.0, (e.note[1] - 69) / 12)
    if (e.note[0] === 1) {
      n.port.postMessage({ type: 'trigger' })
    }
  })

  const cutoff = document.getElementById('cutoff')
  cutoff.addEventListener('input', e => {
    n.parameters.get('cutoff').value = e.target.value
  })

  const q = document.getElementById('q')
  q.addEventListener('input', e => {
    n.parameters.get('q').value = e.target.value * 0.5
  })

  const amount = document.getElementById('amount')
  amount.addEventListener('input', e => {
    n.parameters.get('amount').value = e.target.value * 0.01
  })

  const decay = document.getElementById('decay')
  decay.addEventListener('input', e => {
    n.parameters.get('decay').value = e.target.value * 0.01
  })

  const gain = document.getElementById('gain')
  gain.addEventListener('input', e => {
    n.parameters.get('gain').value = e.target.value * 0.01
  })
})
