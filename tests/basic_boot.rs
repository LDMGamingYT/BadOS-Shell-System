#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(bad_os_shell_system::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bad_os_shell_system::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bad_os_shell_system::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
