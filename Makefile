run:
	WEB_EXPORTER_LOG_LEVEL=info cargo run
# before releasing update version in cargo.toml file and run prerelease.
prerelease:
	cargo generate-lockfile
	cargo build --release --locked
	cargo test
