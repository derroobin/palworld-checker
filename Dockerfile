FROM rust:buster as builder
WORKDIR /palworld/
COPY . .
RUN cargo build --release


FROM debian:buster-slim
WORKDIR /palworld/
#
COPY entrypoint.sh  entrypoint.sh
RUN chmod +x entrypoint.sh

RUN apt-get update  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /palworld/target/release/palworld /usr/local/bin/palworld

ENV RCON_ADDRESS=test
ENV RCON_PASS=test


CMD ["./entrypoint.sh"]