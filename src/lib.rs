#[macro_use]
extern crate lazy_static;

use std::f64::consts::PI;
use std::sync::Mutex;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

const TPS: f64 = PI * 2.0 / 44100 as f64;

struct Synth {
    wave_phase: i64,
    frequency: f64,
    env_time: i64,
    attack: i64,
    decay: f64,
    // sustain: f64,
    // release: f64,
    cutoff: f64,
    amount: f64,
    q: f64,
    gain: f64,
    biquad_in1: f32,
    biquad_in2: f32,
    biquad_out1: f32,
    biquad_out2: f32,
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            wave_phase: 0,
            env_time: 0,
            attack: 0,
            decay: 1.0,
            frequency: 440.0,
            gain: 0.5,
            cutoff: 1000.0,
            q: 0.8,
            amount: 0.1,
            biquad_in1: 0.0,
            biquad_in2: 0.0,
            biquad_out1: 0.0,
            biquad_out2: 0.0,
        }
    }

    pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        let dur = self.decay * 44100.0;

        for i in 0..size {
            let v = self.sawtooth_wave(self.frequency, self.wave_phase);
            out_buf[i] = v as f32;
            self.wave_phase = self.wave_phase + 1;
        }
        let filtered = self.low_pass_filter(out_buf);
        for i in 0..size {
            out_buf[i] = filtered[i]
        }
        for i in 0..size {
            let g = self.gain * self.env_val(i);
            out_buf[i] *= g as f32;
        }
        if ((self.env_time as f64) < dur) {
            self.env_time += size as i64;
        }
    }

    pub fn env_start(&mut self) {
        self.env_time = 0;
    }

    fn env_val(&self, offset: usize) -> f64 {
        let dur = self.decay * 44100.0;
        let t = (self.env_time + offset as i64) as f64;
        match t {
            0.0 => 0.0,
            x if x >= dur => 0.0,
            _ => dur - t / dur,
        }
    }

    fn sine_wave(&self, frequency: f64, phase: i64) -> f64 {
        ((phase as f64) * TPS * frequency).sin()
    }

    fn sawtooth_wave(&self, frequency: f64, phase: i64) -> f64 {
        let t = (phase as f64) / 44100.0;
        let t_factor = t * frequency;
        t_factor - t_factor.floor() - 0.5
    }

    // http://vstcpp.wpblog.jp/?page_id=523
    fn low_pass_filter(&mut self, input: &mut [f32]) -> [f32; 128] {
        let mut output: [f32; 128] = [0.0; 128];
        let _q = match self.q {
            0.0 => 0.01,
            _ => self.q,
        };

        for i in 0..input.len() {
            let cutoff = self.cutoff + self.amount * 1000.0 * self.env_val(i);

            let omega = 2.0 * (PI as f32) * (cutoff as f32) / 44100.0;
            let alpha = omega.sin() / (2.0 * (_q as f32));
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * omega.cos();
            let a2 = 1.0 - alpha;
            let b0 = (1.0 - omega.cos()) / 2.0;
            let b1 = 1.0 - omega.cos();
            let b2 = (1.0 - omega.cos()) / 2.0;

            output[i] = b0 / a0 * input[i] + b1 / a0 * self.biquad_in1 + b2 / a0 * self.biquad_in2
                - a1 / a0 * self.biquad_out1
                - a2 / a0 * self.biquad_out2;
            self.biquad_in2 = self.biquad_in1;
            self.biquad_in1 = input[i];
            self.biquad_out2 = self.biquad_out1;
            self.biquad_out1 = output[i];
        }
        output
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
pub extern "C" fn set_cutoff(cutoff: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.cutoff = cutoff as f64;
}

#[no_mangle]
pub extern "C" fn set_q(q: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.q = q as f64;
}

#[no_mangle]
pub extern "C" fn set_decay(decay: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.decay = decay as f64;
}

#[no_mangle]
pub extern "C" fn set_amount(amount: f32) {
    let mut synth = SYNTH.lock().unwrap();
    synth.amount = amount as f64;
}

#[no_mangle]
pub extern "C" fn trigger() {
    let mut synth = SYNTH.lock().unwrap();
    synth.env_start()
}
