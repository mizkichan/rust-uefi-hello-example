#![no_std]
#![no_main]

use core::fmt::Write as _;
use uefi::prelude::*;
use uefi::Handle;

#[no_mangle]
pub extern "C" fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).unwrap().unwrap();
    writeln!(system_table.stdout(), "Hello, world!").unwrap();
    Status::SUCCESS
}
