FROM europe-west1-docker.pkg.dev/broxus-infrastructure/docker/rust-builder:v1.59 AS builder

WORKDIR /build

RUN rustup component add rustfmt
# Build dependencies only, when source code changes,
# this build can be cached, we don't need to compile dependency again.
RUN mkdir src && touch src/lib.rs
COPY Cargo.toml Cargo.lock ./
RUN RUSTFLAGS=-g cargo build --release

# Build App
COPY . .
RUN RUSTFLAGS=-g cargo build --release


FROM europe-west1-docker.pkg.dev/broxus-infrastructure/docker/rust-runtime:v1.59
COPY --from=builder /build/target/release/dao-api /app/application
COPY --from=builder /build/entrypoint.sh /app/entrypoint.sh
COPY --from=builder /build/migrations /app/migrations
COPY --from=builder /build/sqlx-data.json /app/sqlx-data.json
USER runuser
EXPOSE 9000
ENTRYPOINT ["/app/entrypoint.sh"]
