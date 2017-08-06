#include "usb_desc.h"
#include "kinetis.h"

#define MCG_MC (*(volatile uint8_t *)0x40064018)
#define MCG_MC_HIRCEN (1 << 7)
#define MCG_C1_CLKS_HIRC (3 << 6)
#define MCG_S (*(volatile uint8_t *)0x40064006)
#define MCG_S_CLKST (3 << 2)
#define MCG_C1 (*(volatile uint8_t *)0x40064000)
#define IRCLKEN (1 << 1)

void init_clocks() {
  MCG_MC |= MCG_MC_HIRCEN;
  MCG_C1 &= ~MCG_C1_CLKS_HIRC;
  while ((MCG_S & MCG_S_CLKST) != 0)
    ;
  MCG_C1 |= IRCLKEN;
}

volatile uint32_t systick_millis_count = 0;
void systick_isr(void) { systick_millis_count++; }

uint32_t micros(void) {
  __disable_irq();
  uint32_t current = SYST_CVR;
  uint32_t count = systick_millis_count;
  uint32_t istatus = SCB_ICSR; // bit 26 indicates if systick exception pending
  __enable_irq();
  if ((istatus & SCB_ICSR_PENDSTSET) && current > 50)
    count++;
  current = ((F_CPU / 1000) - 1) - current;
  return count * 1000 + ((current * (uint32_t)87381) >> 22);
}

void msdelay(uint32_t ms) {
  uint32_t start = micros();

  if (ms > 0) {
    while (1) {
      while ((micros() - start) >= 1000) {
        ms--;
        if (ms == 0)
          return;
        start += 1000;
      }
    }
  }
}

void init_systick() {
  SYST_RVR = (F_CPU / 1000) - 1;
  SYST_CVR = 0;
  SYST_CSR = SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT | SYST_CSR_ENABLE;
  SCB_SHPR3 = 0x20200000; // Systick = priority 32
}
