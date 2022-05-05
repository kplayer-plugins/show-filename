.PHONY: clean
OBJS= src/lib.rs

build: $(OBJS)
	cargo build --target wasm32-wasi --release
	cp -f target/wasm32-wasi/release/show_filename.wasm target/wasm32-wasi/release/show-filename.kpe

clean:
	rm -rf target