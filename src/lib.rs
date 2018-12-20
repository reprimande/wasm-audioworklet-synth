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

#[no_mangle]
pub extern "C" fn sine_wave(out_ptr: *mut f32, size: usize, time: i32, freq: f32, gain: f32) -> i32 {
    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..size {
        out_buf[i] = ((i as f32 + time as f32) * STEP * freq).sin() as f32 * gain
    }
    time + size as i32
}
