#![no_std]
#![no_main]

use uefi::{
    entry,
    table::{Boot, SystemTable},
    Handle, Status,
};

#[entry]
fn main(_handle: Handle, _system_table: SystemTable<Boot>) -> Status {
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
