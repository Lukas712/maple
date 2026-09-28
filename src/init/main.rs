#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::BaseRevision;

// Define a revisão base utilizando BaseRevision::new() sem argumentos
#[used]
#[link_section = ".limine_requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

// Função para escrever na porta serial COM1
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nostack, preserves_flags)
    );
}

fn imprimir_serial(texto: &str) {
    for byte in texto.bytes() {
        unsafe {
            outb(0x3F8, byte);
        }
    }
}

// O ponto de entrada do kernel em Rust (chamado pelo Limine)
#[no_mangle]
extern "C" fn kmain() -> ! {
    // Verifica se a revisão base é suportada pelo bootloader
    if !BASE_REVISION.is_supported() {
        hcf();
    }

    imprimir_serial("\n========================================\n");
    imprimir_serial("[MAPLE KERNEL] Inicializado com sucesso em Rust!\n");
    imprimir_serial("========================================\n\n");

    hcf();
}

// Trava a CPU em caso de erro fatal (Halt and Catch Fire)
fn hcf() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

// Obrigatório em ambientes no_std para tratar falhas de pânico
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf();
}