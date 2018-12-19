const ctx = AudioContext()
ctx.audioWorklet.addModule('js/p.js').then(() => {
  const n = new AudioWorkletNode(ctx, 'my-processor')
  const o = new OscillatorNode(ctx)

  o.connect(n).connect(ctx.destination)
  o.start()

  n.port.onmessage = e => {
    console.log('onmessage node:', e)
  }

  fetch('wasm/wasm_audioworklet.wasm')
    .then(r => r.arrayBuffer())
    .then(r => n.port.postMessage(r))
})
