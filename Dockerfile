FROM clux/muslrust:stable AS chef
ARG MASTER_BACKEND_PORT
RUN cargo install cargo-chef
WORKDIR /master-backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /master-backend/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /master-backend/target/x86_64-unknown-linux-musl/release/helios-master-backend /master-backend
ENTRYPOINT ["/master-backend"]
EXPOSE $MASTER_BACKEND_PORT

LABEL org.opencontainers.image.source https://github.com/HeliosShieldProject/master-backend-rust