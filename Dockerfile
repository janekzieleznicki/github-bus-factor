FROM rust:slim-buster as builder-base
WORKDIR /usr/src/myapp
COPY . .
RUN rustup component add rustfmt

FROM builder-base as builder
RUN apt update && \
            apt install -y libssl-dev pkg-config
RUN cargo install --path .

FROM debian:buster-slim

RUN apt update && \
      apt install --no-install-recommends -y openssl ca-certificates && \
      update-ca-certificates && \
      rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/busfactor /usr/local/bin/busfactor

ENTRYPOINT ["/usr/local/bin/busfactor"]