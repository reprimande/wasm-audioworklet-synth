#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

const TPS: f64 = ::std::f64::consts::PI * 2.0 / 44100 as f64;

struct Synth {
    wave_phase: i64,
    frequency: f64,
    env_time: i64,
    attack: i64,
    decay: i64,
    // sustain: f64,
    // release: f64,
    // cutoff: f64,
    // resonance: f64,
    gain: f64,
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            wave_phase: 0,
            env_time: 0,
            attack: 0,
            decay: 44100,
            frequency: 440.0,
            gain: 0.5,
        }
    }

    pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        for i in 0..size {
            if self.env_time < self.decay {
                let v = self.sine_wave(self.frequency, self.wave_phase);
                let g = self.gain * ((self.decay - self.env_time) as f64 / self.env_time as f64);
                out_buf[i] = (v * g) as f32;
                self.wave_phase += 1;
                self.env_time += 1;
            } else {
                out_buf[i] = 0.0;
            }
        }
    }

    pub fn env_start(&mut self) {
        self.env_time = 0;
    }

    fn sine_wave(&self, frequency: f64, phase: i64) -> f64 {
        ((phase as f64) * TPS * frequency).sin()
    }
}

lazy_static! {
    static ref SYNTH: Mutex<Synth> = Mutex::new(Synth::new());
}

#[no_mangle]
pub extern "C" fn process(out_ptr: *mut f32, size: usize) {
    let mut synth = SYNTH.lock().unwrap();
    synth.process(out_ptr, size);
}

#[no_mangle]
pub extern "C" fn set_frequency(frequency: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.frequency = frequency as f64;
}

#[no_mangle]
pub extern "C" fn set_gain(gain: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.gain = gain as f64;
}

#[no_mangle]
pub extern "C" fn trigger() {
    let mut synth = SYNTH.lock().unwrap();
    synth.env_start()
}
