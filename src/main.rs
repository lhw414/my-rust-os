#![no_std]
#![no_main]
#![feature(asm)]

mod arch;
mod interrupts;
mod println;

use crate::arch::gdt;
use crate::arch::interrupts::IDT;
use crate::interrupts::init as interrupts_init;

// Rust 엔트리 포인트
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // GDT(Global Descriptor Table)를 초기화합니다.
    gdt::init();

    // 인터럽트를 초기화합니다.
    interrupts_init();

    // 무한 루프를 실행합니다.
    loop {}
}
