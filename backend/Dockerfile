FROM rust:1 as builder
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
COPY . .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000


CMD ["cargo", "run"]
