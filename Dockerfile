FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-dev musl-tools -y
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/prometheus_web_exporter
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

ENV WEB_EXPORTER_LOG_LEVEL=warn

WORKDIR /usr/local/prometheus_web_exporter

COPY --from=cargo-build /usr/src/prometheus_web_exporter/target/x86_64-unknown-linux-musl/release/prometheus_web_exporter .

CMD ["/usr/local/prometheus_web_exporter/prometheus_web_exporter"]
