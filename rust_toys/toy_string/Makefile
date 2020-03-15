ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libqsplit.$(EXT)
	g++ src/main.cc -L ./target/debug/ -lqsplit -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/libqsplit.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run