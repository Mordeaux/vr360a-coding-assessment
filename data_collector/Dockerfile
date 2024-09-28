FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bookworm-slim as runner
COPY --from=builder /usr/local/cargo/bin/data_collector /usr/local/bin/data_collector
CMD ["data_collector"]
