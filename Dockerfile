FROM rust:buster as builder
WORKDIR /palworld/
COPY . .
RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update \
    && apt-get -y --no-install-recommends install -y cron \
    # Remove package lists for smaller image sizes
    && rm -rf /var/lib/apt/lists/* \
    && which cron \
    && rm -rf /etc/cron.*/

COPY cronjob /etc/crontab

RUN chmod 0644 /etc/crontab

# Apply cron job
RUN crontab /etc/crontab


COPY --from=builder /palworld/target/release/palworld /usr/local/bin/palworld

ENV RCON_ADDRESS=test
ENV RCON_PASS=test

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

CMD ["cron","-f", "-L", "2"]