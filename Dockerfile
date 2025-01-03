ARG RUST_VERSION=1.78.0
ARG APP_NAME=akkord

FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG APP_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin ${APP_NAME}

FROM alpine:3.18 AS final
ARG APP_NAME

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${APP_NAME} /usr/local/bin/${APP_NAME}
COPY ./templates ./templates
COPY ./static ./static
COPY ./favicon.ico ./favicon.ico

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/akkord"]