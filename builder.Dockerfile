FROM rust:1.53
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p /work/src
WORKDIR /work
COPY Cargo.lock /work/Cargo.lock
COPY Cargo.toml /work/Cargo.toml
RUN touch ./src/lib.rs && cargo vendor && cargo build --release
