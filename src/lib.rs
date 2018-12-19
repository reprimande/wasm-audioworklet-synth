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
pub extern fn process_array(inputs_ptr: *const f32, len: i32, g: f32) -> const *f32 {
    let inputs: &[f32] = unsafe {std::slice::from_raw_parts(inputs_ptr, len as usize)};
    let mut outputs: &[f32]
    for i in 0..(len - 1) {
        outputs[i] = inputs[i]
    }
    &outputs
}
