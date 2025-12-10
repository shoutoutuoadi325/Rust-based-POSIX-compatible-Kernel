//! Trap handling module

mod context;

pub use context::TrapContext;

use crate::syscall::syscall;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

core::arch::global_asm!(include_str!("trap.S"));

/// Initialize trap handling
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// Enable timer interrupt
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
/// Handle trap from user/kernel
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!("[KERNEL] Page fault at {:#x}, bad addr = {:#x}", cx.sepc, stval);
            panic!("Page fault!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[KERNEL] Illegal instruction at {:#x}", cx.sepc);
            panic!("Illegal instruction!");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            println!("[KERNEL] Timer interrupt");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
