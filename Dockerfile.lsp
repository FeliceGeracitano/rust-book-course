# syntax=docker/dockerfile:1
# rust-analyzer LSP bridge: needs the Rust toolchain (RA shells out to cargo) +
# Node (the WebSocket bridge).
FROM rust:1.95-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl ca-certificates \
 && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && rm -rf /var/lib/apt/lists/*

# rust-analyzer needs rust-src to analyze the standard library (std-aware
# completion, hover, and diagnostics).
RUN rustup component add rust-analyzer rust-src
# rust-analyzer watches for a global config and warns when it's absent — create
# an empty one to silence the noise.
RUN mkdir -p /root/.config/rust-analyzer && touch /root/.config/rust-analyzer/rust-analyzer.toml

WORKDIR /lsp
COPY lsp/package.json lsp/package-lock.json ./
RUN npm ci
COPY lsp/ ./

ENV LSP_PORT=3030 \
    CHAPTERS_DIR=/app/chapters \
    CARGO_TARGET_DIR=/ra-target
EXPOSE 3030
CMD ["node", "server.mjs"]
