#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn process(x: f64, g: f64) -> f64 {
    x * g
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}

#[no_mangle]
pub extern fn process_array(in_ptr: *mut f32, out_ptr: *mut f32, size: usize, gain: f32) {
    let in_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(in_ptr, size) };
    let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    for i in 0..(size - 1) {
        out_buf[i] = in_buf[i] * gain;
    }
}
