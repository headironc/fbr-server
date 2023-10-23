FROM rust:1.73.0 as builder

WORKDIR /usr/src/fbr-server
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/fbr-server /usr/local/bin/fbr-server
CMD ["fbr-server"]
