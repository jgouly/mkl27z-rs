#![no_std]

#[macro_use]
mod memory_mapped_register;

mod sim;
mod gpio;

pub fn init() {
  sim::disable_watchdog();
  gpio::gate_gpio();
}
