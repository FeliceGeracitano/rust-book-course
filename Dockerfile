# syntax=docker/dockerfile:1

# 1) Build the React client
FROM node:22-slim AS client
WORKDIR /app/client
COPY client/package.json client/package-lock.json ./
RUN npm ci
COPY client/ ./
RUN npm run build

# 2) Build the Rust server
FROM rust:1.95-slim AS server
WORKDIR /app/server
COPY server/Cargo.toml ./
COPY server/src ./src
RUN cargo build --release

# 3) Runtime — keeps the Rust toolchain so the Check endpoint can run
#    `cargo test` against the mounted chapter crates.
FROM rust:1.95-slim
WORKDIR /app
COPY --from=server /app/server/target/release/server /usr/local/bin/server
COPY --from=client /app/client/dist /app/client/dist
COPY chapters /app/chapters
COPY content /app/content
ENV CLIENT_DIR=/app/client/dist \
    CONTENT_DIR=/app/content \
    CHAPTERS_DIR=/app/chapters \
    CARGO_TARGET_DIR=/cargo-target \
    PORT=8080
EXPOSE 8080
CMD ["server"]
