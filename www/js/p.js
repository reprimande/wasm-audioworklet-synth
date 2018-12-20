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
          this._time = 0
        })
      }
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
      this._inBuf.set(inputChannel)

      this._time = this._wasm.exports.sine_wave(
        this._outPtr,
        this._size,
        this._time, // TODO
        parameters.freq[0],
        parameters.gain[0] * parameters.onoff[0]
      )
      outputChannel.set(this._outBuf)
    }

    return true
  }
}

registerProcessor('my-processor', MyProcessor)
