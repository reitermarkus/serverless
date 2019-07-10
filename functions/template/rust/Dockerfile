FROM rust:1.36-slim-stretch as builder

WORKDIR /home/app

COPY . /home/app/

RUN cargo build --release --manifest-path /home/app/main/Cargo.toml

FROM debian:stretch-slim as ship

ARG WATCHDOG_VERSION=0.5.3

RUN apt-get update -qy \
    && apt-get install -qy curl ca-certificates --no-install-recommends \
    && echo "Pulling watchdog binary from Github." \
    && curl -sSL https://github.com/openfaas-incubator/of-watchdog/releases/download/$WATCHDOG_VERSION/of-watchdog -o /usr/bin/fwatchdog \
    && chmod +x /usr/bin/fwatchdog \
    && apt-get -qy remove curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /home/app
COPY --from=builder /home/app/main/target/release/main /home/app/

ENV function_process=./main

HEALTHCHECK --interval=2s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
