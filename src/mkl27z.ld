MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
  /* FIXME: The origin and length are incorrect */
  RAM  (rwx) : ORIGIN = 0x1FFFF800, LENGTH = 8K
}

EXTERN(_EXCEPTIONS)
EXTERN(_FLASHCONFIG)

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
    KEEP(*(.exceptions))
    . = 0x400;
    KEEP(*(.flashconfig))
    *(.text*)
    *(.rodata*)
  } > FLASH = 0xFF

  _etext = .;

  .data : AT (_etext) {
    . = ALIGN(4);
    _sdata = .;
    *(.data*)
    . = ALIGN(4);
    _edata = .;
  } > RAM

  .bss : {
    . = ALIGN(4);
    _sbss = .;
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
