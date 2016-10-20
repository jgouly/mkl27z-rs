#![no_std]

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
