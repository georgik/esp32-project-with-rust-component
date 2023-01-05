#![no_std]
#[no_mangle]
pub unsafe extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
