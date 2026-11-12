//! Driver da porta serial COM1 (UART 16550), usada para depuração.

use core::arch::asm;

const COM1: u16 = 0x3F8;

// Offsets dos registradores em relação à porta base.
const REG_DATA: u16 = 0; // dados (DLAB=0) / divisor baixo (DLAB=1)
const REG_IER: u16 = 1; // habilitação de interrupções
const REG_FCR: u16 = 2; // controle de FIFO
const REG_LCR: u16 = 3; // controle de linha
const REG_MCR: u16 = 4; // controle de modem
const REG_LSR: u16 = 5; // status de linha

const LSR_THR_EMPTY: u8 = 0x20;

unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nostack, preserves_flags));
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    asm!("in al, dx", in("dx") port, out("al") val, options(nostack, preserves_flags));
    val
}

/// Configura a COM1: 115200 baud, 8 bits, sem paridade, 1 stop bit, FIFO ligado.
pub fn init() {
    unsafe {
        outb(COM1 + REG_IER, 0x00); // sem interrupções: usamos polling
        outb(COM1 + REG_LCR, 0x80); // DLAB=1 para programar o divisor
        outb(COM1 + REG_DATA, 0x01); // divisor = 1 -> 115200 baud
        outb(COM1 + REG_IER, 0x00);
        outb(COM1 + REG_LCR, 0x03); // 8N1, DLAB=0
        outb(COM1 + REG_FCR, 0xC7); // FIFO ligado e limpo, gatilho de 14 bytes
        outb(COM1 + REG_MCR, 0x03); // DTR + RTS
    }
}

/// Envia um byte, aguardando o buffer de transmissão ficar livre.
pub fn putc(byte: u8) {
    unsafe {
        while inb(COM1 + REG_LSR) & LSR_THR_EMPTY == 0 {}
        outb(COM1 + REG_DATA, byte);
    }
}

/// Envia uma string inteira.
pub fn puts(texto: &str) {
    for byte in texto.bytes() {
        putc(byte);
    }
}
