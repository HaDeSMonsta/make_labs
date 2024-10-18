target_linux := "x86_64-unknown-linux-musl"
target_win := "x86_64-pc-windows-gnu"
target_mac_x86 := "x86_64-apple-darwin"
target_mac_arm := "aarch64-apple-darwin"

default:
	@just --list

release:
	cargo build --release
	cross build --release --target {{target_linux}}
	@#cross build --release --target {{target_win}}
	@#cross build --release --target {{target_mac_x86}}
	@#cross build --release --target {{target_mac_arm}}
