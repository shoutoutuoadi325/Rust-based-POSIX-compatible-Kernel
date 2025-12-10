//! Trap context for saving registers

use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// Trap context saved on kernel stack
pub struct TrapContext {
    /// General registers x0-x31
    pub x: [usize; 32],
    /// Supervisor status register
    pub sstatus: Sstatus,
    /// Supervisor exception program counter
    pub sepc: usize,
}

impl TrapContext {
    /// Create an empty trap context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let sstatus = sstatus::read();
        // Note: set_spp is not available in riscv 0.10, we'll manually set bits if needed
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }

    /// Set stack pointer
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
}
