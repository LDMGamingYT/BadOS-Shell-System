// Copyright (c) 2024 Logan Dhillon

#![no_std]

use core::panic::PanicInfo;

fn main() {}
 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}