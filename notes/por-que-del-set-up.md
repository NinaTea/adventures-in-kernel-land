

## Curiosidades

### En .cargo/config.toml

Con 

```rust
[build]
target = "riscv32imac-unknown-none-elf"
```
Le estamos diciendo para quien queremos compilar.



Con 

```rust
[target.riscv32imac-unknown-none-elf]
runner = """ qemu-system-riscv32
  -cpu rv32
  -machine virt
  -m 150M
  -s
  -nographic
  -bios """
  ```
Le estamos diciendo qué máquina estamos usando.
Dato, si no especificamos el runner, rust tira segmetation fault.

### En src/main.rs

Tenemos que usar el atributo no_std porque el kernel
no tiene sistema operativo, claramente, entonces
su compilacion no puede usar la libreria estandar de rust (porque depende del so xd)
```rust
#![no_std]
#![no_main]
```
Tenemos que definir un panic handler para cuando algo salga mal(esto viene en la std).

Además, exactamente `main` no es el punto de entrada de los programas, es `_start` o alguna otra variación que depende del compilador.

En este caso, con `#![no_main]` estamos diciendo que no use el main de la std, sino que use el nuestro. Con `#[no_mangle]` estamos diciendo que no cambie el nombre de la función y que ese va a ser el punto de entrada del programa.


```rust
use core::panic::PanicInfo;

// error: using `fn main` requires the standard library
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
```