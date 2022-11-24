FROM rust:1.65.0-buster as builder
WORKDIR /build
COPY . .
RUN rustup default nightly
RUN apt update
RUN apt install \
    protobuf-compiler \
    libprotobuf-dev \
    -y
RUN cargo install --path .
RUN sh genkeys.sh

FROM debian:buster-slim
RUN apt-get update && apt install libssl1.1
COPY --from=builder /usr/local/cargo/bin/si_assignment_03_order_service /usr/local/bin/application
COPY --from=builder /build/keys /keys
ENV ROCKET_ADDRESS=0.0.0.0
ENV KEY_DIR=/keys
EXPOSE 8000
CMD ["application"]
