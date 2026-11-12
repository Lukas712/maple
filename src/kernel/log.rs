//! Log formatado do kernel, enviado à serial.
//!
//! Use `kprint!` e `kprintln!` com a sintaxe de `format_args!`
//! (ex.: `kprintln!("{:#018x}", endereco)`).

use core::fmt::{self, Write};

use super::serial;

struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial::puts(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let _ = SerialWriter.write_fmt(args);
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::kernel::log::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n")
    };
    ($($arg:tt)*) => {
        $crate::kprint!("{}\n", format_args!($($arg)*))
    };
}
