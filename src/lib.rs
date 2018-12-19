#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn process(x: u32) -> u32 {
    x * x
}

#[no_mangle]
pub extern fn foo(x: u32) -> u32 {
    x * x
}
