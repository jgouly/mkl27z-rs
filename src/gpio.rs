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

const PORT_PCR_MUX1: u32 = 1 << 8;
const PORT_PCR_PS: u32 = 0x1;
const PORT_PCR_PE: u32 = 0x2;

pub struct OutputPin {
  pub port: MMReg<u32>,
  pub pddr: MMReg<u32>,
  pub psor: MMReg<u32>,
  pub pcor: MMReg<u32>,
  pub num: u32,
}

impl OutputPin {
  pub fn init(&self) {
    self.port.write(self.port.read() | PORT_PCR_MUX1);
    self.pddr.write(self.pddr.read() | (1 << self.num));
  }
  pub fn high(&self) {
    self.psor.write(self.psor.read() | (1 << self.num));
  }
  pub fn low(&self) {
    self.pcor.write(self.pcor.read() | (1 << self.num));
  }
}

pub struct InputPin {
  pub port: MMReg<u32>,
  pub pddr: MMReg<u32>,
  pub pdir: MMReg<u32>,
  pub num: u32,
}

impl InputPin {
  pub fn init(&self) {
    self.port.write(self.port.read() | PORT_PCR_MUX1 | PORT_PCR_PE);
    self.port.write(self.port.read() & !PORT_PCR_PS);
    self.pddr.write(self.pddr.read() & !(1 << self.num));
  }
  pub fn read(&self) -> u32 {
    if (self.pdir.read() & (1 << self.num)) != 0 {
      1
    } else {
      0
    }
  }
}

// GPIO - Port B
mmaddr!(GPIOB_PDDR, u32, 0x400FF054);
mmaddr!(GPIOB_PSOR, u32, 0x400FF044);
mmaddr!(GPIOB_PCOR, u32, 0x400FF048);

mmaddr!(PORTB_PCR18, u32, 0x4004A048);
mmaddr!(PORTB_PCR19, u32, 0x4004A04C);
