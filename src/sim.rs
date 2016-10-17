use memory_mapped_register::MMReg;

mmaddr!(SIM_COPC, u32, 0x40048100);

pub fn disable_watchdog() {
  MMReg::new(SIM_COPC).write(0);
}
