// Copyright (c) 2024 Logan Dhillon

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(bad_os_shell_system::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod qemu;
mod serial;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bad_os_shell_system::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to BadOS Shell System.");

    bad_os_shell_system::init();

    #[cfg(test)]
    test_main();

    loop {
        use bad_os_shell_system::print;
        print!("!");
    }
}