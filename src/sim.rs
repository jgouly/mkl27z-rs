use memory_mapped_register::MMReg;

mmaddr!(SIM_COPC, u32, 0x40048100);

pub const SIM_SCGC5_LPUART0: u32 = 1 << 20;
pub const SIM_SOPT2_LPUART0SRC_IRC48M_0: u32 = 1 << 26;
pub const SIM_SOPT2_LPUART0SRC_IRC48M_1: u32 = 1 << 27;

pub fn disable_watchdog() {
  MMReg::new(SIM_COPC).write(0);
}
