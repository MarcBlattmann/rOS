#![no_std]
#![no_main]
use core::panic::PanicInfo;

//Start of the os
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {

    }
}

//This function is called on panic / error
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        //Loop to infinity
    }
}