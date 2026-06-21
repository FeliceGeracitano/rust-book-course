# syntax=docker/dockerfile:1
# rust-analyzer LSP bridge: needs the Rust toolchain (RA shells out to cargo) +
# Node (the WebSocket bridge).
FROM rust:1.95-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl ca-certificates \
 && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
 && apt-get install -y --no-install-recommends nodejs \
 && rm -rf /var/lib/apt/lists/*

RUN rustup component add rust-analyzer

WORKDIR /lsp
COPY lsp/package.json lsp/package-lock.json ./
RUN npm ci
COPY lsp/ ./

ENV LSP_PORT=3030 \
    CHAPTERS_DIR=/app/chapters \
    CARGO_TARGET_DIR=/ra-target
EXPOSE 3030
CMD ["node", "server.mjs"]
