const ctx = new AudioContext()
ctx.audioWorklet.addModule('js/p.js?t=' + new Date().getTime()).then(() => {
  const n = new AudioWorkletNode(ctx, 'my-processor')
  const o = new OscillatorNode(ctx)

  o.connect(n).connect(ctx.destination)
  o.start()

  fetch('wasm/wasm_audioworklet.wasm?t=' + new Date().getTime())
    .then(r => r.arrayBuffer())
    .then(r => n.port.postMessage(r))
})
