FROM rust:alpine AS builder

WORKDIR /build
COPY . .

RUN apk add musl-dev
RUN cargo build --release

FROM alpine

COPY --from=builder /build/target/release/rust-axum /usr/bin/rust-axum

EXPOSE 8080

CMD [ "/usr/bin/rust-axum" ]