## Setting up

- Rust
- Qemu
- gdb-multiarch 


### Rust

Para instalar rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Vamos a usar rust nightly para el kernel.
Hay que setearla como la toolchain default.
En la carpeta del proyecto:

```bash
rustup override set nightly
```
### Qemu

```bash
sudo apt install qemu-system-riscv32
```

### gdb-multiarch

Para instalar gdb-multiarch

```bash
sudo apt install gdb-multiarch
```

Para debuggear ejecutables de otra arquitecura, hay que usar `gdb-multiarch` en vez de `gdb`.

Para debuggear hay que abrir otra terminal después de correr qemu y ejecutar:

```bash
gdb-multiarch 
```

```bash
file target/riscv64imac-unknown-none-elf/debug/kernel
```

```bash
target remote localhost:1234
```
