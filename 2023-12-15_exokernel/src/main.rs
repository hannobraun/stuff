#![no_std]
#![no_main]

use uefi::{
    entry,
    table::{Boot, SystemTable},
    Handle, Status,
};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    Status::SUCCESS
}
