use memory_mapped_register::MMReg;

mmaddr!(SIM_SCGC5, u32, 0x40048038);
const SIM_SCGC5_PORTA: u32 = 1 << 9;
const SIM_SCGC5_PORTB: u32 = 1 << 10;
const SIM_SCGC5_PORTC: u32 = 1 << 11;
const SIM_SCGC5_PORTD: u32 = 1 << 12;
const SIM_SCGC5_PORTE: u32 = 1 << 13;

pub fn gate_gpio() {
  let sim_scgc5 = MMReg::new(SIM_SCGC5);
  sim_scgc5.write(SIM_SCGC5_PORTA | SIM_SCGC5_PORTB | SIM_SCGC5_PORTC |
                  SIM_SCGC5_PORTD | SIM_SCGC5_PORTE);
}