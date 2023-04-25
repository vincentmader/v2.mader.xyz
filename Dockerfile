FROM alpine:3.14

RUN apk add curl gcc make musl-dev ca-certificates

# Install Nightly Rust Toolchain
RUN curl --proto '=https' \
         --tlsv1.2 -sSf https://sh.rustup.rs | \
         sh -s -- --default-toolchain nightly -y
ENV PATH=$PATH:/root/.cargo/bin

# Install wasm-bindgen-cli & Add WASM32 Rust Target
RUN source /root/.cargo/env; \
    cargo install -f wasm-bindgen-cli; \
    rustup target add wasm32-unknown-unknown; 

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY ./ ./

ENTRYPOINT ./entrypoint.sh
