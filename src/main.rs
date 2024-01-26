// Copyright (c) 2024 Logan Dhillon. This software is subject to the Bad Technologies Open Software License.

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(bad_os_shell_system::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

mod vga_buffer;
mod qemu;
mod serial;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    bad_os_shell_system::hlt_loop();
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bad_os_shell_system::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to BadOS Shell System.");

    bad_os_shell_system::init();

    #[cfg(test)]
    test_main();

    bad_os_shell_system::hlt_loop();
}