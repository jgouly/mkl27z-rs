use usb;

fn default_handler() {
  loop {}
}

const _NUM_INT: usize = 48;
type VectorTable = [Option<fn()>; _NUM_INT];

#[link_section = ".exceptions"]
#[no_mangle]
pub static _EXCEPTIONS: VectorTable = [Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(usb::usb_isr_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler),
                                       Some(default_handler)];
