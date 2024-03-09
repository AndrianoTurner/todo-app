FROM rust:alpine AS builder
ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add "$TARGET"

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release --locked

#Run
FROM alpine:latest
RUN apk add --no-cache libgcc
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/todo-app .
EXPOSE 8080
RUN chmod +x ./todo-app
ENTRYPOINT ["./todo-app"]
