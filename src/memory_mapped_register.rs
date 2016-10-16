use core::ptr::{read_volatile, write_volatile};

macro_rules! mmaddr {
  ($v: ident, $ty: ty, $x: expr) => { pub const $v: *mut $ty = ($x) as *mut $ty; };
}

pub struct MemoryMappedRegister<T> {
  addr: *mut T,
}

impl<T> MemoryMappedRegister<T> {
  pub fn new(addr: *mut T) -> MemoryMappedRegister<T> {
    MemoryMappedRegister { addr: addr }
  }
  pub fn read(&self) -> T {
    unsafe { read_volatile(self.addr) }
  }
  pub fn write(&self, val: T) {
    unsafe {
      write_volatile(self.addr, val);
    }
  }
}

pub type MMReg<T> = MemoryMappedRegister<T>;

impl<T> From<*mut T> for MemoryMappedRegister<T> {
  fn from(val: *mut T) -> MemoryMappedRegister<T> {
    MemoryMappedRegister::new(val)
  }
}
