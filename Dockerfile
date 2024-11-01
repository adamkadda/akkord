# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.78.0
ARG APP_NAME=akkord

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

COPY . .

RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

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

COPY --from=build /bin/server /bin/
COPY static /static
COPY templates /templates

EXPOSE 8080

CMD ["/bin/server"]
