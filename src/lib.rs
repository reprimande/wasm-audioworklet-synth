#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

mod synth;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

lazy_static! {
    static ref SYNTH: Mutex<synth::Synth> = Mutex::new(synth::Synth::new());
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
