build:
	cargo build --target wasm32-unknown-unknown --release

deploy:
	near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId $(ACCOUNT_ID)