VERSION = $(shell cargo pkgid | sed 's!.*\:!!')
version:
	@echo "$(VERSION)"
run:
	WEB_EXPORTER_LOG_LEVEL=warn cargo run
# before releasing update version in cargo.toml file and run prerelease.
prerelease:
	cargo generate-lockfile
	cargo build --release --locked
	cargo test
	cargo check
	cargo fmt
	cargo clippy -- -D warnings
release: prerelease
	cargo publish
	git tag v$(VERSION);
	git push origin --tags
