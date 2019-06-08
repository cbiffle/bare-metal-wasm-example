#![no_std]

#[no_mangle]
pub extern fn the_answer() -> u32 {
    42
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
