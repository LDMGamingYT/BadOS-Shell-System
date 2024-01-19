// Copyright (c) 2024 Logan Dhillon

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static MESSAGE: &[u8] = b"Hello, BadOS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::tmp();

    loop {}
}
