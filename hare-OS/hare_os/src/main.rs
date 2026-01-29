//Desabilitar a blioteca externa
#![no_std]

use core::panic::PanicInfo;

fn main() {
use::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) !{
    loop{}
}  
}
