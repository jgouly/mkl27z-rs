const LPUART0_CTRL_TE: u32 = 1 << 19;

pub fn init() {
  let sim_scgc5 = ::memory_mapped_register::MMReg::new(::gpio::SIM_SCGC5);
  sim_scgc5.write(sim_scgc5.read() | ::sim::SIM_SCGC5_LPUART0);

  let porta_pcr2 =
    ::memory_mapped_register::MMReg::<u32>::new(::gpio::PORTA_PCR2);
  porta_pcr2.write(
    ::gpio::PORT_PCR_PE | ::gpio::PORT_PCR_PS | ::gpio::PORT_PCR_PFE |
      ::gpio::port_pcr_mux(2),
  );

  let sim_sopt2: ::memory_mapped_register::MMReg<u32> =
    (0x40048004 as *mut u32).into();
  sim_sopt2.write(
    (sim_sopt2.read() | ::sim::SIM_SOPT2_LPUART0SRC_IRC48M_0) &
      !::sim::SIM_SOPT2_LPUART0SRC_IRC48M_1,
  );

  let lpuart0_baud: ::memory_mapped_register::MMReg<u32> =
    (0x40054000 as *mut u32).into();
  lpuart0_baud.write(lpuart0_baud.read() | 313);

  let lpuart0_ctrl: ::memory_mapped_register::MMReg<u32> =
    (0x40054008 as *mut u32).into();
  lpuart0_ctrl.write(lpuart0_ctrl.read() | LPUART0_CTRL_TE);
}

fn putc(c: u8) {
  let lpuart0_stat: ::memory_mapped_register::MMReg<u32> =
    (0x40054004 as *mut u32).into();
  while (lpuart0_stat.read() & (1 << 22)) == 0 {}
  while (lpuart0_stat.read() & (1 << 23)) == 0 {}
  let lpuart0_data: ::memory_mapped_register::MMReg<u32> =
    (0x4005400C as *mut u32).into();
  lpuart0_data.write(lpuart0_data.read() | c as u32);
}

#[no_mangle]
pub extern "C" fn lpuart0_putc(c: u8) {
  putc(c)
}
