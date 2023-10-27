# Builder Stage
FROM rust:1.73-slim-buster as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y build-essential libpq-dev
COPY . .
RUN cargo build --release

# Runner Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/server /server
RUN apt-get update && apt-get install -y libpq5
CMD ["/server"]
