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

// pub mod generic;        // 7872 bytes
// pub mod dyn_wrapper;    // 9225 bytes
// pub mod dyn_explicit;   // 9637 bytes
