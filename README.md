# rusty-donut
ASCII raymarching inside a terminal

![donut gif](https://s3.gifyu.com/images/donut97dd57264f76666d.gif)

## Build instructions

1. Requirements

  * [rust]
  * gcc
  * git

[rust]: https://github.com/rust-lang/rust

2. Clone source

```sh
git clone https://github.com/drip-drip/rusty-donut
cd rusty-donut
```

3. Build with cargo

```sh
cargo build --release
```

4. Run

The binary is located at (`donut.exe` on Windows)
```sh
target\release\donut
```

## Options

```
Usage: donut [options]
    options:
        -h, --help  shows this help message
        --sd        use 10 character charset (default 70)
        --inline    do not clear terminal on start (Windows only)
    
    options (sizes, default is normal size):
        --tiny      shows tiny donut
        --small     shows small donut
        --big       shows big donut
        --huge      shows huge donut
```
