//! Ponto de entrada do kernel Maple.
//!
//! - `#![no_std]`  Remove a biblioteca padrão "std" (que assume um SO por
//!   trás: heap, threads, arquivos, etc.). Equivale a compilar C com
//!   `-ffreestanding -nostdlib`.
//! - `#![no_main]` desliga o `main()` padrão do Rust (que espera um
//!   runtime rodando coisas antes dele). Definimos nosso próprio ponto de
//!   entrada manualmente (`kmain`), do mesmo jeito que o oak usa
//!   `ENTRY(kmain)` no `linker.ld`.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[path = "../boot/mod.rs"]
mod boot;

#[path = "../kernel/mod.rs"]
mod kernel;

/// Ponto de entrada do kernel, chamado diretamente pelo Limine após o
/// boot.
///
/// - `#[no_mangle]` impede o Rust de renomear internamente o nome da
///   função (name mangling). Sem isso, o símbolo exportado no binário
///   teria um nome "decorado" e o `linker.ld` não encontraria `kmain`.
/// 
/// - `extern "C"` faz esta função usar a convenção de chamada do C,
///   necessária para que código externo (o bootloader, escrito em C)
///   consiga chamá-la corretamente.
#[no_mangle]
extern "C" fn kmain() -> ! {
    if !boot::is_supported() {
        kernel::hcf();
    }

    kernel::kernel_main();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::hcf();
}
