# Makefile

TARGET = compiler
SRC = src/main.rs

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(SRC)
	rustc $< -o $@

compile:
	cargo run Contract.sol

build: 
	cargo build --release

clean:
	cargo clean

copy:
	cp ./target/release/compiler .

rm:
	rm -rf ./compiler
