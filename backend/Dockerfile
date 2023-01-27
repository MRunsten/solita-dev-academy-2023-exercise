FROM rust:1.67 as builder

WORKDIR /usr/src/backend
COPY backend/ .

# Use SQLX offline mode to build without a database connection.
ARG SQLX_OFFLINE=true

RUN cargo install --locked --path .

FROM debian:buster-slim
RUN apt-get update &&  \
    apt-get install -y libssl-dev &&  \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
ENV CITY_BICYCLES_IN_PRODUCTION=true
CMD ["backend"]
