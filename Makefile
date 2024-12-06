CC=gcc
CFLAGS=-Wall -Wextra -pedantic -std=c11 -O0 -ggdb -no-pie
LIBS:=-lm

all: cachesim

run:
	cargo run trazas/blowfish.xex 1024 2 32

build:
	cargo build

debug: build
	rust-gdb --args target/debug/cachelab trazas/blowfish.xex 1024 2 32

debug_llbm: build
	rust-lldb target/debug/cachelab trazas/blowfish.xex 1024 2 32

cachesim:
	cargo build --release
	cp target/release/cachesim .

cmp_summary:
	diff 4096-4-16_summary.txt output/blowfish_4096-4-16_summary.txt
	diff 2048-2-64_summary.txt output/adpcm_2048-2-64_summary.txt
	diff 1024-4-32_summary.txt output/fft_1024-4-32_summary.txt

cmp_txt:
	diff 1024-4-32.txt output/fft_1024-4-32.txt
	diff 2048-2-64.txt output/adpcm_2048-2-64.txt
	diff 4096-1-256.txt output/blowfish_4096-1-256.txt

# Nota: "cachesim" es phony solo para Rust.
.PHONY: cachesim
