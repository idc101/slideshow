FROM rust:1.84 as builder
WORKDIR /app
RUN apt-get update && apt-get install -y libgexiv2-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
COPY . .
RUN cargo xtask install

FROM debian:bookworm-slim AS runtime
WORKDIR /opt/app
RUN apt-get update && apt-get install -y libgexiv2-2
COPY --from=builder /app/output /opt/app
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENTRYPOINT ["/opt/app/backend"]
