#[cfg(feature = "usb")]
mod imp {
  extern "C" {
    fn usb_init();
    fn usb_isr();
  }

  pub fn init() {
    unsafe {
      usb_init();
    }
  }

  pub fn usb_isr_handler() { unsafe { usb_isr(); } }
}

#[cfg(not(feature = "usb"))]
mod imp {
  pub fn init() {}
  pub fn usb_isr_handler() { loop {} }
}

pub use self::imp::*;
