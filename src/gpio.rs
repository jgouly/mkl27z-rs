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

pub struct OutputPin {
  pub port: MMReg<u32>,
  pub pddr: MMReg<u32>,
  pub psor: MMReg<u32>,
  pub pcor: MMReg<u32>,
  pub num: u32,
}

impl OutputPin {
  pub fn init(&self) {
    self.port.write(self.port.read() | (1 << 8));
    self.pddr.write(self.pddr.read() | (1 << self.num));
  }
  pub fn high(&self) {
    self.psor.write(self.psor.read() | (1 << self.num));
  }
  pub fn low(&self) {
    self.pcor.write(self.pcor.read() | (1 << self.num));
  }
}

// GPIO - Port B
mmaddr!(GPIOB_PDDR, u32, 0x400FF054);
mmaddr!(GPIOB_PSOR, u32, 0x400FF044);
mmaddr!(GPIOB_PCOR, u32, 0x400FF048);

mmaddr!(PORTB_PCR18, u32, 0x4004A048);
