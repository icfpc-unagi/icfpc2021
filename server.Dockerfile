FROM rust:1.53 AS rust-builder
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p /work/src
WORKDIR /work
COPY Cargo.lock /work/Cargo.lock
COPY Cargo.toml /work/Cargo.toml
RUN touch ./src/lib.rs && cargo vendor && cargo build --release
COPY ./src /work/src
RUN touch ./src/lib.rs && cargo vendor && cargo build --release

FROM golang:1.16.5 AS go-builder
RUN mkdir -p /work
WORKDIR /work
COPY ./go/go.mod ./go/go.sum /work/
RUN go mod download
COPY ./go /work/go
RUN cd go && go build -o /usr/local/bin/server ./cmd/server

FROM ubuntu:20.04
ARG UNAGI_PASSWORD
RUN [ "${UNAGI_PASSWORD}" != "" ]
ENV UNAGI_PASSWORD "${UNAGI_PASSWORD}"
RUN apt-get update -q && apt-get install -qy openssl ca-certificates
COPY --from=rust-builder /work/target/release/calculate_score /usr/local/bin/calculate_score
COPY --from=rust-builder /work/target/release/evaluate /usr/local/bin/evaluate
COPY --from=go-builder /usr/local/bin/server /usr/local/bin/server
COPY ./problems /problems
COPY ./problems /static/problems
COPY ./web/dist /static/show
COPY ./gui/dist /static/gui
COPY ./scripts/server.sh /usr/local/bin/server.sh
RUN chmod +x /usr/local/bin/server.sh
ENTRYPOINT server.sh
