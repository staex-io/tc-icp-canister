fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

test:
	cargo test tests::unit_test --jobs 1 -- --nocapture --test-threads 1

build:
	cargo build --release --target wasm32-unknown-unknown --package tc-icp
	candid-extractor target/wasm32-unknown-unknown/release/tc_icp.wasm > tc-icp.did
	dfx canister create tc-icp --ic --identity staex-prod
	dfx canister install --ic --identity staex-prod --async-call -y tc-icp --mode upgrade --wasm target/wasm32-unknown-unknown/release/tc_icp.wasm

update_declarations:
	rm -rf ../../tc-ui/src/assets/declarations
	dfx generate tc-icp
	mv src/declarations ../../tc-ui/src/assets
