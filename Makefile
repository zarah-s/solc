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
	cargo build --release && cp ./target/release/solc .

clean:
	cargo clean && rm -rf ./solc

test_:
	cargo test

copy:
	cp ./target/release/solc .

rm:
	rm -rf ./solc

run: 
	./solc Contract.sol

push:
	git add . && git commit -m $(msg) && git push && echo pushed to remote repository with $(msg) message