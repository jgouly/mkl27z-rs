const MCG_MC_HIRCEN: u8 = 1 << 7;
const MCG_C1_CLKS_HIRC: u8 = 3 << 6;
const MCG_S_CLKST: u8 = 3 << 2;
const IRCLKEN: u8 = 1 << 1;

pub fn init_clocks() {
  let mcg_mc: ::memory_mapped_register::MMReg<u8> = (0x40064018 as *mut u8)
    .into();
  mcg_mc.write(mcg_mc.read() | MCG_MC_HIRCEN);
  let mcg_c1: ::memory_mapped_register::MMReg<u8> = (0x40064000 as *mut u8)
    .into();
  mcg_c1.write(mcg_c1.read() & !MCG_C1_CLKS_HIRC);
  let mcg_s: ::memory_mapped_register::MMReg<u8> = (0x40064006 as *mut u8)
    .into();
  while (mcg_s.read() & MCG_S_CLKST) != 0 {}
  mcg_c1.write(mcg_c1.read() | IRCLKEN);
}
