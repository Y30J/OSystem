#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]

mod sbi;
mod console;
mod lang_items;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
use core::arch::asm;

#[no_mangle]
extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(9);
}

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    loop{};
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

