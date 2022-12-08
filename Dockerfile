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
RUN cargo install refinery_cli

FROM tobiaszimmer/exam-gateway-subscription:rust-grpc
RUN apt-get update && apt install libssl1.1
COPY --from=builder /usr/local/cargo/bin/auth_service_exam_2022 /usr/local/bin/application
COPY start.sh /start.sh
COPY --from=builder /build/keys /keys
COPY --from=builder /build/migrations /migrations
COPY --from=builder /usr/local/cargo/bin/refinery /usr/local/bin/refinery
COPY gateway-routes.json /gateway-routes.json
ENV ROCKET_ADDRESS=0.0.0.0
ENV KEY_DIR=/keys
ENV HAS_GATEWAY true
CMD ["sh","/start.sh"]
