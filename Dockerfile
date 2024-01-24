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

COPY cronjob /etc/cron.d/hello-cron

RUN chmod 0644 /etc/cron.d/hello-cron

# Apply cron job
RUN crontab /etc/cron.d/hello-cron

RUN touch /var/log/cron.log

COPY --from=builder /palworld/target/release/palworld /usr/local/bin/palworld

ENV RCON_ADDRESS=test
ENV RCON_PASS=test


CMD ["cron","-f", "-L", "2"]