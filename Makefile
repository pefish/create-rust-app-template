

DEFAULT: build-release

build-release:
	cargo +nightly build --release

build:
	cargo +nightly build

publish:
	cargo +nightly publish

run:
	RUST_LOG=debug RUST_BACKTRACE=full RUST_CONFIG=./config/sample.toml cargo +nightly run

run-bin:
	RUST_LOG=debug RUST_BACKTRACE=full RUST_CONFIG=./config/sample.toml cargo +nightly run --bin $(bin)

install: build-release
	sudo install -C ./target/release/lotus-window-client /usr/local/bin/lotus-window-client-rs

install-service: install
	sudo mkdir -p /etc/systemd/system
	sudo install -C -m 0644 ./script/lotus-window-client-rs.service /etc/systemd/system/lotus-window-client-rs.service
	sudo systemctl daemon-reload
	@echo
	@echo "lotus-window-client service installed."
