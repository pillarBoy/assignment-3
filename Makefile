run-tmp:
	SKIP_WASM_BUILD= cargo +nightly-2020-10-05 run -- --dev --tmp -lruntime=debug

run:
	SKIP_WASM_BUILD= cargo run -- --dev -lruntime=debug

toolchain:
	./scripts/init.sh

build-full:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release

build-release:
	cargo +nightly-2020-10-05 build --release
	
check:
	cargo +nightly-2020-10-05 check -p pallet-kitties

build:
	SKIP_WASM_BUILD= cargo build

test:
	SKIP_WASM_BUILD= cargo test --all

purge:
	SKIP_WASM_BUILD= cargo run -- purge-chain --dev -y

restart: purge run

init: toolchain build-full


playground-run:
	./target/release/node-template --dev --ws-external


playground-set-git:
	git config --global user.email "601991736@qq.com"
	git config --global user.name "pillarBoy"
