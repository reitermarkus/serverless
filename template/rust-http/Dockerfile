FROM openfaas/of-watchdog:0.5.3 as watchdog

FROM rust:1.36-slim-stretch as builder

WORKDIR /home/app

RUN mkdir -p function/src
RUN echo 'pub fn handle(_: http::Method, _: http::Uri, _: http::HeaderMap, _: String) -> impl futures::future::Future<Item = (http::StatusCode, String), Error = http::StatusCode> { futures::future::ok((http::StatusCode::OK, "".to_string())) }' > function/src/lib.rs

COPY openfaas openfaas
COPY main main

COPY function/Cargo.toml function/Cargo.toml

# Fetch and precompile dependencies.
RUN cargo fetch --manifest-path main/Cargo.toml
RUN cargo build --release --manifest-path main/Cargo.toml

COPY . .

RUN cargo build --release --manifest-path main/Cargo.toml

FROM debian:stretch-slim as ship

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

ARG ADDITIONAL_PACKAGE
RUN apt-get update -qy \
    && apt-get install -qy --no-install-recommends ${ADDITIONAL_PACKAGE} \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN addgroup --system app \
    && adduser --system --ingroup app app

RUN chown app:app -R /home/app

USER app

WORKDIR /home/app
COPY --from=builder /home/app/main/target/release/main /home/app/

ENV function_process=./main
EXPOSE 8080

HEALTHCHECK --interval=2s CMD [ -e /tmp/.lock ] || exit 1

ENV upstream_url="http://127.0.0.1:7878"
ENV mode="http"

CMD ["fwatchdog"]
