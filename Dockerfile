# Two-stage build: compile with the full Rust toolchain, ship a slim runtime.
# The runtime image carries only the binary and the ca-certificates needed
# for outbound HTTPS (aa.org scraping, Grapevine, the AI gateway).

FROM rust:1.82-bookworm AS builder
WORKDIR /build

# Cache deps: copy manifests, build a stub, then drop the real sources in.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release && rm -rf src target/release/iwiywi target/release/deps/iwiywi*

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates tini \
    && rm -rf /var/lib/apt/lists/*

# Run as a non-root uid; volume-mount ~/.iwiywi into /data at runtime so
# fetched readings, favorites, and .env all survive container restarts.
RUN useradd --uid 10001 --user-group --home /data iwiywi \
    && mkdir -p /data && chown iwiywi:iwiywi /data
USER iwiywi
ENV HOME=/data
WORKDIR /data

COPY --from=builder /build/target/release/iwiywi /usr/local/bin/iwiywi

EXPOSE 8080
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/iwiywi"]
CMD ["serve", "--bind", "0.0.0.0", "--port", "8080"]
