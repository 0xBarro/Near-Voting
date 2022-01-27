build:
	cargo build --target wasm32-unknown-unknown --release

deploy:
	near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId $(ACCOUNT_ID)

get_sandbox:
	git clone https://github.com/near/nearcore && cd nearcore && make sandbox

sandbox: 
	nearcore/target/debug/neard-sandbox --home /tmp/near-sandbox init
	nearcore/target/debug/neard-sandbox --home /tmp/near-sandbox run

clean_sandbox: 
	rm -rf nearcore/target/temp/near-sandbox

