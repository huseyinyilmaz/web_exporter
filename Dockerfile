# FROM ekidd/rust-musl-builder as build
# ADD . /home/rust/src
# WORKDIR /home/rust/src
# RUN cargo build --release

# FROM scratch
# COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/prometheus_web_exporter /
# EXPOSE 3000

# CMD ["/prometheus_web_exporter"]

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools build-essential libssl-dev openssl pkg-config -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/prometheus_web_exporter

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/prometheus_web_exporter*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 prometheus_web_exporter

RUN adduser -D -s /bin/sh -u 1000 -G prometheus_web_exporter prometheus_web_exporter

WORKDIR /home/prometheus_web_exporter/bin/

COPY --from=cargo-build /usr/src/prometheus_web_exporter/target/x86_64-unknown-linux-musl/release/prometheus_web_exporter .

RUN chown prometheus_web_exporter:prometheus_web_exporter prometheus_web_exporter

USER prometheus_web_exporter

CMD ["./prometheus_web_exporter"]
