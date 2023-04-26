FROM rust:1.67-alpine

# Install dependencies.
RUN apk add musl-dev
RUN apk add sqlite

# Define server details.
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0

# Setup working directory & initialize binary crate.
WORKDIR /var/www/
RUN USER=root cargo new --bin src
WORKDIR /var/www/src

# Pre-compile dependencies.
COPY ./src/Cargo.lock ./src/Cargo.lock
COPY ./src/Cargo.toml ./src/Cargo.toml
RUN cargo build --release;\
    rm ./src/*.rs;\
    rm ./target/release/deps/src*

# Configure Rust runtime.
ENV RUSTFLAGS="-C target-feature=-crt-static"
COPY ./rust-toolchain.toml ./rust-toolchain.toml

# Compile server.
COPY ./src ./src
RUN cd src && cargo build --release

# Compile client.
RUN apk add pkgconfig  # TODO Move `apk add` & `cargo install` to top.
RUN apk add openssl 
RUN apk add openssl-dev
RUN cargo install -f wasm-bindgen-cli
COPY ./bin/build_client ./bin/build_client
RUN cd bin && ./build_client --release

# Start server.
COPY ./entrypoint.sh ./entrypoint.sh
ENTRYPOINT ./entrypoint.sh
