# Build
FROM rust:bookworm AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release && mv ./target/release/to-gaze-bot ./to-gaze-bot

# Runtime image
FROM debian:bookworm 

RUN apt-get update && apt install -y openssl ca-certificates

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/to-gaze-bot /app/to-gaze-bot

CMD ./to-gaze-bot /app/
