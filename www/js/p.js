class MyProcessor extends AudioWorkletProcessor {
  constructor() {
    super()
    this.port.onmessage = e => {
      WebAssembly.instantiate(e.data).then(w => {
        console.log(w.instance.exports)
        console.log(w.instance.exports.memory.buffer)
        this._process = w.instance.exports.process
      })
    }
  }

  process(inputs, outputs, parameters) {
    if (!this._process) {
      return true
    }

    let input = inputs[0]
    let output = outputs[0]
    // let gain = parameters.gain
    for (let channel = 0; channel < input.length; ++channel) {
      let inputChannel = input[channel]
      let outputChannel = output[channel]
      for (let i = 0; i < inputChannel.length; ++i) {
        outputChannel[i] = this._process(inputChannel[i], 0.1)
      }
    }

    return true
  }
}

registerProcessor('my-processor', MyProcessor)
