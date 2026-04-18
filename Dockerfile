FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

COPY main.rs .

RUN rustc -O main.rs -o hello_world

FROM debian:bookworm-slim

COPY --from=builder /app/hello_world /usr/local/bin/hello_world

CMD ["hello_world"]
