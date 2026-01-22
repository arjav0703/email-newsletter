FROM rust:latest

RUN apt update && apt install lld clang -y

WORKDIR /app
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/email-newsletter"]
