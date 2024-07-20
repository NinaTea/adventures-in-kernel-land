// Tenemos que usar el atributo no_std porque el kernel
// no tiene sistema operativo, claramente, entonces
// su compilacion no puede usar la libreria estandar de rust
// (porque depende del so xd)
#![no_std]
#![no_main]
// Tenemos que definir un panic handler
// para cuando algo salga mal(esto viene en la std)
use core::panic::PanicInfo;

// error: using `fn main` requires the standard library
//   |
//   = help: use `#![no_main]` to bypass the Rust generated
//   entrypoint and declare a platform specific entrypoint yourself, usually with `#[no_mangle]`
#[no_mangle]
fn _start() {
    loop {}
}

// ! significa que nunca returnea
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
