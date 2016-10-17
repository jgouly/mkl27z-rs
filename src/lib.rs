#![no_std]

#[macro_use]
mod memory_mapped_register;

mod sim;

pub fn init() {
  sim::disable_watchdog();
}
