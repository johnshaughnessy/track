# Use the base builder image
FROM johnshaughnessy/track/track-base-builder as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runner Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/server /server
RUN apt-get update && apt-get install -y libpq5
CMD ["/server"]
