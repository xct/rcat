# rcat

Simple rust reverse shell, similar to nc. Binaries are around 4 MB in size.


## compile

rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu

Windows:
cargo build --release --target x86_64-pc-windows-gnu

Linux:
cargo build --release