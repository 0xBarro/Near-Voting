build:
	cargo build --target wasm32-unknown-unknown --release

deploy:
	near deploy --wasmFile target/wasm32-unknown-unknown/release/voting.wasm --accountId $(ACCOUNT_ID)

dev-deploy:
	near dev-deploy target/wasm32-unknown-unknown/release/voting.wasm