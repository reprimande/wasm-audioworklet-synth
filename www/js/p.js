class MyProcessor extends AudioWorkletProcessor {
  constructor() {
    super()
    this.port.onmessage = e => {
      WebAssembly.instantiate(e.data).then(w => {
        console.log(w.instance.exports)
        console.log(w.instance.exports.memory.buffer)
        this._wasm = w.instance
      })
    }
  }

  process(inputs, outputs, parameters) {
    if (!this._wasm) {
      return true
    }

    let input = inputs[0]
    let output = outputs[0]
    // let gain = parameters.gain
    for (let channel = 0; channel < input.length; ++channel) {
      let inputChannel = input[channel]
      let outputChannel = output[channel]
      const size = inputChannel.length
      const inPtr = this._wasm.exports.alloc(size)
      const outPtr = this._wasm.exports.alloc(size)
      const inBuf = new Float32Array(
        this._wasm.exports.memory.buffer,
        inPtr,
        size
      )
      const outBuf = new Float32Array(
        this._wasm.exports.memory.buffer,
        outPtr,
        size
      )
      for (let i = 0; i < size; i++) {
        inBuf[i] = inputChannel[i]
      }
      this._wasm.exports.process_array(inPtr, outPtr, size, 0.5)
      for (let i = 0; i < size; i++) {
        outputChannel[i] = outBuf[i]
      }

      // for (let i = 0; i < inputChannel.length; ++i) {
      //   outputChannel[i] = this._wasm.exports.process(inputChannel[i], 0.1)
      // }
    }

    return true
  }
}

registerProcessor('my-processor', MyProcessor)
