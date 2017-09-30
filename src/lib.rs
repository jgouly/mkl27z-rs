#![no_std]
#![feature(lang_items)]

mod lang_items;

pub mod flashconfig;
pub mod vector_table;

#[macro_use]
mod memory_mapped_register;

mod sim;
mod mcg;
pub mod gpio;

mod usb;
pub mod lpuart0;

extern "C" {
  fn init_systick();
  fn msdelay(ms: u32);
}

pub fn init() {
  sim::disable_watchdog();
  copy_rom_to_ram();
  gpio::gate_gpio();
  mcg::init_clocks();
  unsafe { init_systick() }
  lpuart0::init();
  // TODO: Add a condition to wait on, rather than hardcoded delay.
  unsafe { msdelay(1) }
  usb::init();
}

fn copy_rom_to_ram() {
  unsafe {
    extern "C" {
      static _etext: u32;
      static mut _sdata: u32;
      static mut _edata: u32;
      static mut _sbss: u32;
      static mut _ebss: u32;
    }
    let mut src: *const u32 = &_etext;
    let mut dest: *mut u32 = &mut _sdata;
    while dest < &mut _edata as *mut u32 {
      *dest = *src;
      dest = dest.offset(1);
      src = src.offset(1);
    }
    let mut dest: *mut u32 = &mut _ebss;
    while dest >= &mut _sbss as *mut u32 {
      *dest = 0;
      dest = dest.offset(-1);
    }
  }
}

#[macro_export]
macro_rules! reset_fn {
  ($v: ident) => {
    #[link_section = ".reset"]
    #[no_mangle]
    pub static _RESET: fn() -> ! = $v;
  }
}
