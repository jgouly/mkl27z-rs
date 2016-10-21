#![no_std]
#![feature(lang_items)]

mod lang_items;

pub mod flashconfig;
pub mod vector_table;

#[macro_use]
mod memory_mapped_register;

mod sim;
pub mod gpio;

pub fn init() {
  sim::disable_watchdog();
  gpio::gate_gpio();
}

#[macro_export]
macro_rules! reset_fn {
  ($v: ident) => {
    #[link_section = ".reset"]
    #[no_mangle]
    pub static _RESET: fn() -> ! = $v;
  }
}
