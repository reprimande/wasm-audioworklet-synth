#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

#[no_mangle]
pub extern "C" fn process(in_ptr: *mut f32, out_ptr: *mut f32, size: usize, gain: f32) {
    let in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(in_ptr, size) };
    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..size {
        out_buf[i] = in_buf[i] * gain;
    }
}

const PI_2: f32 = ::std::f32::consts::PI * 2.0;
const STEP: f32 = PI_2 / 44100 as f32;

const TPS: f64 = ::std::f64::consts::PI * 2.0 / 44100 as f64;

struct Synth {
    time: i64,
    frequency: f64,
    // attack: f64,
    // decay: f64,
    // sustain: f64,
    // release: f64,
    // cutoff: f64,
    // resonance: f64,
    gain: f64,
}

impl Synth {
    fn new() -> Synth {
        Synth {
            time: 0,
            frequency: 440.0,
            gain: 0.5,
        }
    }

    fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        for i in 0..size {
            out_buf[i] =
                (((i as f64 + self.time as f64) * TPS * self.frequency).sin() * self.gain) as f32
        }
        self.time += size as i64
    }
}

static mut synth: Synth = Synth::new();

#[no_mangle]
pub extern "C" fn sine_wave(out_ptr: *mut f32, size: usize, _time: i32, freq: f32, gain: f32) {
    synth.frequency = freq as f64;
    synth.gain = gain as f64;
    synth.process(out_ptr, size);
    // let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    // for i in 0..size {
    //     out_buf[i] = ((i as f32 + time as f32) * STEP * freq).sin() as f32 * gain
    // }
    // time + size as i32
}
