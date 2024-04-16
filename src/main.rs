#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod boot {
  use core::arch::global_asm;

  global_asm!(".section .text._start");
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    
  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop{}
}
