#![no_std] // dont link the Rust standard libeary
#![no_main] // disable Rust-level entry points

mod vga_buffer;

#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // panic!("Some panic message");

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
