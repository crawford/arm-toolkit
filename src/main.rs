#![feature(asm, lang_items, start)]
#![no_std]
#![no_main]

extern crate volatile_register;

mod arch;

use arch::efm32lg990f256;

#[no_mangle]
pub fn main() -> ! {
    //efm32lg990f256::gpio().lock.read_bits();
    unsafe { asm!("BKPT 7") };
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {
    loop {}
}

mod isr_vector {
    #[link_section = ".isr_vector"]
    static RESET: fn() -> ! = ::main;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        efm32lg990f256::gpio().lock.read_bits();
    }
}
