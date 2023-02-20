#![feature(downcast_unchecked)]
#![no_std]
#[macro_use]
extern crate alloc;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}


// enable one of the `mod` lines, and run with:
// `cargo build --target wasm32-unknown-unknown --release`
// then check the size of ./target/wasm32-unknown-unknown/release/file_bloat.wasm

// pub mod generic;    // 27976 bytes
// pub mod dyn_wrapper; // 29622 bytes

