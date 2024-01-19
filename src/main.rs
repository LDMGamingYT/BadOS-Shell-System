// Copyright (c) 2024 Logan Dhillon

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "Hello, BadOS!\nNew lines!").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n\nLorem ipsum dolor sit amet sunt labore laborum incididunt qui deserunt officia pariatur velit nisi occaecat quis esse nisi consectetur tempor ut fugiat ut veniam proident veniam minim pariatur non et incididunt ex velit minim ea ex mollit in fugiat pariatur cupidatat duis anim magna ex in exercitation eiusmod exercitation proident deserunt anim aliquip cillum").unwrap();

    loop {}
}
