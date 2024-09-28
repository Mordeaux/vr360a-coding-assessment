FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bookworm-slim as runner
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/data_collector /usr/local/bin/data_collector
CMD ["data_collector"]
