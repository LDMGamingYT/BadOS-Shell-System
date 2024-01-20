// Copyright (c) 2024 Logan Dhillon

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

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
    serialln!("[failed]\n");
    serialln!("Error: {}\n", info);
    qemu::exit_qemu(qemu::ExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serialln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit_qemu(qemu::ExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to BadOS Shell System.");

    #[cfg(test)]
    test_main();

    loop {}
}


#[test_case]
fn trivial_assertion() {
    serial!("trivial assertion... ");
    assert_eq!(1, 2);
    serialln!("[ok]");
}