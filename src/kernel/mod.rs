//! Núcleo do microkernel.
//!

/// Escreve um byte na porta de I/O indicada, usando a instrução `out` da
/// x86 — é a função de mais baixo nível usada para falar com a serial
/// COM1.
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nostack, preserves_flags)
    );
}

/// Imprime uma string inteira na serial, byte a byte, usando [`outb`].
fn print_serial(texto: &str) {
    for byte in texto.bytes() {
        unsafe {
            outb(0x3F8, byte);
        }
    }
}

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
    print_serial("\n========================================\n");
    print_serial("[MAPLE KERNEL] Inicializado com sucesso em Rust!\n");
    print_serial("========================================\n\n");

    hcf();
}
