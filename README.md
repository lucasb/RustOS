# RustOS


build with linker
``` sh
$ cargo rustc -- -C link-arg=--script=./linker.ld
```

check binary
```sh
$ objdump -D target/aarch64-unknown-none/debug/rustos | less
```

generate  image
``` sh
$ objcopy -O binary target/aarch64-unknown-none/debug/rustos ./rustoskernel.img
```

run on qemu

``` sh
$ qemu-system-aarch64 -M virt -m 1024 -drive format=raw,file=rustoskernel.img
```


setting to delete
config-settings:

    "rust-analyzer.checkOnSave.overrideCommand": [
        "./cargo",
        "xcheck",
        "--json-output"
    ]
}
{
    "rust-analyzer.cargo.buildScripts.rebuildOnSave": true
}
