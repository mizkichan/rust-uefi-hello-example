# Hello, world! on UEFI with Rust

## How to build and run

On shell:

```console
$ cargo insatll cargo-xbuild
$ git clone https://github.com/mizkichan/rust-uefi-hello-example.git
$ cd rust-uefi-hello-example
$ cargo xbuild --target x86_64-unknown-uefi
$ qemu-system-x86_64 -boot menu=on -bios /path/to/OVMF_CODE.fd -hda fat:rw:target/x86_64-unknown-uefi/debug
```

On qemu:

1. Press Esc key to open boot config screen while TianoCore logo is shown.
2. Select "Boot Manager" → "EFI Internal Shell" to launch UEFI Shell.
3. Enter `fs0:rust-uefi-hello.efi`.
