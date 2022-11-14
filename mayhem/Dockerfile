FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /resize
WORKDIR /resize/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /resize/fuzz/target/x86_64-unknown-linux-gnu/release/resize-fuzz /