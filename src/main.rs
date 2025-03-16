#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

//Start of the os
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {
        //Loop to infinity
    }
}

//This function is called on panic / error
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}