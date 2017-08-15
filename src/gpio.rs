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

// GPIO - Port A
mmaddr!(GPIOA_PDIR, u32, 0x400FF010);
mmaddr!(GPIOA_PDDR, u32, 0x400FF014);
mmaddr!(GPIOA_PSOR, u32, 0x400FF004);
mmaddr!(GPIOA_PCOR, u32, 0x400FF008);

mmaddr!(PORTA_PCR1, u32, 0x40049004);
mmaddr!(PORTA_PCR2, u32, 0x40049008);
mmaddr!(PORTA_PCR18, u32, 0x40049048);
mmaddr!(PORTA_PCR19, u32, 0x4004904C);

// GPIO - Port B
mmaddr!(GPIOB_PDIR, u32, 0x400FF050);
mmaddr!(GPIOB_PDDR, u32, 0x400FF054);
mmaddr!(GPIOB_PSOR, u32, 0x400FF044);
mmaddr!(GPIOB_PCOR, u32, 0x400FF048);

mmaddr!(PORTB_PCR0, u32, 0x4004A000);
mmaddr!(PORTB_PCR1, u32, 0x4004A004);
mmaddr!(PORTB_PCR2, u32, 0x4004A008);
mmaddr!(PORTB_PCR3, u32, 0x4004A00C);
mmaddr!(PORTB_PCR16, u32, 0x4004A040);
mmaddr!(PORTB_PCR17, u32, 0x4004A044);
mmaddr!(PORTB_PCR18, u32, 0x4004A048);
mmaddr!(PORTB_PCR19, u32, 0x4004A04C);

// GPIO - Port C
mmaddr!(GPIOC_PDIR, u32, 0x400FF090);
mmaddr!(GPIOC_PDDR, u32, 0x400FF094);
mmaddr!(GPIOC_PSOR, u32, 0x400FF084);
mmaddr!(GPIOC_PCOR, u32, 0x400FF088);

mmaddr!(PORTC_PCR0, u32, 0x4004B000);
mmaddr!(PORTC_PCR1, u32, 0x4004B004);
mmaddr!(PORTC_PCR2, u32, 0x4004B008);
mmaddr!(PORTC_PCR4, u32, 0x4004B010);
mmaddr!(PORTC_PCR5, u32, 0x4004B014);
mmaddr!(PORTC_PCR6, u32, 0x4004B018);
mmaddr!(PORTC_PCR7, u32, 0x4004B01C);
mmaddr!(PORTC_PCR9, u32, 0x4004B024);

// GPIO - Port D
mmaddr!(GPIOD_PDDR, u32, 0x400FF0D4);
mmaddr!(GPIOD_PSOR, u32, 0x400FF0C4);
mmaddr!(GPIOD_PCOR, u32, 0x400FF0C8);

mmaddr!(PORTD_PCR0, u32, 0x4004C000);
mmaddr!(PORTD_PCR1, u32, 0x4004C004);
mmaddr!(PORTD_PCR2, u32, 0x4004C008);
mmaddr!(PORTD_PCR3, u32, 0x4004C00C);
mmaddr!(PORTD_PCR4, u32, 0x4004C010);
mmaddr!(PORTD_PCR5, u32, 0x4004C014);
mmaddr!(PORTD_PCR6, u32, 0x4004C018);
mmaddr!(PORTD_PCR7, u32, 0x4004C01C);

// GPIO - Port E
mmaddr!(GPIOE_PDDR, u32, 0x400FF114);
mmaddr!(GPIOE_PDIR, u32, 0x400FF110);
mmaddr!(GPIOE_PCOR, u32, 0x400FF108);
mmaddr!(GPIOE_PSOR, u32, 0x400FF104);

mmaddr!(PORTE_PCR23, u32, 0x4004D05C);
mmaddr!(PORTE_PCR24, u32, 0x4004D060);
mmaddr!(PORTE_PCR25, u32, 0x4004D064);
mmaddr!(PORTE_PCR29, u32, 0x4004D074);
mmaddr!(PORTE_PCR30, u32, 0x4004D078);
mmaddr!(PORTE_PCR31, u32, 0x4004D07C);
