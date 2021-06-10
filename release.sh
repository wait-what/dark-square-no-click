cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/dark-square-no-click bin/dark_square_no_click_1.0.1_linux64
cp target/x86_64-pc-windows-gnu/release/dark-square-no-click.exe bin/dark_square_no_click_1.0.1_windows64.exe

strip bin/*
upx --best --lzma bin/*
