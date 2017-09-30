extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  // Pass our linker script to the top crate
  let script = "mkl27z.ld";
  let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    .join("src");
  let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

  fs::copy(src.join(script), out.join(script)).unwrap();
  println!("cargo:rustc-link-search={}", out.display());

  build_third_party();
}

#[cfg(feature = "usb")]
fn build_third_party() {
  let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    .join("src");
  let third_party_dir = src.join("third_party");
  let files = vec![
    "clz.c",
    "systick.c",
    "usb_dev.c",
    "usb_desc.c",
    "usb_mem.c",
    "usb_keyboard.c",
  ];
  let flags = vec![
    "-mcpu=cortex-m0plus",
    "-mthumb",
    "-fno-PIC",
    "-ffunction-sections",
    "-ffreestanding",
    "-fno-exceptions",
    "-O2", // -Os generates calls to __gnu_thumb1_case_u{q,h}i
    "-fdata-sections",
  ];
  let mut c = cc::Build::new();
  c.include(third_party_dir.clone())
    .define("__MKL26Z64__", None)
    .define("USB_KEYBOARDONLY", None)
    .define("F_CPU", Some("48000000"));
  for flag in flags {
    c.flag(flag);
  }
  for file in files {
    c.file(third_party_dir.join(file));
  }
  c.compile("libthird_party.a");
}

#[cfg(not(feature = "usb"))]
fn build_third_party() {}
