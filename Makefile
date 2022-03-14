
run:
	cargo run -- --dev -lruntime=debug 

purge: 
	target/release/liberland_node purge-chain --dev -y

build:
	sh scripts/inits.sh
	cargo build --release

check:
	cargo check --release

test:
	cargo test --all --all-features

linux_install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

linux_light:
	wget http://get.testnet.liberland.org/liberland_node
	rm -rf data/
	chmod +x liberland_node
	bash scripts/run_remote_val.sh 

update:
	git pull
	cargo update

#ChainSpec
build_spec:
	./target/release/liberland_node build-spec --disable-default-bootnode --chain staging > scripts/chainspec.json


gen_test_key:
	subkey inspect --scheme ed25519 "fire penalty pony chase gift loan grid mule tape wrestle stuff salute" > node.key


run_test_validator:
	build
	gen_test_key
	./target/release/liberland_node --chain scripts/chainspec.json -d data/validator3 --name validator3 --validator --port 30335 --ws-port 9947 --rpc-port 9935 --ws-external --rpc-cors all --rpc-methods=unsafe --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM                                                                       


