//! Núcleo do microkernel.
//!

pub mod log;
pub mod serial;

use crate::kprintln;

/// Trava em loop  executando a instrução `hlt` (Halt and Catch Fire).
/// 
/// Usada quando não há mais nada útil a fazer.
pub fn hcf() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

pub fn kernel_main() -> ! {
    serial::init();

    kprintln!();
    kprintln!("========================================");
    kprintln!("[MAPLE KERNEL] Inicializado com sucesso em Rust!");
    kprintln!("========================================");
    kprintln!();

    hcf();
}
