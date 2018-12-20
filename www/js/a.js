const ctx = new AudioContext()
ctx.audioWorklet.addModule('js/p.js?t=' + new Date().getTime()).then(() => {
  const n = new AudioWorkletNode(ctx, 'my-processor')
  const o = new OscillatorNode(ctx)
  o.connect(n).connect(ctx.destination)
  o.start()

  fetch('wasm/wasm_audioworklet.wasm?t=' + new Date().getTime())
    .then(r => r.arrayBuffer())
    .then(r => n.port.postMessage({ type: 'loadWasm', data: r }))

  const key = document.getElementById('key')
  key.addEventListener('change', e => {
    console.log(e.note[1])
    n.freq = 440.0 * Math.pow(2.0, (e.note[1] - 69) / 12)
    console.log(n.freq)
  })
})
