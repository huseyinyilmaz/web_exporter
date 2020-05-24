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

# FROM rust:latest as cargo-build

# RUN apt-get update

# RUN apt-get install musl-tools build-essential libssl-dev pkg-config -y

# ENV OPENSSL_DIR=/usr/lib/x86_64-linux-gnu

# RUN rustup target add x86_64-unknown-linux-musl

# WORKDIR /usr/src/prometheus_web_exporter

# COPY Cargo.toml Cargo.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# ENV PKG_CONFIG_ALLOW_CROSS=1
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/prometheus_web_exporter*

# COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

# ARG OPENSSL_VERSION=1.0.2s

RUN apt-get update

# RUN apt-get install sudo musl-dev musl-tools libssl-dev openssl ca-certificates git libc6-dev gcc -y
RUN apt-get install musl-dev musl-tools -y

# RUN rustup target add x86_64-unknown-linux-musl

# ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig/

# RUN echo "Building OpenSSL" && \
#     ls /usr/include/linux && \
#     sudo mkdir -p /usr/local/musl/include && \
#     sudo ln -s /usr/include/linux /usr/local/musl/include/linux && \
#     sudo ln -s /usr/include/x86_64-linux-gnu/asm /usr/local/musl/include/asm && \
#     sudo ln -s /usr/include/asm-generic /usr/local/musl/include/asm-generic && \
#     cd /tmp && \
#     curl -LO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
#     tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
#     env CC=musl-gcc ./Configure no-shared no-zlib -fPIC --prefix=/usr/local/musl -DOPENSSL_NO_SECURE_MEMORY linux-x86_64 && \
#     env C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
#     env C_INCLUDE_PATH=/usr/local/musl/include/ make && \
#     sudo make install && \
#     sudo rm /usr/local/musl/include/linux /usr/local/musl/include/asm /usr/local/musl/include/asm-generic && \
#     rm -r /tmp/*

# ENV OPENSSL_DIR=/usr/local/musl/ssl \
#     PKG_CONFIG_PATH=/usr/local/musl/lib/pkgconfig \
#     OPENSSL_LIB_DIR=/usr/local/musl/lib \
#     OPENSSL_INCLUDE_DIR=/usr/local/musl/include

# WORKDIR /usr/src/prometheus_web_exporter
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

ENV WEB_EXPORTER_LOG_LEVEL=info

RUN addgroup -g 1000 prometheus_web_exporter

RUN adduser -D -s /bin/sh -u 1000 -G prometheus_web_exporter prometheus_web_exporter

WORKDIR /home/prometheus_web_exporter/bin/

COPY --from=cargo-build /usr/src/prometheus_web_exporter/target/x86_64-unknown-linux-musl/release/prometheus_web_exporter .

RUN chown prometheus_web_exporter:prometheus_web_exporter prometheus_web_exporter

USER prometheus_web_exporter

CMD ["./prometheus_web_exporter"]
