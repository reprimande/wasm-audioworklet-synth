class MyProcessor extends AudioWorkletProcessor {
  static get parameterDescriptors() {
    return [
      {
        name: 'freq',
        defaultValue: 440.0
      },
      {
        name: 'onoff',
        defaultValue: 0
      },
      {
        name: 'gain',
        defaultValue: 0.5
      }
    ]
  }

  constructor() {
    super()
    this.port.onmessage = e => {
      if (e.data.type === 'loadWasm') {
        WebAssembly.instantiate(e.data.data).then(w => {
          this._wasm = w.instance
          this._size = 128
          this._inPtr = this._wasm.exports.alloc(this._size)
          this._outPtr = this._wasm.exports.alloc(this._size)
          this._inBuf = new Float32Array(
            this._wasm.exports.memory.buffer,
            this._inPtr,
            this._size
          )
          this._outBuf = new Float32Array(
            this._wasm.exports.memory.buffer,
            this._outPtr,
            this._size
          )
        })
      } else if (e.data.type === 'trigger') {
        this._wasm.exports.trigger()
      }
    }
  }

  process(inputs, outputs, parameters) {
    if (!this._wasm) {
      return true
    }
    this._wasm.exports.set_frequency(parameters.freq[0])
    this._wasm.exports.set_gain(parameters.gain[0])

    let output = outputs[0]
    for (let channel = 0; channel < output.length; ++channel) {
      let outputChannel = output[channel]
      this._wasm.exports.process(this._outPtr, this._size)
      outputChannel.set(this._outBuf)
    }

    return true
  }
}

registerProcessor('my-processor', MyProcessor)
